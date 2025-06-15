#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{LogicalPosition, LogicalSize, WebviewUrl};

#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let width = 1920.;
      let height = 1080.;

      let window = tauri::window::WindowBuilder::new(app, "custom_main")
        .inner_size(width, height)
        .build()?;

      // Define all URLs and their positions
      let webviews = [
        // (label, url, x_pos, y_pos)
        ("main1", None, 0., 0.),
        ("main2", Some("https://app.haldor.se/"), width / 2., 0.),
        ("main3", Some("https://www.microsoft.com/sv-se/microsoft-teams/log-in?market=se"), 0., height / 2.),
        ("main4", Some("https://nokportalen.se/"), width / 2., height / 2.),
      ];

      // Create webviews in a loop
      for (_i, (label, url_opt, x, y)) in webviews.iter().enumerate() {
        let webview_url = match url_opt {
          Some(url) => WebviewUrl::External((*url).parse().unwrap()),
          None => WebviewUrl::App(Default::default()),
        };

        let builder = tauri::webview::WebviewBuilder::new(*label, webview_url)
        .auto_resize()
        // force all popups (window.open or <a target="_blank">) into this same view
        .initialization_script(r#"
            // override window.open to reuse this webview
            window.open = (url) => { window.location.href = url; };
            // catch any <a target="_blank">
            document.addEventListener('click', e => {
            const a = e.target.closest('a[target="_blank"]');
            if (a) {
                e.preventDefault();
                window.location.href = a.href;
            }
            }, true);
        "#);

        let _ = window.add_child(
        builder,
        LogicalPosition::new(*x, *y),
        LogicalSize::new(width / 2., height / 2.)
        )?;
      }

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}