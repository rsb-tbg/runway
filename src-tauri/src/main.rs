// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use runway::{*, db::*,common::get_file_vec, frontend_content::{sidebar::*, state_control::*}, facial_rec::*};

use tauri::State;
use tokio::sync::Mutex;

fn main() {
  let appstate = Mutex::new(AppState::default());
  let image_for_db_query = Mutex::new(MFQ::default());
  tauri::Builder::default()
    .manage(appstate)
    .manage(image_for_db_query)
    .invoke_handler(tauri::generate_handler![
      reset_app,
      set_current_view,
      next_view,
      prev_view,
      get_sidebar_items,
      get_db_count,
      get_categories,
      select_category,
      get_models,
      select_model,
      get_params,
      set_params,
      reset_params,
      get_path,
      get_path_type,
      select_directory,
      select_file,
      set_video_feed,
      get_video_feed,
      get_file_vec,
      get_result_thresholds,
      set_result_thresholds,
      reset_result_thresholds,
      set_parse_names,
      load_only,
      get_face_count,
      get_detected_faces,
      select_face,
      compare_face,
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn reset_app(appstate: State<'_, Mutex<AppState>>, image_for_db_query: State<'_, Mutex<MFQ>>, window: tauri::Window) -> Result<(),()> {
  *appstate.lock().await = AppState::default();
  *image_for_db_query.lock().await = MFQ::default();
  let _ = get_sidebar_items(appstate, window).await;
  Ok(())
}

#[tauri::command]
async fn set_current_view(appstate: State<'_, Mutex<AppState>>, window: tauri::Window, view: &str) -> Result<(),()> {
  let _ = window.emit("CURRENT_VIEW", view);
  appstate.lock().await.current_view = match view {
    "SelectCategory" => View::SelectCategory,
    "SelectModel" => View::SelectModel,
    "SetParameters" => View::SetParameters,
    "SelectPath" => View::SelectPath,
    "ConfirmSelections" => View::ConfirmSelections,
    "Results" => View::Results,
    _ => View::SelectCategory,
  };
  let _ = get_sidebar_items(appstate, window).await;
  Ok(())
}

#[tauri::command]
async fn next_view(appstate: State<'_, Mutex<AppState>>, window: tauri::Window) -> Result<(),()> {
  let path_type = appstate.lock().await.path_type.clone();
  appstate.lock().await.current_view.next(path_type);
  let _ = window.emit("CURRENT_VIEW", appstate.lock().await.current_view);
  let _ = get_sidebar_items(appstate, window).await;
  Ok(())
}

#[tauri::command]
async fn prev_view(appstate: State<'_, Mutex<AppState>>, window: tauri::Window) -> Result<(),()> {
  let path_type = appstate.lock().await.path_type.clone();
  appstate.lock().await.current_view.prev(path_type);
  let _ = window.emit("CURRENT_VIEW", appstate.lock().await.current_view);
  let _ = get_sidebar_items(appstate, window).await;
  Ok(())
}