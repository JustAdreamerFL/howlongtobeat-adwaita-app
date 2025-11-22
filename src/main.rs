mod api;
mod ui;

use libadwaita as adw;
use adw::prelude::*;
use gtk::glib;

const APP_ID: &str = "com.github.justadreamerfl.HowLongToBeat";

fn main() -> glib::ExitCode {
    // Create application
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect activate signal
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = ui::AppWindow::new(app);
    window.present();
}
