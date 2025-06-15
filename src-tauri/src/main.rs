#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, Window,
    webview::WebviewBuilder, WebviewUrl, WindowEvent,
};
use serde_json::{Map, Value};
use std::{
    collections::HashMap,
    fs,
    panic::{catch_unwind, AssertUnwindSafe},
    thread,
};
// ... (const OUTER_WINDOW, PANE_LABELS, read_workspace remain the same) ...
const OUTER_WINDOW: &str = "custom_main";
const PANE_LABELS: [&str; 4] = ["main1", "main2", "main3", "main4"];

fn apply_quadrant_layout(window: &Window, half_w: f64, half_h: f64) {
    let positions = [
        (0.0,      0.0),
        (half_w,   0.0),
        (0.0,    half_h),
        (half_w, half_h),
    ];
    for (idx, &label) in PANE_LABELS.iter().enumerate() {
        if let Some(wv) = window.get_webview(label) {
            let (x, y) = positions[idx];
            let _ = wv.set_position(LogicalPosition::new(x, y));
            let _ = wv.set_size(LogicalSize::new(half_w, half_h));
        }
    }
}


fn read_workspace(layout: &str) -> Result<Value, String> {
    println!("Attempting to read workspaces.json for layout: {}", layout);
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))
        .unwrap_or_else(|e| {
            println!("ERROR: {}", e);
            std::path::PathBuf::new()
        });
    println!("Current directory: {:?}", current_dir);
    let json_path = current_dir.join("workspaces.json"); // Ensure correct path
    println!("Looking for workspaces.json at: {:?}", json_path);

    let json_result = fs::read_to_string(&json_path); // Use json_path
    if let Err(ref e) = json_result {
        println!("ERROR reading workspaces.json: {}", e);
        return Err(format!("read {:?}: {}", json_path, e));
    }
    let json = json_result.unwrap();
    println!("Successfully read workspaces.json: {} bytes", json.len());
    let all_result = serde_json::from_str::<Value>(&json);
    if let Err(ref e) = all_result {
        println!("ERROR parsing workspaces.json: {}", e);
        println!("JSON content: {}", json);
        return Err(format!("parse workspaces.json: {}", e));
    }
    let all = all_result.unwrap();
    if let Some(workspace) = all.get(layout) {
        println!("Found workspace '{}' in config", layout);
        Ok(workspace.clone())
    } else {
        println!("ERROR: Workspace '{}' not found in config", layout);
        println!("Available workspaces: {:?}",
                 all.as_object().map(|o| o.keys().collect::<Vec<_>>()).unwrap_or_default());
        Err(format!("Workspace '{}' not found", layout))
    }
}


fn ensure_pane(
  window: &Window,
  label: &str,
  pos: (f64, f64),
  size: (f64, f64),
  url: Option<&str>,
) {
  let url_str = url.unwrap_or("about:blank"); // Default to about:blank if no URL
  println!("Ensuring pane '{}': URL='{}', Pos=({},{}), Size=({},{})",
           label, url_str, pos.0, pos.1, size.0, size.1);

  let parsed_url_result = url_str.parse::<url::Url>();
  let parsed_url = match parsed_url_result {
    Ok(u) => {
      println!("   Successfully parsed URL for '{}': {}", label, u);
      u
    }
    Err(e) => {
      println!("   ERROR: Bad URL string for '{}': '{}'. Error: {}", label, url_str, e);
      // Fallback to about:blank if parsing fails
      "about:blank".parse::<url::Url>().unwrap()
    }
  };

  if let Some(wv) = window.get_webview(label) {
    println!("   Pane '{}' exists. Updating position, size, and navigating.", label);
    if let Err(e) = wv.set_position(LogicalPosition::new(pos.0, pos.1)) {
      println!("   ERROR set_position for '{}': {}", label, e);
    }
    if let Err(e) = wv.set_size(LogicalSize::new(size.0, size.1)) {
      println!("   ERROR set_size for '{}': {}", label, e);
    }
    if let Err(e) = wv.navigate(parsed_url.clone()) { // Use parsed_url
      println!("   ERROR navigate for '{}': {}", label, e);
    } else {
      println!("   ✅ Successfully navigated '{}' to {}", label, parsed_url);
    }
    // Ensure devtools are enabled if it exists
    if !wv.is_devtools_open() { // Check if this method exists, might need feature
        // wv.open_devtools(); // Or similar method if available
        println!("   NOTE: Devtools can be manually opened for existing pane '{}'", label);
    }

  } else {
    println!("   Pane '{}' does not exist. Creating new one.", label);
    println!("   Attempting WebviewBuilder::new for '{}' with URL: {}", label, parsed_url);

    // Catch potential panics during WebviewBuilder creation
    let builder_result = catch_unwind(AssertUnwindSafe(|| {
        WebviewBuilder::new(label, WebviewUrl::External(parsed_url.clone()))
            .devtools(true)
    }));

    let builder = match builder_result {
        Ok(b) => {
            println!("   WebviewBuilder::new for '{}' SUCCEEDED.", label);
            b
        }
        Err(panic_payload) => {
            println!("   ❌ PANIC during WebviewBuilder::new for '{}'!", label);
            if let Some(s) = panic_payload.downcast_ref::<String>() {
                println!("      Panic payload: {}", s);
            } else if let Some(s) = panic_payload.downcast_ref::<&str>() {
                println!("      Panic payload: {}", s);
            } else {
                println!("      Panic payload: (unknown type)");
            }
            return; // Stop processing this pane
        }
    };

    println!("   Attempting window.add_child for '{}'...", label);
    match window.add_child(
      builder,
      LogicalPosition::new(pos.0, pos.1),
      LogicalSize::new(size.0, size.1),
    ) {
      Ok(new_wv) => {
        println!("   ✅ Successfully added child webview '{}'", label);
        // new_wv.open_devtools(); // Optionally open devtools immediately if method exists
      }
      Err(e) => {
        println!("   ❌ Failed to add child webview '{}': {}", label, e);
      }
    }
  }
}
#[tauri::command]
async fn load_workspace(app: AppHandle, layout_name: String) -> Result<(), String> {
    let ws_json = read_workspace(&layout_name)?;
    let pane_cfg: Map<String, Value> = ws_json.as_object().cloned().unwrap_or_default();

    // make URLs easy to look up in the spawned thread
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

    thread::spawn(move || {
        let window = match app.get_window(OUTER_WINDOW) {
            Some(w) => w,
            None => {
                eprintln!("ERROR: main window '{OUTER_WINDOW}' not found");
                return;
            }
        };

        // --- create missing panes & navigate existing ones -----------
        for &label in &PANE_LABELS {
            match window.get_webview(label) {
                Some(wv) => {
                    if let Some(Some(ref url)) = pane_urls.get(label) {
                        let _ = wv.navigate(url.parse().unwrap());
                    }
                }
                None => {
                    // skip creating main1 if it has no URL (keeps the Svelte view)
                    if label == "main1" && pane_urls.get(label).and_then(|v| v.as_ref()).is_none() {
                        continue;
                    }
                    let url = pane_urls
                        .get(label)
                        .and_then(|v| v.as_ref())
                        .unwrap_or(&"about:blank".into())
                        .parse()
                        .unwrap_or_else(|_| "about:blank".parse().unwrap());

                    let builder = WebviewBuilder::new(label, WebviewUrl::External(url))
                        .devtools(true);

                    let _ = window.add_child(
                        builder,
                        LogicalPosition::new(0.0, 0.0),
                        LogicalSize::new(100.0, 100.0),
                    );
                }
            }
        }

        // --- first layout pass ---------------------------------------
        if let Ok(sz) = window.inner_size() {
            let (half_w, half_h) = (sz.width as f64 / 2.0, sz.height as f64 / 2.0);
            apply_quadrant_layout(&window, half_w, half_h);
        }

        // --- relayout on every resize/fullscreen ---------------------
        let win_clone = window.clone();
        window.on_window_event(move |event| {
            if let WindowEvent::Resized(size) = event {
                let (half_w, half_h) = (size.width as f64 / 2.0, size.height as f64 / 2.0);
                apply_quadrant_layout(&win_clone, half_w, half_h);
            }
        });
    });

    Ok(())
}

fn main() {
    println!("Starting Tauri application...");
    tauri::Builder::default()
        .setup(|app| {
            println!("   Setting up main window '{}'", OUTER_WINDOW);
            let (w, h) = (1200.0, 800.0); // Initial window size
            let main_window = tauri::window::WindowBuilder::new(app, OUTER_WINDOW)
                .title("Workspaces App")
                .inner_size(w, h)
                .build()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?; // Ensure error type matches
            println!("   ✅ Main window '{}' created.", OUTER_WINDOW);

            // Create initial Svelte pane (main1) filling the whole window
            println!("   Creating initial Svelte pane (main1)...");
            let svelte_builder = WebviewBuilder::new(
                PANE_LABELS[0], // "main1"
                WebviewUrl::App(Default::default()), // Loads Svelte app
            )
            .devtools(true); // Enable devtools for Svelte app from start

            main_window.add_child(
                svelte_builder,
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(w, h), // Full size
            ).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            println!("   ✅ Initial Svelte pane (main1) added full-size.");
            println!("✅ App ready — single-pane mode (Svelte on 'main1').");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_workspace])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
