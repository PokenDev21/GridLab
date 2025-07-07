use tauri::{
    webview::WebviewBuilder, AppHandle, LogicalPosition, LogicalSize, Manager, WebviewUrl, Window, WindowEvent
};
use serde_json::{Map, Value};
use std::sync::Mutex;
use std::{
    collections::HashMap,
    thread,
};
use crate::json_handler;

const PANE_LABELS: [&str; 4] = ["main1", "main2", "main3", "main4"];


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


pub fn update_sidebar_width(app: AppHandle, width: f64, outer_window: &str) -> Result<(), String> {
    let is_fullscreen;
    {
        println!("Updating width: {}", width);
        let mut state = APP_STATE.lock().unwrap();
        state.sidebar_width = width;
        is_fullscreen = state.is_fullscreen;  // Get current fullscreen state
    }
    
    // Only update layout if NOT in fullscreen mode
    if !is_fullscreen {
        if let Some(window) = app.get_window(outer_window) {
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
pub fn load_workspace(app: AppHandle, layout_name: String, outer_window: String) -> Result<(), String> {
    let ws_json = json_handler::read_workspace(&layout_name)?;  
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

    let outer_window = outer_window.clone(); 

    thread::spawn(move || {
        let window = match app.get_window(&outer_window) {
            Some(w) => w,
            None => {
                eprintln!("ERROR: main window not found");
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

pub fn toggle_fullscreen(app: AppHandle, fullscreen: bool, outer_window : &str) -> Result<(), String> {
    println!("Setting fullscreen value to, {}", fullscreen);
    // Update the fullscreen state
    {
        let mut state = APP_STATE.lock().unwrap();
        state.is_fullscreen = fullscreen;
    }
    
    if let Some(window) = app.get_window(outer_window) {
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