mod api;
mod ui;

use adw::prelude::*;
use gtk::glib;
use libadwaita as adw;

const APP_ID: &str = "com.github.justadreamerfl.HowLongToBeat";

fn main() -> glib::ExitCode {
    // Initialize tokio runtime for async HTTP requests
    // This is required for reqwest to work properly
    // The runtime and guard must stay alive for the entire application lifetime,
    // so we declare them here and let them live until main() returns (after app.run() exits)
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    
    // Enter the runtime context so that tokio async operations work
    // The guard ensures the runtime context remains active
    let _guard = rt.enter();
    
    // Create application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect activate signal
    app.connect_activate(build_ui);

    // Run the application (blocks until the app exits)
    // Both rt and _guard remain in scope during this entire time
    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = ui::AppWindow::new(app);
    window.present();
}
