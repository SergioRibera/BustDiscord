#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread::sleep;

use app::window::{get_splash_screen, get_window};

use crate::app::commands::*;

mod app;

fn main() {
    let mut tauri_app = tauri::Builder::default();
    tauri_app
        .plugin(windowStatePlugin::default().build())
        .invoke_handler(tauri::generate_handler![
            drag_window,
            fullscreen,
            open_browser,
        ])
        .setup(|app| {
            let splashscreen_window = get_splash_screen();
            let main_window = get_window(app, config);
            tauri::async_runtime::spawn(async move {
                println!("Initializing...");
                sleep(Duration::from_secs(3));
                println!("Done initializing.");

                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                #[cfg(target_os = "macos")]
                {
                    event.window().minimize().unwrap();
                }

                #[cfg(not(target_os = "macos"))]
                event.window().close().unwrap();

                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
