use facial_rec::ProcessedImage;
use frontend_content::{sidebar::Sidebar, Categories, Category, Model, Param, Thresholds};
use serde::{Serialize, Deserialize};

use std::path::PathBuf;

pub mod db;
pub mod error;
pub mod facial_rec;
pub mod frontend_content;
pub mod common;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppState {
    pub current_view: View,
    pub sidebar: Sidebar,
    pub categories: Categories,
    pub selected_category: Category,
    pub selected_model: Model,
    pub selected_params: Vec<Param>,
    pub selected_path: Option<PathBuf>,
    pub path_type: PathType,
    pub file_vec: Vec<PathBuf>,
    pub selected_result_thresholds: Thresholds,
    pub parse_names: bool,
    pub db_count: u64,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            current_view: View::VIEWS[0],
            sidebar: Sidebar::new(),
            categories: Categories::new(),
            selected_category: Category { name: String::new(), db_type:String::new(), models: Vec::new() },
            selected_model: Model { name: String::new(), param_ct: 0, default_params: Vec::new(), model_path: PathBuf::new() },
            selected_params: Vec::new(),
            selected_path: None,
            path_type: PathType::None,
            file_vec: Vec::new(),
            selected_result_thresholds: Thresholds::default(),
            parse_names: false,
            db_count: 0,
        }
    }
}

pub struct MFQ {
    pub processed_image: Option::<ProcessedImage>,
    pub total_face_ct: Option<i32>,
    pub selected_face: Option::<String>,
}

impl Default for MFQ {
    fn default() -> Self {
        MFQ {
            processed_image: Option::<ProcessedImage>::None,
            total_face_ct: Option::<i32>::None,
            selected_face: Option::<String>::None,
          }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum View {
    SelectCategory,
    SelectModel,
    SetParameters,
    SelectPath,
    SetResultThresholds,
    ConfirmSelections,
    Results
}

impl View {
    pub const VIEWS: [Self; 7] = [
        Self::SelectCategory,
        Self::SelectModel,
        Self::SetParameters,
        Self::SelectPath,
        Self::SetResultThresholds,
        Self::ConfirmSelections,
        Self::Results,
        ];

    pub fn next(&mut self, path_type: PathType) {
        for (i, view) in View::VIEWS.into_iter().enumerate() {
            if path_type == PathType::Directory && *self == View::SelectPath {
                *self = View::ConfirmSelections;
                break
            } else if self == &view {
                *self = View::VIEWS[i + 1];
                break
            }
        }
    }

    pub fn prev(&mut self, path_type: PathType) {
        for (i, view) in View::VIEWS.into_iter().enumerate() {
            if path_type == PathType::Directory && *self == View::ConfirmSelections {
                *self = View::SelectPath;
                break
            } else if self == &view {
                *self = View::VIEWS[i - 1];
                break
            }
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum PathType {
    None,
    Directory,
    File,
    VideoFeed,
}