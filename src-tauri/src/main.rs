#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
    )]

mod rust {
    use std::{thread::sleep, time::Duration};
    use tauri::{ Manager};

    // this command is here just so the example doesn't throw an error
    #[tauri::command]
    fn close_splashscreen() {}

    pub fn main() {
        tauri::Builder::default()
            .setup(|app| {
                let splashscreen_window = app.get_window("splashscreen").unwrap();
                let main_window = app.get_window("main").unwrap();
                // main_window.on_window_event(|event|{
                //     Event::NewEvents(StartCause::Init)
                // });
                tauri::async_runtime::spawn(async move {
                    println!("Initializing...");
                    sleep(Duration::from_secs(3));
                    println!("Done initializing.");
                
                    splashscreen_window.close().unwrap();
                    main_window.show().unwrap();
                });
                Ok(())
            })
            .on_page_load(|win, _| {
                // win.listen("tauri://update-available".to_string(), move |msg| {
                //     println!("New version available: {:?}", msg);
                // });
                println!("Page Loaded: {:?}", win.label());
                if win.label() == "main" {
                    // splashscreen_window.close().unwrap();
                    println!("Page Loaded main");
                    win.show().unwrap();
                    win.eval(r#"
                        (function () {
                            const style = document.createElement('style');
                            style.textContent = `
                                body {
                                    background: #1ff !important;
                                }
                            `;
                            document.head.append(style);
                        })();
                    "#).expect("Error on injection js");
                     // win.eval("document.body.style = 'background-color: rgba(43, 46, 50, .6) !important;'").unwrap();
                }
            })
            .invoke_handler(tauri::generate_handler![close_splashscreen])
                .run(tauri::generate_context!())
                .expect("failed to run app");
    }
}

fn main() {
    rust::main();
}
