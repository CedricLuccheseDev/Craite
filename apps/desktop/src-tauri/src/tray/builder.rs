use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle,
};

pub fn create_tray(app: &AppHandle) -> Result<TrayIcon, tauri::Error> {
    let show_hide = MenuItemBuilder::with_id("show_hide", "Show / Hide").build(app)?;
    let scan_now = MenuItemBuilder::with_id("scan_now", "Scan Now").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit CrAIte").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&show_hide)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&scan_now)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&quit)
        .build()?;

    let icon = app
        .default_window_icon()
        .cloned()
        .expect("default window icon must be set in tauri.conf.json");

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .tooltip("CrAIte")
        .on_menu_event(super::events::handle_menu_event)
        .on_tray_icon_event(super::events::handle_tray_icon_event)
        .build(app)
}
