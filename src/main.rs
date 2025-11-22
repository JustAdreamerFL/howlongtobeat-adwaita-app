mod api;
mod ui;

use adw::prelude::*;
use gtk::glib;
use libadwaita as adw;

const APP_ID: &str = "com.github.justadreamerfl.HowLongToBeat";

fn main() -> glib::ExitCode {
    // Initialize tokio runtime for async HTTP requests
    // This is required for reqwest to work properly
    // We keep a handle to the runtime to ensure it stays alive for the entire application lifetime
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    
    // Enter the runtime context so that tokio async operations work
    let _guard = rt.enter();
    
    // Create application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect activate signal
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = ui::AppWindow::new(app);
    window.present();
}
