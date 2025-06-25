#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, Window,
    webview::WebviewBuilder, WebviewUrl, WindowEvent,
};
use serde_json::{Map, Value};
use std::{
    collections::HashMap,
    path::PathBuf,  // Add this import
    thread,
};

const OUTER_WINDOW: &str = "custom_main";
const PANE_LABELS: [&str; 4] = ["main1", "main2", "main3", "main4"];

use std::sync::Mutex;

struct AppState {
    sidebar_width: f64,
    is_fullscreen: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sidebar_width: 64.0,
            is_fullscreen: false,
        }
    }
}

static APP_STATE: once_cell::sync::Lazy<Mutex<AppState>> = once_cell::sync::Lazy::new(|| {
    Mutex::new(AppState::default())
});
fn get_workspaces_path() -> Result<PathBuf, String> {
    // Always use current directory (where the exe is, or project root in dev)
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let json_path = current_dir.join("workspaces.json");
    
    // Create default file if it doesn't exist
    if !json_path.exists() {
        let default_workspaces = serde_json::json!({
            "Math Layout": {
                "main1": { "url": "https://app.haldor.se/" },
                "main2": { "url": "https://nokportalen.se/" },
                "main3": { "url": "https://www.onenote.com/?public=1&wdorigin=ondcauth2&wdorigin=ondc" },
                "main4": { "url": "https://www.microsoft.com/sv-se/microsoft-teams/log-in?market=se" }
            },
            "Programming Layout": {
                "main1": { "url": "https://app.haldor.se/" },
                "main2": { "url": "https://github.com/" },
                "main3": { "url": "https://chatgpt.com/" },
                "main4": { "url": "https://www.w3schools.com/" }
            },
            "Swedish Layout": {
                "main1": { "url": "https://word.cloud.microsoft/en-us/" },
                "main2": { "url": "https://app.haldor.se/" },
                "main3": { "url": "https://teams.microsoft.com/v2/" },
                "main4": { "url": "https://svenska.se/" }
            }
        });
        
        let json_string = serde_json::to_string_pretty(&default_workspaces)
            .map_err(|e| format!("serialize default workspaces: {}", e))?;
        std::fs::write(&json_path, json_string)
            .map_err(|e| format!("write default workspaces: {}", e))?;
        
        println!("Created default workspaces.json at: {:?}", json_path);
    }
    
    Ok(json_path)
}

#[tauri::command]
async fn get_all_workspaces() -> Result<Value, String> {
    let json_path = get_workspaces_path()?;
    
    let json = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("read {:?}: {}", json_path, e))?;
    let all: Value = serde_json::from_str(&json)
        .map_err(|e| format!("parse workspaces.json: {}", e))?;
    Ok(all)
}

#[tauri::command]
async fn save_workspace(name: String, config: Value) -> Result<(), String> {
    let json_path = get_workspaces_path()?;
    
    // Read existing workspaces
    let json = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("read {:?}: {}", json_path, e))?;
    let mut all: Map<String, Value> = serde_json::from_str(&json)
        .map_err(|e| format!("parse workspaces.json: {}", e))?;
    
    // Update or add the workspace
    all.insert(name, config);
    
    // Write back to file
    let json_string = serde_json::to_string_pretty(&all)
        .map_err(|e| format!("serialize workspaces: {}", e))?;
    std::fs::write(&json_path, json_string)
        .map_err(|e| format!("write {:?}: {}", json_path, e))?;
    
    Ok(())
}

#[tauri::command]
async fn delete_workspace(name: String) -> Result<(), String> {
    let json_path = get_workspaces_path()?;
    
    // Read existing workspaces
    let json = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("read {:?}: {}", json_path, e))?;
    let mut all: Map<String, Value> = serde_json::from_str(&json)
        .map_err(|e| format!("parse workspaces.json: {}", e))?;
    
    // Remove the workspace
    all.remove(&name);
    
    // Write back to file
    let json_string = serde_json::to_string_pretty(&all)
        .map_err(|e| format!("serialize workspaces: {}", e))?;
    std::fs::write(&json_path, json_string)
        .map_err(|e| format!("write {:?}: {}", json_path, e))?;
    
    Ok(())
}

fn read_workspace(layout: &str) -> Result<Value, String> {
    let json_path = get_workspaces_path()?;
    let json = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("read {:?}: {}", json_path, e))?;
    let all: Value = serde_json::from_str(&json)
        .map_err(|e| format!("parse workspaces.json: {}", e))?;
    all.get(layout)
        .cloned()
        .ok_or_else(|| format!("Workspace '{}' not found", layout))
}


#[tauri::command]
async fn toggle_fullscreen(app: AppHandle, fullscreen: bool) -> Result<(), String> {
    println!("Setting fullscreen value to, {}", fullscreen);
    
    // Update the fullscreen state
    {
        let mut state = APP_STATE.lock().unwrap();
        state.is_fullscreen = fullscreen;
    }
    
    if let Some(window) = app.get_window(OUTER_WINDOW) {
        if fullscreen {
            // FULLSCREEN MODE: Hide all quadrant panes by setting their size to 0
            for &label in &PANE_LABELS {
                if let Some(wv) = window.get_webview(label) {
                    // "Hide" by setting to size 0x0 and moving off-screen
                    let _ = wv.set_size(LogicalSize::new(0.0, 0.0));
                    let _ = wv.set_position(LogicalPosition::new(-1000.0, -1000.0));
                }
            }
            
            // Then maximize the svelte_app webview
            if let Some(svelte_webview) = window.get_webview("svelte_app") {
                if let Ok(size) = window.inner_size() {
                    let width = size.width as f64;
                    let height = size.height as f64;
                    let _ = svelte_webview.set_position(LogicalPosition::new(0.0, 0.0));
                    let _ = svelte_webview.set_size(LogicalSize::new(width, height));
                }
            }
        } else {
            // NON-FULLSCREEN MODE: Restore and reposition all quadrant panes
            if let Ok(size) = window.inner_size() {
                let width = size.width as f64;
                let height = size.height as f64;
                
                // Reset svelte_app to sidebar width
                if let Some(svelte_webview) = window.get_webview("svelte_app") {
                    let sidebar_width = APP_STATE.lock().unwrap().sidebar_width;
                    let _ = svelte_webview.set_position(LogicalPosition::new(0.0, 0.0));
                    let _ = svelte_webview.set_size(LogicalSize::new(sidebar_width, height));
                }
                
                // Apply normal layout to quadrant panes
                apply_quadrant_layout(&window, width, height);
            }
        }
    }
    
    // Add a small delay to ensure UI is updated
    std::thread::sleep(std::time::Duration::from_millis(50));
    
    Ok(())
}
#[tauri::command]
async fn update_sidebar_width(app: AppHandle, width: f64) -> Result<(), String> {
    let is_fullscreen;
    {
        println!("Updating width: {}", width);
        let mut state = APP_STATE.lock().unwrap();
        state.sidebar_width = width;
        is_fullscreen = state.is_fullscreen;  // Get current fullscreen state
    }
    
    // Only update layout if NOT in fullscreen mode
    if !is_fullscreen {
        if let Some(window) = app.get_window(OUTER_WINDOW) {
            if let Ok(sz) = window.inner_size() {
                let width = sz.width as f64;
                let height = sz.height as f64;
                
                // Update svelte_app width
                if let Some(svelte_webview) = window.get_webview("svelte_app") {
                    let _ = svelte_webview.set_size(LogicalSize::new(width, height));
                }
                
                apply_quadrant_layout(&window, width, height);
            }
        }
    }
    
    Ok(())
}
fn apply_quadrant_layout(window: &Window, window_width: f64, window_height: f64) {
    let sidebar_width = APP_STATE.lock().unwrap().sidebar_width;
    
    // Available space after sidebar
    let content_width = window_width - sidebar_width;
    let half_content_width = content_width / 2.0;
    let half_height = window_height / 2.0;
    
    // Positions for the four quadrants - only cover the content area, not the sidebar
    let positions = [
        (sidebar_width,                      0.0),          // Top-left (starts after sidebar)
        (sidebar_width + half_content_width, 0.0),          // Top-right
        (sidebar_width,                      half_height),   // Bottom-left (starts after sidebar)
        (sidebar_width + half_content_width, half_height),   // Bottom-right
    ];
    
    // Apply layout to ALL quadrant panes (main1, main2, main3, main4)
    for (idx, &label) in PANE_LABELS.iter().enumerate() {
        if let Some(wv) = window.get_webview(label) {
            let (x, y) = positions[idx];
            let _ = wv.set_position(LogicalPosition::new(x, y));
            let _ = wv.set_size(LogicalSize::new(half_content_width, half_height));
        }
    }
}

#[tauri::command]
async fn load_workspace(app: AppHandle, layout_name: String) -> Result<(), String> {
    let ws_json = read_workspace(&layout_name)?;  
    let pane_cfg: Map<String, Value> = ws_json.as_object().cloned().unwrap_or_default();

    let pane_urls: HashMap<&'static str, Option<String>> = PANE_LABELS
        .iter()
        .map(|&lbl| {
            let url = pane_cfg
                .get(lbl)
                .and_then(|p| p.get("url"))
                .and_then(Value::as_str)
                .map(|s| s.to_owned());
            (lbl, url)
        })
        .collect();

    // Get current fullscreen state
    let is_fullscreen = APP_STATE.lock().unwrap().is_fullscreen;

    thread::spawn(move || {
        let window = match app.get_window(OUTER_WINDOW) {
            Some(w) => w,
            None => {
                eprintln!("ERROR: main window '{OUTER_WINDOW}' not found");
                return;
            }
        };

        // Handle ALL quadrant panes (main1, main2, main3, main4)
        for &label in &PANE_LABELS {
            match window.get_webview(label) {
                Some(wv) => {
                    // If webview exists, navigate to new URL
                    if let Some(Some(ref url)) = pane_urls.get(label) {
                        let _ = wv.navigate(url.parse().unwrap());
                    }
                    
                    // If in fullscreen mode, hide the webview
                    if is_fullscreen {
                        let _ = wv.set_size(LogicalSize::new(0.0, 0.0));
                        let _ = wv.set_position(LogicalPosition::new(-1000.0, -1000.0));
                    }
                }
                None => {
                    // Create new webview for panes that don't exist
                    let url = pane_urls
                        .get(label)
                        .and_then(|v| v.as_ref())
                        .unwrap_or(&"about:blank".into())
                        .parse()
                        .unwrap_or_else(|_| "about:blank".parse().unwrap());

                    let builder = WebviewBuilder::new(label, WebviewUrl::External(url));
                    let position = if is_fullscreen {
                        LogicalPosition::new(-1000.0, -1000.0)
                    } else {
                        LogicalPosition::new(0.0, 0.0)
                    };
                    
                    let size = if is_fullscreen {
                        LogicalSize::new(0.0, 0.0)
                    } else {
                        LogicalSize::new(100.0, 100.0)
                    };
                    
                    let _ = window.add_child(builder, position, size);
                }
            }
        }
        
        // Apply the quadrant layout only if NOT in fullscreen mode
        if !is_fullscreen {
            if let Ok(sz) = window.inner_size() {
                let width = sz.width as f64;
                let height = sz.height as f64;
                apply_quadrant_layout(&window, width, height);
            }
        } else {
            // If in fullscreen mode, ensure svelte_app takes full window
            if let Some(svelte_webview) = window.get_webview("svelte_app") {
                if let Ok(size) = window.inner_size() {
                    let width = size.width as f64;
                    let height = size.height as f64;
                    let _ = svelte_webview.set_position(LogicalPosition::new(0.0, 0.0));
                    let _ = svelte_webview.set_size(LogicalSize::new(width, height));
                }
            }
        }

        // Set up resize handler
        let win_clone = window.clone();
        window.on_window_event(move |event| {
            if let WindowEvent::Resized(size) = event {
                let width = size.width as f64;
                let height = size.height as f64;
                
                // Only apply normal layout if NOT in fullscreen mode
                let is_fullscreen = APP_STATE.lock().unwrap().is_fullscreen;
                if !is_fullscreen {
                    apply_quadrant_layout(&win_clone, width, height);
                } else {
                    // In fullscreen mode, only resize the svelte_app
                    if let Some(svelte_webview) = win_clone.get_webview("svelte_app") {
                        let _ = svelte_webview.set_position(LogicalPosition::new(0.0, 0.0));
                        let _ = svelte_webview.set_size(LogicalSize::new(width, height));
                    }
                }
            }
        });
    });

    Ok(())
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