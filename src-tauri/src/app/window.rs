use crate::app::config::PakeConfig;
use std::path::PathBuf;
use tauri::{App, Window, WindowBuilder, WindowUrl};

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

pub fn get_window(app: &mut App, config: PakeConfig) -> Window {
    let window_config = config.windows;
    let url = WindowUrl::App("https://canary.discord.com/app".parse().unwrap());

    let mut window_builder = WindowBuilder::new(app, "main", url)
        .title("BustDiscord")
        .visible(false) // Prevent initial shaking
        .resizable(window_config.resizable)
        .inner_size(window_config.width, window_config.height)
        .initialization_script(include_str!("../inject/style.js"))
        .initialization_script(include_str!("../inject/event.js"))
        .initialization_script(include_str!("../inject/component.js"));

    #[cfg(target_os = "macos")]
    {
        let title_bar_style = if window_config.transparent {
            TitleBarStyle::Overlay
        } else {
            TitleBarStyle::Visible
        };
        window_builder = window_builder.title_bar_style(title_bar_style)
    }

    window_builder.build().unwrap()
}

pub fn get_splash_screen() -> Window {
    WindowBuilder::new(
        app,
        "splashscreen",
        WindowUrl::App(PathBuf::from("src/splashscreen.html")),
    )
    .title("BustDiscord")
    .visible(false)
    .resizable(false)
    .decorations(false)
    .skip_taskbar(true)
    .center();
}
