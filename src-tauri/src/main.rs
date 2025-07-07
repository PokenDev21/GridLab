/*Todo List for rust backend

Popups for "are you sure" should not be handeld by frontend (edge) it should be a modal popup by the application(windows tauri)
Dont accept all pages being about:blank, creates a broken template
*/

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod json_handler;
mod window_manager;
use tauri::{
    webview::WebviewBuilder, AppHandle, LogicalPosition, LogicalSize, WebviewUrl
};
use serde_json::{Value};

const OUTER_WINDOW: &str = "custom_main";

/// COMMANDS FRONTEND ///
#[tauri::command]
async fn delete_workspace(name: String) -> Result<(), String> {
    json_handler::delete_workspace_in_storage(name)
}
#[tauri::command]
async fn get_all_workspaces() -> Result<Value, String> {
    json_handler::get_all_workspaces_in_storage()
}
#[tauri::command]
async fn save_workspace(name: String, config: Value) -> Result<(), String> {
    json_handler::save_workspace_in_storage(name, config)
}

#[tauri::command]
async fn load_workspace(app: AppHandle, layout_name: String) -> Result<(), String> {
    window_manager::load_workspace(app, layout_name, OUTER_WINDOW.to_string())
}
#[tauri::command]
async fn update_sidebar_width(app: AppHandle, width: f64) -> Result<(), String> {
    window_manager::update_sidebar_width(app, width, OUTER_WINDOW)
}
#[tauri::command]
async fn toggle_fullscreen(app: AppHandle, fullscreen: bool) -> Result<(), String> {
    window_manager::toggle_fullscreen(app, fullscreen, OUTER_WINDOW)
}

fn main() {
    println!("Starting Tauri application...");
    tauri::Builder::default()
        .setup(|app| {
            let (w, h) = (1200.0, 800.0);
            let main_window = tauri::window::WindowBuilder::new(app, OUTER_WINDOW)
                .title("Workspaces App")
                .inner_size(w, h)
                .build()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            // Use a different label for the Svelte app (not "main1")
            let svelte_builder = WebviewBuilder::new("svelte_app", WebviewUrl::App(Default::default()))
                .auto_resize()
                .initialization_script(r#"
                    window.open = (url) => { window.location.href = url; };
                    document.addEventListener('click', e => {
                        const a = e.target.closest('a[target="_blank"]');
                        if (a) {
                            e.preventDefault();
                            window.location.href = a.href;
                        }
                    }, true);
                "#);

            main_window.add_child(
                svelte_builder,
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(w, h),
            ).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(())
        })
       .invoke_handler(tauri::generate_handler![
            load_workspace,
            update_sidebar_width,
            toggle_fullscreen,
            get_all_workspaces,
            save_workspace,
            delete_workspace
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}