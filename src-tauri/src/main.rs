#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, SystemTrayEvent};

#[cfg(target_os = "macos")]
mod menu;
#[cfg(target_os = "linux")]
mod tray_menu;

fn main() {
    let builder = tauri::Builder::default();

    #[cfg(target_os = "macos")]
    let builder = builder.menu(menu::menu());

    #[cfg(target_os = "linux")]
    let builder = builder
        .system_tray(tray_menu::tray_menu())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        });
    #[cfg(not(target_os = "linux"))]
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    #[cfg(target_os = "linux")]
    {
        use tauri::WindowEvent;
        builder
            .build(tauri::generate_context!())
            .expect("error while building tauri application")
            .run(|app_handle, event| match event {
                tauri::RunEvent::WindowEvent {
                    label,
                    event: WindowEvent::CloseRequested { api, .. },
                    ..
                } => {
                    // not close main window
                    if label == "main" {
                        let app_handle = app_handle.clone();
                        let window = app_handle.get_window(&label).unwrap();
                        // use the exposed close api, and prevent the event loop to close
                        api.prevent_close();
                        window.hide().unwrap();
                    }
                }
                tauri::RunEvent::ExitRequested { api, .. } => {
                    api.prevent_exit();
                }
                _ => {}
            });
    }
}
