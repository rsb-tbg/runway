use mongodb::{Client, options::ClientOptions, bson::doc};
use tauri::State;
use tokio::{task::JoinHandle, sync::Mutex};
use tokio_stream::StreamExt;

use crate::{error::RunwayError, AppState, facial_rec::Face, frontend_content::Thresholds};

const CONNECTION_STRING: &str = "mongodb://localhost:27017";
const DB_NAME: &str = "face_rec_db";

/// get_face_collection_handle establishes db connection and returns handle to face collection
pub async fn get_face_collection_handle() -> Result<mongodb::Collection<Face>, RunwayError> {
    let client = Client::with_options(ClientOptions::parse(CONNECTION_STRING).await?)?;
    let db = client.database(DB_NAME);
    let faces: mongodb::Collection<Face> = db.collection::<Face>("faces");
    Ok(faces)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_db_count(appstate: State<'_, Mutex<AppState>>, window: tauri::Window) -> Result<(), RunwayError> {
    let db_type = appstate.lock().await.selected_category.db_type.clone();
    let mut db_count: u64 = 0;
    if db_type == String::from("Faces") {
        let db_collection = get_face_collection_handle().await?;
        db_count = db_collection.count_documents(None, None).await?;
    }
    appstate.lock().await.db_count = db_count;
    let payload = serde_json::json!({
        "db_type": db_type,
        "db_count":  db_count,
    });
    let _ = window.emit("DB_COUNT", payload);
    Ok(())
}

/// write_records creates documents for each detected face
// pub async fn write_records(db_collection: &mongodb::Collection<Face>, norm_faces: &Vec<Face>) -> Result<(), RunwayError> {
//     for face in norm_faces.iter() {
//         let face = Face {
//             file_path: face.file_path.to_owned(),
//             person_name: face.person_name.to_owned(),
//             encoded_cropped_face: face.encoded_cropped_face.to_owned(),
//             embedding: face.embedding.clone(),
//         };
//         db_collection.insert_one(face, None).await?;
//     }
//     Ok(())
// }

/// compare_file compares detected faces to all database facial embeddings and prints the matches
pub async fn compare_file(thresholds: &Thresholds, db_collection: &mongodb::Collection<Face>, faces: &Vec<Face>, window: tauri::Window) -> Result<(), RunwayError> {
    let l2_norm_threshold = thresholds.0[0].value;
    let cos_sim_threshold = thresholds.0[1].value;
    for target_face in faces {
        let mut db_collection_cursor = db_collection.find(doc! {}, None).await?;
        while let Some(doc_result) = db_collection_cursor.next().await {
            let cloned_window = window.clone();
            let target_face_clone: Face = target_face.clone();
            let doc: Face = doc_result?;
            let _ : JoinHandle<Result<(), RunwayError>> = tokio::spawn( async move {
                if target_face_clone.person_name != "Unknown".to_string() && target_face_clone.person_name == doc.person_name {
                    let _ = cloned_window.emit("MATCHED_NAME", true);
                }
                let mut euc_dist: f32 = 0.0;
                for i in 0..128 {
                    euc_dist += (doc.embedding[i] - target_face_clone.embedding[i]).powf(2.0);
                }
                if euc_dist.sqrt() <= l2_norm_threshold {
                    let cos_similarity: f32;
                    let mut cos_sum: f32 = 0.0;
                    let mut targ_sum: f32 = 0.0;
                    let mut db_sum: f32 = 0.0;
                    for i in 0..128 {
                        cos_sum += doc.embedding[i] * target_face_clone.embedding[i];
                        targ_sum += target_face_clone.embedding[i].powf(2.0);
                        db_sum += doc.embedding[i].powf(2.0);
                    }
                    cos_similarity = cos_sum / (targ_sum * db_sum);
                    if cos_similarity >= cos_sim_threshold {
                        let payload = serde_json::json!({
                            "file_path": doc.file_path,
                            "person_name": doc.person_name,
                            "euc_dist": euc_dist.sqrt(),
                            "cos_sim": cos_similarity,
                            "e_c_f": doc.encoded_cropped_face,
                          });
                        let _ = cloned_window.emit("MATCHED_FACE", payload);
                    }
                }
                Ok(())
            });
        }
    }
    Ok(())
}