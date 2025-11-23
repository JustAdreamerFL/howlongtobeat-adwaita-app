use libadwaita as adw;
use adw::prelude::*;
use gtk::{gdk, gdk_pixbuf, gio, glib};
use gtk::Orientation;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::api::{Game, HltbClient};

pub struct AppWindow {
    pub window: adw::ApplicationWindow,
    #[allow(dead_code)]
    search_entry: gtk::SearchEntry,
    #[allow(dead_code)]
    scrolled_window: gtk::ScrolledWindow,
    #[allow(dead_code)]
    list_box: gtk::ListBox,
    #[allow(dead_code)]
    status_page: adw::StatusPage,
    #[allow(dead_code)]
    stack: gtk::Stack,
    #[allow(dead_code)]
    client: Arc<HltbClient>,
    #[allow(dead_code)]
    view_mode: Arc<Mutex<ViewMode>>,
}

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    List,
    Grid,
}

impl AppWindow {
    pub fn new(app: &adw::Application) -> Self {
        let client = Arc::new(HltbClient::new());

        // Create the main window
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)
            .title("HowLongToBeat")
            .build();

        // Create header bar
        let header_bar = adw::HeaderBar::new();

        // Create search entry
        let search_entry = gtk::SearchEntry::builder()
            .placeholder_text("Search for a game...")
            .hexpand(true)
            .build();

        // Make the search entry draggable (allow dragging window from search field)
        let drag_controller = gtk::GestureDrag::new();
        let window_weak = window.downgrade();
        let search_entry_weak = search_entry.downgrade();
        
        drag_controller.connect_drag_begin(move |controller, _x, _y| {
            if let (Some(window), Some(entry)) = (window_weak.upgrade(), search_entry_weak.upgrade()) {
                // Only start dragging if the search entry doesn't have focus
                // This allows users to still type in the search field
                if !entry.has_focus() {
                    if let Some(surface) = window.surface() {
                        if let Ok(surface) = surface.downcast::<gdk::Toplevel>() {
                            let device = controller.device();
                            if let Some(device) = device {
                                surface.begin_move(&device, 1, 0.0, 0.0, controller.current_event_time());
                            }
                        }
                    }
                }
            }
        });
        
        search_entry.add_controller(drag_controller);

        header_bar.set_title_widget(Some(&search_entry));

        // Add preferences button to header bar
        let preferences_button = gtk::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .tooltip_text("Preferences")
            .build();

        // Create preferences menu
        let menu = gio::Menu::new();
        let view_section = gio::Menu::new();
        view_section.append(Some("List View"), Some("app.view-mode-list"));
        view_section.append(Some("Grid View"), Some("app.view-mode-grid"));
        menu.append_section(Some("View Mode"), &view_section);
        
        preferences_button.set_menu_model(Some(&menu));
        header_bar.pack_end(&preferences_button);

        // Create main content area with stack
        let stack = gtk::Stack::new();

        // Create status page (shown when no search is performed)
        let status_page = adw::StatusPage::builder()
            .icon_name("system-search-symbolic")
            .title("Search for a game")
            .description("Enter a game title to see completion times")
            .build();

        // Create list box for search results
        let list_box = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .css_classes(vec!["boxed-list"])
            .build();

        // Create scrolled window
        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true) // Make scrolled window expand to fill vertical space
            .child(&list_box)
            .build();

        // Add pages to stack
        stack.add_named(&status_page, Some("empty"));
        stack.add_named(&scrolled_window, Some("results"));

        // Set initial page
        stack.set_visible_child_name("empty");

        // Create main box
        let main_box = gtk::Box::new(Orientation::Vertical, 0);
        main_box.append(&header_bar);
        main_box.append(&stack);

        window.set_content(Some(&main_box));

        let app_window = Self {
            window,
            search_entry: search_entry.clone(),
            scrolled_window,
            list_box: list_box.clone(),
            status_page,
            stack: stack.clone(),
            client,
            view_mode: Arc::new(Mutex::new(ViewMode::List)),
        };

        // Connect search entry signal with debouncing
        let client_clone = app_window.client.clone();
        let list_box_clone = list_box.clone();
        let stack_clone = stack.clone();
        let search_timeout: Arc<Mutex<Option<glib::SourceId>>> = Arc::new(Mutex::new(None));

        search_entry.connect_search_changed(move |entry| {
            let query = entry.text().to_string();
            if query.is_empty() {
                // Cancel pending search
                if let Ok(mut timeout) = search_timeout.lock() {
                    if let Some(id) = timeout.take() {
                        id.remove();
                    }
                }
                stack_clone.set_visible_child_name("empty");
                return;
            }

            let client = client_clone.clone();
            let list_box = list_box_clone.clone();
            let stack = stack_clone.clone();
            let search_timeout_clone = search_timeout.clone();

            // Cancel previous search timeout
            if let Ok(mut timeout) = search_timeout.lock() {
                if let Some(id) = timeout.take() {
                    id.remove();
                }

                // Add new debounced search with 300ms delay
                let new_id = glib::timeout_add_local_once(Duration::from_millis(300), move || {
                    // Clear the timeout reference
                    if let Ok(mut timeout) = search_timeout_clone.lock() {
                        *timeout = None;
                    }

                    // Spawn async search
                    glib::spawn_future_local(async move {
                // Clear previous results
                while let Some(child) = list_box.first_child() {
                    list_box.remove(&child);
                }

                // Show loading state
                let loading_row = adw::ActionRow::builder().title("Searching...").build();
                let spinner = gtk::Spinner::new();
                spinner.start();
                loading_row.add_suffix(&spinner);
                list_box.append(&loading_row);
                stack.set_visible_child_name("results");

                // Perform search
                eprintln!("Searching for: {}", query);
                match client.search(&query).await {
                    Ok(games) => {
                        eprintln!("Search successful, found {} games", games.len());
                        // Clear loading indicator
                        while let Some(child) = list_box.first_child() {
                            list_box.remove(&child);
                        }

                        if games.is_empty() {
                            let no_results = adw::ActionRow::builder()
                                .title("No results found")
                                .subtitle(format!("No games found for '{}'", query))
                                .build();
                            list_box.append(&no_results);
                        } else {
                            for game in games {
                                let row = create_game_row(&game);
                                list_box.append(&row);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Search failed: {}", e);
                        // Clear loading indicator
                        while let Some(child) = list_box.first_child() {
                            list_box.remove(&child);
                        }

                        // Create error row with prominent error message
                        let error_row = adw::ActionRow::builder()
                            .title("Search Failed")
                            .build();
                        
                        // Create a label for the full error message
                        let error_label = gtk::Label::builder()
                            .label(format!("{}", e))
                            .wrap(true)
                            .wrap_mode(gtk::pango::WrapMode::Word)
                            .xalign(0.0)
                            .margin_top(8)
                            .margin_bottom(8)
                            .margin_start(12)
                            .margin_end(12)
                            .css_classes(vec!["dim-label"])
                            .build();
                        
                        let error_box = gtk::Box::new(Orientation::Vertical, 4);
                        error_box.append(&error_row);
                        error_box.append(&error_label);
                        
                        list_box.append(&error_box);
                    }
                }
                    });
                });

                *timeout = Some(new_id);
            }
        });

        app_window
    }

    pub fn present(&self) {
        self.window.present();
    }
}

fn create_game_row(game: &Game) -> adw::ExpanderRow {
    let row = adw::ExpanderRow::builder()
        .title(&game.game_name)
        .subtitle(if !game.profile_platform.is_empty() {
            &game.profile_platform
        } else {
            "Multiple Platforms"
        })
        .build();

    // Add game image if available
    if !game.game_image.is_empty() {
        let image_url = game.image_url();
        let image = gtk::Picture::builder()
            .width_request(80)
            .height_request(80)
            .can_shrink(true)
            .build();

        // Load image asynchronously
        let image_clone = image.clone();
        glib::spawn_future_local(async move {
            if let Ok(response) = reqwest::get(&image_url).await {
                if let Ok(bytes) = response.bytes().await {
                    if let Ok(pixbuf) = gdk_pixbuf::Pixbuf::from_read(std::io::Cursor::new(bytes.to_vec())) {
                        let texture = gdk::Texture::for_pixbuf(&pixbuf);
                        image_clone.set_paintable(Some(&texture));
                    }
                }
            }
        });

        row.add_prefix(&image);
    }

    // Create details box
    let details_box = gtk::Box::new(Orientation::Vertical, 12);
    details_box.set_margin_top(12);
    details_box.set_margin_bottom(12);
    details_box.set_margin_start(12);
    details_box.set_margin_end(12);

    // Add completion times
    if game.comp_main > 0 {
        let main_time = format_time(game.main_story_hours());
        let main_row = create_time_row("Main Story", &main_time, game.comp_main_count);
        details_box.append(&main_row);
    }

    if game.comp_plus > 0 {
        let plus_time = format_time(game.main_plus_hours());
        let plus_row = create_time_row("Main + Extras", &plus_time, game.comp_plus_count);
        details_box.append(&plus_row);
    }

    if game.comp_100 > 0 {
        let comp_time = format_time(game.completionist_hours());
        let comp_row = create_time_row("Completionist", &comp_time, game.comp_100_count);
        details_box.append(&comp_row);
    }

    if game.comp_all > 0 {
        let all_time = format_time(game.all_styles_hours());
        let all_row = create_time_row("All Styles", &all_time, game.count_comp);
        details_box.append(&all_row);
    }

    // Add link to game page
    let link_box = gtk::Box::new(Orientation::Horizontal, 6);
    link_box.set_halign(gtk::Align::Start);

    let link_button = gtk::LinkButton::builder()
        .label("View on HowLongToBeat")
        .uri(game.game_url())
        .build();

    link_box.append(&link_button);
    details_box.append(&link_box);

    row.add_row(&details_box);

    row
}

fn create_time_row(label: &str, time: &str, count: u32) -> gtk::Box {
    let row = gtk::Box::new(Orientation::Horizontal, 12);

    let label_widget = gtk::Label::builder()
        .label(label)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();

    let time_label = gtk::Label::builder()
        .label(time)
        .halign(gtk::Align::End)
        .css_classes(vec!["dim-label"])
        .build();

    let count_label = gtk::Label::builder()
        .label(format!("({} ratings)", count))
        .halign(gtk::Align::End)
        .css_classes(vec!["dim-label", "caption"])
        .build();

    row.append(&label_widget);
    row.append(&time_label);
    row.append(&count_label);

    row
}

fn format_time(hours: f64) -> String {
    if hours < 1.0 {
        format!("{}m", (hours * 60.0) as u32)
    } else if hours >= 100.0 {
        format!("{:.0}h", hours)
    } else {
        format!("{:.1}h", hours)
    }
}
