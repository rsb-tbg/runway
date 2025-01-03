use std::path::PathBuf;
use crate::{error::RunwayError, AppState, facial_rec::{self}, PathType};
use tauri::{State, api::dialog::blocking::FileDialogBuilder};
use tokio::sync::Mutex;
use super::{Param, Thresholds};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_categories(appstate: State<'_, Mutex<AppState>>) -> Result<serde_json::Value, RunwayError> {
  let mut categories = vec![];
  for category in &appstate.lock().await.categories.0 {
    categories.push(category.name.clone())
  }
  let response = serde_json::json!({
    "categories": categories,
  });
  Ok(response)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_category(appstate: State<'_, Mutex<AppState>>, selected_category: &str) -> Result<(),()> {
    let categories = appstate.lock().await.categories.0.clone();
    for category in categories {
        if category.name == selected_category {
            appstate.lock().await.selected_category = category;
        };
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_models(appstate: State<'_, Mutex<AppState>>) -> Result<serde_json::Value, RunwayError> {
  let mut models = vec![];
  // appstate.lock().await.selected_params = Vec::<Param>::new();
  for model in &appstate.lock().await.selected_category.models {
    models.push(model.name.clone())
  }
  let category = appstate.lock().await.selected_category.name.clone();
  let response = serde_json::json!({
    "category": category,
    "models": models,
  });
  Ok(response)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_model(appstate: State<'_, Mutex<AppState>>, selected_model: &str) -> Result<(),()> {
    let models = appstate.lock().await.selected_category.models.clone();
    for model in models {
        if model.name == selected_model {
            appstate.lock().await.selected_model = model;
            appstate.lock().await.selected_params = Vec::new();
        };
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_params(appstate: State<'_, Mutex<AppState>>) -> Result<serde_json::Value, RunwayError> {
  if appstate.lock().await.selected_params.len() == 0 {
    return Ok(serde_json::json!({
      "model_name": appstate.lock().await.selected_model.name,
      "params": appstate.lock().await.selected_model.default_params,
    }));
  } else {
    return Ok(serde_json::json!({
      "model_name": appstate.lock().await.selected_model.name,
      "params": appstate.lock().await.selected_params,
    }));
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_params(appstate: State<'_, Mutex<AppState>>, selected_params: Vec<Param>) -> Result<(),()> {
  appstate.lock().await.selected_params = selected_params;
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn reset_params(appstate: State<'_, Mutex<AppState>>) -> Result<(),()> {
  appstate.lock().await.selected_params = Vec::new();
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_path(appstate: State<'_, Mutex<AppState>>) -> Result<serde_json::Value, RunwayError> {
  if appstate.lock().await.selected_path != None {
    return Ok(serde_json::json!({
      "path": appstate.lock().await.selected_path,
    }))
  } else {
    return Ok(serde_json::json!({
      "path": "",
    }))
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_path_type(appstate: State<'_, Mutex<AppState>>) -> Result<serde_json::Value, RunwayError> {
  return Ok(serde_json::json!({
    "path_type": appstate.lock().await.path_type,
  }))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_directory(appstate: State<'_, Mutex<AppState>>) -> Result<(),()> {
  appstate.lock().await.selected_path = FileDialogBuilder::new().pick_folder();
  appstate.lock().await.path_type = PathType::Directory;
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_file(appstate: State<'_, Mutex<AppState>>) -> Result<(),()> {
  appstate.lock().await.selected_path = FileDialogBuilder::new().pick_file();
  appstate.lock().await.path_type = PathType::File;
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_video_feed(appstate: State<'_, Mutex<AppState>>) -> Result<(),()> {
  appstate.lock().await.selected_path = Some(PathBuf::from("video_feed"));
  appstate.lock().await.path_type = PathType::VideoFeed;
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_result_thresholds(appstate: State<'_, Mutex<AppState>>) -> Result<serde_json::Value, RunwayError> {
    return Ok(serde_json::json!({
      "result_thresholds": appstate.lock().await.selected_result_thresholds,
    }))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_result_thresholds(appstate: State<'_, Mutex<AppState>>, selected_result_thresholds: Thresholds) -> Result<(),()> {
  appstate.lock().await.selected_result_thresholds = selected_result_thresholds;
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn reset_result_thresholds(appstate: State<'_, Mutex<AppState>>) -> Result<(),()> {
  appstate.lock().await.selected_result_thresholds = Thresholds::default();
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn load_only(appstate: State<'_, Mutex<AppState>>, window: tauri::Window) -> Result<(), RunwayError> {
  let selected_category_str = appstate.lock().await.selected_category.name.clone();
  match selected_category_str.as_str() {
      "Facial Recognition" => facial_rec::load_faces(appstate, window).await,
      _ => Err(RunwayError { msg: String::from("Won't happen bc get_results can't be called while selected_category is blank") }),
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_parse_names(appstate: State<'_, Mutex<AppState>>, parse_names: bool) -> Result<(), ()> {
  appstate.lock().await.parse_names = parse_names;
  Ok(())
}