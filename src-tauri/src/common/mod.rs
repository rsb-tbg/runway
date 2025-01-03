use std::{path::{self, PathBuf}, fs};

use tauri::State;
use tokio::sync::Mutex;

use crate::AppState;

use super::error::RunwayError;

/// get_file_vec takes the path value and returns a vec of PathBufs to one or more files
#[tauri::command(rename_all = "snake_case")]
pub async fn get_file_vec(appstate: State<'_, Mutex<AppState>>) -> Result<(), RunwayError> {
    if appstate.lock().await.selected_path.is_none() {
        return Err(RunwayError { msg: String::from("No path set for get_file_vec") });
    }
    let path = appstate.lock().await.selected_path.clone().unwrap();
    let mut file_vec = Vec::<PathBuf>::new();
    if path.is_file() {
        file_vec.push(path::PathBuf::from(path));
        appstate.lock().await.file_vec = file_vec;
        Ok(())
    } else if path.is_dir() {
        path_to_vec(path, &mut file_vec)?;
        appstate.lock().await.file_vec = file_vec;
        Ok(())
    } else {
        Err(RunwayError{msg: String::from("Invalid path was entered")}) // do i need this? leftover from cli tool
    }
}

/// path_to_vect recursively walks through directory while adding one or more files to the vec of PathBufs
fn path_to_vec(path: PathBuf, file_vec: &mut Vec<PathBuf>) -> Result<(), RunwayError> {
    let read_dir = fs::read_dir(path)?;
    for dir in read_dir {
        match dir {
            Ok(dir) if path::Path::new(&dir.path()).is_file() => file_vec.push(dir.path()),
            Ok(dir) if path::Path::new(&dir.path()).is_dir() => path_to_vec(dir.path(), file_vec)?,
            Ok(_) => return Err(RunwayError{msg: String::from("Nested path does not point to a valid file or directory.")}),
            Err(e) => return Err(RunwayError { msg: format!("Unable to read nested directory - {}.", e) }),
        };
    };
    Ok(())
}