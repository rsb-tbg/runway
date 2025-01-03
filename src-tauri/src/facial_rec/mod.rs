pub mod sface;
pub mod yunet;

extern crate serde;
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};

use cv::{
    imgcodecs::IMREAD_COLOR, objdetect::FaceDetectorYNTrait, prelude::MatTraitConst,
    videoio::VideoCaptureTrait,
};
use opencv as cv;
use tauri::State;
use tokio::sync::{mpsc, Mutex};

use crate::{
    db::{compare_file, get_db_count, get_face_collection_handle},
    error::RunwayError,
    AppState, MFQ,
};
use std::{path::PathBuf, sync::Arc};

use yunet::yunet_detect_faces;

use self::yunet::create_detector;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Face {
    pub file_path: PathBuf,
    pub person_name: String,
    pub face_coords: Vec<f32>,
    pub encoded_cropped_face: String,
    pub embedding: Vec<f32>,
}

#[derive(Clone)]
pub struct ProcessedImage {
    pub file_path: PathBuf,
    pub image_mat: cv::core::Mat,
    pub detected_faces: cv::core::Mat,
    pub faces_for_db: Vec<Face>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_face_count(
    appstate: State<'_, Mutex<AppState>>,
    image_for_db_query: State<'_, Mutex<MFQ>>,
) -> Result<serde_json::Value, RunwayError> {
    if image_for_db_query.lock().await.total_face_ct == None {
        // get image matrices
        // this only gets called when a single file was selected but for uniformity it still uses a vector
        // vector lengths will always be 1 here
        let file_vec = appstate.lock().await.file_vec.clone();
        let mut temp_processed_images = proccess_images(file_vec).await?;

        // detect faces and send count to front end
        let model = appstate.lock().await.selected_model.clone();
        let params = appstate.lock().await.selected_params.clone();
        let parse_names = appstate.lock().await.parse_names;
        let total_face_ct = match model.name.as_str() {
            "YuNet" => yunet_detect_faces(
                &mut temp_processed_images,
                model.model_path,
                params,
                parse_names,
            ),
            _ => Err(RunwayError {
                msg: String::from("This shouldn't happen"),
            }),
        };

        let temp_processed_image = temp_processed_images[0].clone();
        image_for_db_query.lock().await.processed_image = Some(temp_processed_image);
        image_for_db_query.lock().await.total_face_ct =
            Some(total_face_ct.as_ref().unwrap().clone());
        // if total_face_ct is 1 then selected_face is filled with it since there are no other options
        if total_face_ct.as_ref().unwrap().clone() == 1 {
            let selected_face = image_for_db_query
                .lock()
                .await
                .processed_image
                .as_ref()
                .unwrap()
                .faces_for_db[0]
                .encoded_cropped_face
                .to_owned();
            image_for_db_query.lock().await.selected_face = Some(selected_face);
        }
    }

    Ok(serde_json::json!({
        "face_count": image_for_db_query.lock().await.total_face_ct.unwrap(),
    }))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_detected_faces(
    image_for_db_query: State<'_, Mutex<MFQ>>,
) -> Result<serde_json::Value, RunwayError> {
    let temp_processed_image = image_for_db_query
        .lock()
        .await
        .processed_image
        .clone()
        .unwrap();
    let mut detected_faces = Vec::<String>::new();
    for face in temp_processed_image.faces_for_db {
        detected_faces.push(face.encoded_cropped_face)
    }
    return Ok(serde_json::json!({
        "detected_faces": detected_faces,
    }));
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_face(
    selected_face: String,
    image_for_db_query: State<'_, Mutex<MFQ>>,
    window: tauri::Window,
) -> Result<(), ()> {
    image_for_db_query.lock().await.selected_face = Some(selected_face);
    image_for_db_query.lock().await.total_face_ct = Some(1);
    let _ = window.emit("CHANGE_QUERY_VIEW", true);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn compare_face(
    appstate: State<'_, Mutex<AppState>>,
    image_for_db_query: State<'_, Mutex<MFQ>>,
    window: tauri::Window,
) -> Result<(), RunwayError> {
    let mut temp_processed_image_vec = vec![image_for_db_query
        .lock()
        .await
        .processed_image
        .clone()
        .unwrap()];

    // measure faces TODO: separate recognition model?
    let _ = sface::generate_embeddings(&mut temp_processed_image_vec).await;

    let temp_processed_image = temp_processed_image_vec[0].clone();
    let mut selected_face: Option<Face> = None;
    for face in temp_processed_image.faces_for_db {
        if &face.encoded_cropped_face
            == image_for_db_query
                .lock()
                .await
                .selected_face
                .as_ref()
                .unwrap()
        {
            selected_face = Some(face);
        }
    }

    let db_collection = get_face_collection_handle().await?;
    let thresholds = appstate.lock().await.selected_result_thresholds.clone();
    if selected_face != None {
        let _ = compare_file(
            &thresholds,
            &db_collection,
            &vec![selected_face.unwrap()],
            window.clone(),
        )
        .await;
    }
    let temp_processed_image = temp_processed_image_vec[0].clone();
    let _ = db_collection
        .insert_many(&temp_processed_image.faces_for_db, None)
        .await;

    *image_for_db_query.lock().await = MFQ::default();
    get_db_count(appstate, window).await
}

pub async fn load_faces(
    appstate: State<'_, Mutex<AppState>>,
    window: tauri::Window,
) -> Result<(), RunwayError> {
    // get image matrices
    let file_vec = appstate.lock().await.file_vec.clone();
    let mut processed_images = proccess_images(file_vec).await?;
    let model = appstate.lock().await.selected_model.clone();
    let params = appstate.lock().await.selected_params.clone();
    let parse_names = appstate.lock().await.parse_names;
    let total_face_ct = match model.name.as_str() {
        "YuNet" => yunet_detect_faces(&mut processed_images, model.model_path, params, parse_names),
        _ => Err(RunwayError {
            msg: String::from("This shouldn't happen"),
        }),
    };
    // measure faces TODO: separate recognition model?
    let _ = sface::generate_embeddings(&mut processed_images).await;

    let processed_images_ct = processed_images.len();

    let db_collection = get_face_collection_handle().await?;
    for file in processed_images {
        let _ = db_collection.insert_many(&file.faces_for_db, None).await;
    }
    let _ = window.emit("PROCESSED_IMAGES_CT", processed_images_ct);
    let _ = window.emit("TOTAL_FACES_DETECTED", total_face_ct?);
    get_db_count(appstate, window).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_video_feed(
    appstate: State<'_, Mutex<AppState>>,
    window: tauri::Window,
) -> Result<(), String> {
    let mut v_f =
        cv::videoio::VideoCapture::from_file("tcp://192.168.11.38:8554", cv::videoio::CAP_ANY)
            .unwrap();
    if !v_f.grab().unwrap() == true {
        let _ = v_f.release();
        return Err(String::from("Hmmm...unable to connect to camera")
            + "\u{00A0}\u{00A0}\u{00A0}"
            + "\u{1F914}");
    }
    let mut frame_mat = cv::core::Mat::default();
    let mut faces_mat = cv::core::Mat::default();
    let mut frame_buf = cv::core::Vector::<u8>::new();
    let stop_frames = Arc::new(std::sync::Mutex::new(false));
    let stop_frames_clone = stop_frames.clone();
    let id = window.listen("STOP_FRAMES", move |_| {
        *stop_frames_clone.lock().unwrap() = true;
    });
    let model_path = appstate.lock().await.selected_model.model_path.clone();
    let params = appstate.lock().await.selected_params.clone();
    let mut yunet_detector = create_detector(model_path, params).unwrap();
    let mut x: i32;
    let mut y: i32;
    let mut width: i32;
    let mut height: i32;
    let face_box_color = cv::core::Scalar::new(0.0, 255.0, 0.0, 0.0);
    while *stop_frames.lock().unwrap() == false {
        let read_result = v_f.read(&mut frame_mat);
        if read_result.is_ok() {
            let _ = yunet_detector.set_input_size(frame_mat.size().unwrap());
            yunet_detector.detect(&frame_mat, &mut faces_mat).unwrap();
            for r in 0..faces_mat.rows() {
                let face_coords = mat_to_vec(&faces_mat.row(r).unwrap()).unwrap();
                x = face_coords[0] as i32;
                y = face_coords[1] as i32;
                width = face_coords[2] as i32;
                height = face_coords[3] as i32;
                if x < 0 {
                    x = 0
                };
                if y < 0 {
                    y = 0
                };
                let face_box = cv::core::Rect::new(x, y, width, height);
                let _ = cv::imgproc::rectangle(&mut frame_mat, face_box, face_box_color, 1, 1, 0);
            }
            let _encode = cv::imgcodecs::imencode(
                ".jpeg",
                &frame_mat,
                &mut frame_buf,
                &cv::core::Vector::<i32>::new(),
            );
            let encoded_frame =
                "data:image/png;base64,".to_owned() + &general_purpose::STANDARD.encode(&frame_buf);
            let _ = window.emit("NEXT_FRAME", encoded_frame);
        }
    }
    let _ = v_f.release();
    window.unlisten(id);
    Ok(())
}

async fn proccess_images(file_vec: Vec<PathBuf>) -> Result<Vec<ProcessedImage>, RunwayError> {
    let (tx, mut rx) = mpsc::channel(250);

    for file in file_vec {
        let tx_new = tx.clone();
        tokio::spawn(async move {
            let image_mat = image_to_mat(&file)?;
            if image_mat.size()? != cv::core::Size::new(0, 0) {
                let _ = tx_new
                    .send(ProcessedImage {
                        file_path: file.clone(),
                        image_mat: image_mat,
                        detected_faces: cv::core::Mat::default(),
                        faces_for_db: Vec::<Face>::new(),
                    })
                    .await;
                Ok(())
            } else {
                Err(RunwayError {
                    msg: format!("{:?} has a mat size of (0, 0)", file),
                })
            }
        });
    }
    drop(tx);

    let mut processed_image_files: Vec<ProcessedImage> = Vec::new();
    while let Some(pf) = rx.recv().await {
        processed_image_files.push(pf)
    }

    Ok(processed_image_files)
}

fn image_to_mat(file: &PathBuf) -> Result<cv::core::Mat, RunwayError> {
    Ok(cv::imgcodecs::imread(
        file.to_str().unwrap_or_default(),
        IMREAD_COLOR,
    )?)
}

pub fn mat_to_vec(mat: &impl cv::core::MatTraitConst) -> Result<Vec<f32>, RunwayError> {
    let mut return_vec = Vec::new();
    for y in 0..mat.cols() {
        return_vec.push(mat.at_2d::<f32>(0, y)?.clone());
    }
    Ok(return_vec)
}

pub fn pathbuf_to_name(path: &PathBuf) -> String {
    let file_stem = path
        .file_stem()
        .unwrap_or(std::ffi::OsStr::new(""))
        .to_str()
        .unwrap_or("");
    if file_stem.contains("_") {
        let split_file_stem: Vec<&str> = file_stem.split("_").collect();
        return String::from(split_file_stem[0].to_owned() + " " + split_file_stem[1]);
    }
    return String::from("Unknown");
}
