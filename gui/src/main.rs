slint::include_modules!();


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.set_current_page(Page::Login);

    let ui_handle_login = ui.as_weak();
    ui.on_switch_to_login(move || {
        if let Some(ui) = ui_handle_login.upgrade() {
            ui.set_current_page(Page::Login);
            println!("Switched to Login Page");
        }
    });

    let ui_handle_dashboard = ui.as_weak();
    ui.on_switch_to_dashboard(move || {
        if let Some(ui) = ui_handle_dashboard.upgrade() {
            ui.set_current_page(Page::Dashboard);
            println!("Switched to Dashboard Page");
        }
    });

    let ui_handle_settings = ui.as_weak();
    ui.on_switch_to_settings(move || {
        if let Some(ui) = ui_handle_settings.upgrade() {
            ui.set_current_page(Page::Settings);
            println!("Switched to Settings Page");
        }
    });

    let ui_handle_login_request = ui.as_weak();
    ui.on_login_requested_from_child(move |username, password| { // Rust side still uses names
        if let Some(ui) = ui_handle_login_request.upgrade() {
            println!("Login requested for user: {}, password: {}", username, password);
            ui.set_current_page(Page::Dashboard);
        }
    });

    ui.run()
}