pub mod sidebar;
pub mod state_control;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Categories(Vec<Category>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub name: String,
    pub db_type: String,
    pub models: Vec<Model>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub name: String,
    pub param_ct: usize,
    pub default_params: Vec<Param>,
    pub model_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Param {
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub increment: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thresholds(pub Vec<Threshold>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Threshold {
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub increment: String,
    pub include_results: String,
}

impl Categories {
    pub fn new() -> Categories {
        let yunet = Model {
            name: String::from("YuNet"),
            param_ct: 3,
            default_params: vec![
                Param {
                    name: String::from("Score Threshold"),
                    value: 0.85,
                    min: 0.7,
                    max: 1.0,
                    increment: String::from("0.01"),
                },
                Param {
                    name: String::from("NMS Threshold"),
                    value: 0.3,
                    min: 0.0,
                    max: 0.6,
                    increment: String::from("0.01"),
                },
                Param {
                    name: String::from("Top K"),
                    value: 5000.0,
                    min: 4500.0,
                    max: 5500.0,
                    increment: String::from("1.0"),
                },
            ],
            model_path: PathBuf::from(
                "../src/static/models/face_detection_yunet_2023mar_int8.onnx",
            ),
        };
        let facial_rec_category = Category {
            name: String::from("Facial Recognition"),
            db_type: String::from("Faces"),
            models: vec![yunet],
        };
        let other_stuff_category = Category {
            name: String::from("Other Stuff"),
            db_type: String::from("Things"),
            models: vec![],
        };

        Categories(vec![facial_rec_category, other_stuff_category])
    }
}

impl Thresholds {
    pub fn default() -> Thresholds {
        Thresholds(vec![
            Threshold {
                name: String::from("L2 Norm"),
                value: 1.07,
                min: 0.87,
                max: 1.27,
                increment: String::from("0.01"),
                include_results: String::from("<="),
            },
            Threshold {
                name: String::from("Cosine Similarity"),
                value: 0.38,
                min: 0.18,
                max: 0.58,
                increment: String::from("0.01"),
                include_results: String::from(">="),
            },
        ])
    }
}
