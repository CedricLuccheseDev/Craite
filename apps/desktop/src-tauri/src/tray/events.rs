use tauri::{AppHandle, Manager};
use tauri::menu::MenuEvent;
use tauri::tray::{TrayIconEvent, MouseButton, MouseButtonState};

pub fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "show_hide" => toggle_window_visibility(app),
        "scan_now" => trigger_background_scan(app),
        "quit" => app.exit(0),
        _ => {}
    }
}

pub fn handle_tray_icon_event(tray: &tauri::tray::TrayIcon, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        toggle_window_visibility(tray.app_handle());
    }
}

fn toggle_window_visibility(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

fn trigger_background_scan(app: &AppHandle) {
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        crate::background::scanner::run_scan_cycle(&handle).await;
    });
}
