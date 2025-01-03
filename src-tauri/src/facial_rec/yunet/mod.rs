extern crate opencv as cv;

use base64::{engine::general_purpose, Engine as _};
use std::path::PathBuf;
use std::sync::Mutex;

use cv::objdetect::FaceDetectorYN;
use cv::prelude::{FaceDetectorYNTrait, MatTraitConst};
use rayon::prelude::*;

use crate::error::RunwayError;
use crate::facial_rec::pathbuf_to_name;
use crate::frontend_content::Param;

use super::{mat_to_vec, Face, ProcessedImage};

// Measurements:
// 0-1: x, y of bbox top left corner
// 2-3: width, height of bbox
// 4-5: x, y of right eye (blue point in the example image)
// 6-7: x, y of left eye (red point in the example image)
// 8-9: x, y of nose tip (green point in the example image)
// 10-11: x, y of right corner of mouth (pink point in the example image)
// 12-13: x, y of left corner of mouth (yellow point in the example image)
// 14: face score

pub fn create_detector(
    model_path: PathBuf,
    params: Vec<Param>,
) -> Result<cv::core::Ptr<FaceDetectorYN>, RunwayError> {
    Ok(FaceDetectorYN::create(
        model_path.to_str().unwrap(),
        "",
        cv::core::Size::new(300, 300),
        params[0].value,
        params[1].value,
        params[2].value as i32,
        0,
        0,
    )?)
}

/// yunet_face_detection accepts a file path, searches file for faces, and
/// collects 15 measurements for 5 facial landmarks per face
pub fn yunet_detect_faces(
    processed_images: &mut Vec<ProcessedImage>,
    model_path: PathBuf,
    params: Vec<Param>,
    parse_names: bool,
) -> Result<i32, RunwayError> {
    let avail_threads: usize = std::thread::available_parallelism()?.into();
    let mut images_per_thread: usize = processed_images.len() / avail_threads;
    if images_per_thread < 1 {
        images_per_thread = processed_images.len()
    }
    let total_face_ct: Mutex<i32> = Mutex::new(0);
    processed_images
        .par_chunks_mut(images_per_thread)
        .for_each(|chunk_of_images| {
            let mut yunet_detector = create_detector(model_path.clone(), params.clone()).unwrap();
            let mut face_ct: i32 = 0;
            for image in chunk_of_images {
                let _ = yunet_detector.set_input_size(image.image_mat.size().unwrap());
                yunet_detector
                    .detect(&image.image_mat, &mut image.detected_faces)
                    .unwrap();
                for r in 0..image.detected_faces.rows() {
                    let face_coords = mat_to_vec(&image.detected_faces.row(r).unwrap()).unwrap();
                    let mut x = face_coords[0] as i32;
                    let mut y = face_coords[1] as i32;
                    let w = face_coords[2] as i32;
                    let h = face_coords[3] as i32;
                    if x < 0 {
                        x = 0
                    };
                    if y < 0 {
                        y = 0
                    };
                    let width_end = if x + w > image.image_mat.size().unwrap().width {
                        image.image_mat.size().unwrap().width
                    } else {
                        x + w
                    };
                    let height_end = if y + h > image.image_mat.size().unwrap().height {
                        image.image_mat.size().unwrap().height
                    } else {
                        y + h
                    };

                    let cropped_image = image
                        .image_mat
                        .roi(cv::core::Rect::new(x, y, width_end - x, height_end - y))
                        .unwrap();
                    let mut cropped_buf = cv::core::Vector::<u8>::new();
                    let _encode = cv::imgcodecs::imencode(
                        ".jpeg",
                        &cropped_image,
                        &mut cropped_buf,
                        &cv::core::Vector::<i32>::new(),
                    );
                    let encoded_cropped_face = "data:image/png;base64,".to_owned()
                        + &general_purpose::STANDARD.encode(&cropped_buf);
                    let person_name: String;
                    if parse_names && image.detected_faces.rows() == 1 {
                        person_name = pathbuf_to_name(&image.file_path);
                    } else {
                        person_name = String::from("Unknown");
                    };
                    image.faces_for_db.push(Face {
                        file_path: image.file_path.to_owned(),
                        person_name: person_name,
                        face_coords: face_coords,
                        encoded_cropped_face: encoded_cropped_face,
                        embedding: Vec::<f32>::new(),
                    });
                    // let _write = cv::imgcodecs::imwrite("/Users/ryanburke/Downloads/runway_saved.jpeg", &cropped_image, &cv::core::Vector::<i32>::new());
                }

                face_ct += image.detected_faces.rows();
            }
            *total_face_ct.lock().unwrap() += face_ct;
        });

    let total_face_ct = *total_face_ct.lock().unwrap();
    Ok(total_face_ct)
}
