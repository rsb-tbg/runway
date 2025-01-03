use serde::{Serialize, Deserialize};
use tauri::State;
use tokio::sync::Mutex;

use crate::{AppState, View, PathType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SidebarItem {
    pub name: String,
    pub assoc_view: View,
    pub disabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sidebar {
    pub items: Vec<SidebarItem>,
}

impl Sidebar {
    pub fn new() -> Self {
        Sidebar {
            items: vec![
            SidebarItem { name: String::from("ML Category"), assoc_view: View::SelectCategory, disabled: false },
            SidebarItem { name: String::from("Select Model"), assoc_view: View::SelectModel, disabled: true },
            SidebarItem { name: String::from("Set Parameters"), assoc_view: View::SetParameters, disabled: true },
            SidebarItem { name: String::from("Select Data"), assoc_view: View::SelectPath, disabled: true },
            SidebarItem { name: String::from("Result Thresholds"), assoc_view: View::SetResultThresholds, disabled: true },
            SidebarItem { name: String::from("Review Selections"), assoc_view: View::ConfirmSelections, disabled: true },
            SidebarItem { name: String::from("Results"), assoc_view: View::Results, disabled: true },
            ],
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_sidebar_items(appstate: State<'_, Mutex<AppState>>, window: tauri::Window) -> Result<(),()> {
    let disabled_index = appstate.lock().await.current_view.clone() as usize;
    let sidebar_items = appstate.lock().await.sidebar.items.clone();
    for (i, sidebar_item) in sidebar_items.into_iter().enumerate() {
        if appstate.lock().await.path_type == PathType::Directory && sidebar_item.assoc_view == View::SetResultThresholds {
            appstate.lock().await.sidebar.items[i].disabled = true;
        } else if i <= disabled_index {
            appstate.lock().await.sidebar.items[i].disabled = false;
        } else {
            appstate.lock().await.sidebar.items[i].disabled = true;
        }
    };
    let _ = window.emit("SIDEBAR_ITEMS", serde_json::json!({
        "items": appstate.lock().await.sidebar.items,
    }));
    Ok(())
}