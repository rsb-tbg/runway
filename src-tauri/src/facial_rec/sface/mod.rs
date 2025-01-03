extern crate opencv as cv;
use cv::objdetect::FaceRecognizerSF;
use cv::prelude::{FaceRecognizerSFTrait, FaceRecognizerSFTraitConst};
use rayon::prelude::ParallelIterator;
use rayon::slice::ParallelSliceMut;

use crate::error::RunwayError;
use crate::facial_rec::mat_to_vec;

use super::ProcessedImage;

pub fn create_recognizer() -> Result<cv::core::Ptr<FaceRecognizerSF>, RunwayError> {
    Ok(FaceRecognizerSF::create(
        "/Users/rsb-tbg/Projects/runway/src/static/models/face_recognition_sface_2021dec_int8.onnx",
        "", // not needed for onnx file
        0,
        0,
    )?)
}

pub async fn generate_embeddings(
    processed_images: &mut Vec<ProcessedImage>,
) -> Result<(), RunwayError> {
    let avail_threads: usize = std::thread::available_parallelism()?.into();
    let mut images_per_thread: usize = processed_images.len() / avail_threads;
    if images_per_thread < 1 {
        images_per_thread = processed_images.len()
    }
    processed_images
        .par_chunks_mut(images_per_thread)
        .for_each(|chunk_of_images| {
            let mut sface = create_recognizer().unwrap();
            for image in chunk_of_images {
                for face in &mut image.faces_for_db {
                    let face_mat =
                        cv::core::Mat::from_slice(&face.face_coords[..]).unwrap_or_else(|err| {
                            println!("{}", err);
                            panic!();
                        });
                    let mut aligned_mat = cv::core::Mat::default();
                    let mut features_mat = cv::core::Mat::default();
                    sface
                        .align_crop(&image.image_mat, &face_mat, &mut aligned_mat)
                        .unwrap();
                    sface.feature(&aligned_mat, &mut features_mat).unwrap();
                    let mut normalized_face = cv::core::Mat::default();
                    cv::core::normalize(
                        &features_mat,
                        &mut normalized_face,
                        1.0,
                        0.0,
                        cv::core::NORM_L2,
                        -1,
                        &cv::core::no_array(),
                    )
                    .unwrap();
                    face.embedding = mat_to_vec(&normalized_face).unwrap();
                    // image.faces_for_db.push(Face { image_path: image.image_path.clone(), person_name: String::from(""), encoded_cropped_face: Vec::<u8>::new(), embedding: mat_to_vec(&normalized_face).unwrap()});
                }
            }
        });
    Ok(())
}
