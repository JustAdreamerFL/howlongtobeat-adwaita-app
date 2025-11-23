use libadwaita as adw;
use adw::prelude::*;
use gtk::{gdk, gdk_pixbuf, gio, glib};
use gtk::Orientation;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::api::{Game, HltbClient};

#[allow(dead_code)]
pub struct AppWindow {
    pub window: adw::ApplicationWindow,
    search_entry: gtk::SearchEntry,
    scrolled_window: gtk::ScrolledWindow,
    list_box: gtk::ListBox,
    flow_box: gtk::FlowBox,
    results_stack: gtk::Stack, // Stack to switch between list and grid
    status_page: adw::StatusPage,
    stack: gtk::Stack,
    client: Arc<HltbClient>,
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

        // Create list box for search results (list view)
        let list_box = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .css_classes(vec!["boxed-list"])
            .build();

        // Create scrolled window for list view
        let list_scrolled = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .child(&list_box)
            .build();

        // Create flow box for search results (grid view)
        let flow_box = gtk::FlowBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .max_children_per_line(3)
            .min_children_per_line(1)
            .column_spacing(12)
            .row_spacing(12)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .homogeneous(true)
            .build();

        // Create scrolled window for grid view
        let grid_scrolled = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .child(&flow_box)
            .build();

        // Create a stack to switch between list and grid views
        let results_stack = gtk::Stack::new();
        results_stack.add_named(&list_scrolled, Some("list"));
        results_stack.add_named(&grid_scrolled, Some("grid"));
        results_stack.set_visible_child_name("list");

        // Add pages to main stack
        stack.add_named(&status_page, Some("empty"));
        stack.add_named(&results_stack, Some("results"));

        // Set initial page
        stack.set_visible_child_name("empty");

        // Create main box
        let main_box = gtk::Box::new(Orientation::Vertical, 0);
        main_box.append(&header_bar);
        main_box.append(&stack);

        window.set_content(Some(&main_box));

        let app_window = Self {
            window: window.clone(),
            search_entry: search_entry.clone(),
            scrolled_window: list_scrolled.clone(),
            list_box: list_box.clone(),
            flow_box: flow_box.clone(),
            results_stack: results_stack.clone(),
            status_page,
            stack: stack.clone(),
            client,
            view_mode: Arc::new(Mutex::new(ViewMode::List)),
        };

        // Connect search entry signal with debouncing
        let client_clone = app_window.client.clone();
        let list_box_clone = list_box.clone();
        let flow_box_clone = flow_box.clone();
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
            let flow_box = flow_box_clone.clone();
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
                while let Some(child) = flow_box.first_child() {
                    flow_box.remove(&child);
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
                        while let Some(child) = flow_box.first_child() {
                            flow_box.remove(&child);
                        }

                        if games.is_empty() {
                            let no_results = adw::ActionRow::builder()
                                .title("No results found")
                                .subtitle(format!("No games found for '{}'", query))
                                .build();
                            list_box.append(&no_results);
                        } else {
                            for game in &games {
                                // Add to list view
                                let row = create_game_row(game);
                                list_box.append(&row);
                                
                                // Add to grid view
                                let card = create_game_card(game);
                                flow_box.insert(&card, -1);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Search failed: {}", e);
                        // Clear loading indicator from both views
                        while let Some(child) = list_box.first_child() {
                            list_box.remove(&child);
                        }
                        while let Some(child) = flow_box.first_child() {
                            flow_box.remove(&child);
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

        // Add view mode actions
        let view_mode_clone = app_window.view_mode.clone();
        let results_stack_clone = results_stack.clone();
        
        let list_action = gio::SimpleAction::new("view-mode-list", None);
        list_action.connect_activate(move |_, _| {
            if let Ok(mut mode) = view_mode_clone.lock() {
                *mode = ViewMode::List;
                results_stack_clone.set_visible_child_name("list");
            }
        });
        window.add_action(&list_action);

        let view_mode_clone2 = app_window.view_mode.clone();
        let results_stack_clone2 = results_stack.clone();
        
        let grid_action = gio::SimpleAction::new("view-mode-grid", None);
        grid_action.connect_activate(move |_, _| {
            if let Ok(mut mode) = view_mode_clone2.lock() {
                *mode = ViewMode::Grid;
                results_stack_clone2.set_visible_child_name("grid");
            }
        });
        window.add_action(&grid_action);

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
                if response.status().is_success() {
                    if let Ok(bytes) = response.bytes().await {
                        // Use PixbufLoader which can handle the bytes directly
                        let loader = gdk_pixbuf::PixbufLoader::new();
                        if loader.write(&bytes).is_ok() && loader.close().is_ok() {
                            if let Some(pixbuf) = loader.pixbuf() {
                                let texture = gdk::Texture::for_pixbuf(&pixbuf);
                                image_clone.set_paintable(Some(&texture));
                            }
                        }
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

fn create_game_card(game: &Game) -> gtk::Box {
    let card = gtk::Box::new(Orientation::Vertical, 0);
    card.set_css_classes(&["card"]);
    card.set_width_request(250);

    // Create image box
    let image = gtk::Picture::builder()
        .width_request(250)
        .height_request(150)
        .can_shrink(false)
        .build();

    // Load image asynchronously
    if !game.game_image.is_empty() {
        let image_url = game.image_url();
        let image_clone = image.clone();
        glib::spawn_future_local(async move {
            if let Ok(response) = reqwest::get(&image_url).await {
                if response.status().is_success() {
                    if let Ok(bytes) = response.bytes().await {
                        // Use PixbufLoader which can handle the bytes directly
                        let loader = gdk_pixbuf::PixbufLoader::new();
                        if loader.write(&bytes).is_ok() && loader.close().is_ok() {
                            if let Some(pixbuf) = loader.pixbuf() {
                                let texture = gdk::Texture::for_pixbuf(&pixbuf);
                                image_clone.set_paintable(Some(&texture));
                            }
                        }
                    }
                }
            }
        });
    }

    card.append(&image);

    // Create info box
    let info_box = gtk::Box::new(Orientation::Vertical, 6);
    info_box.set_margin_top(12);
    info_box.set_margin_bottom(12);
    info_box.set_margin_start(12);
    info_box.set_margin_end(12);

    // Game title
    let title_label = gtk::Label::builder()
        .label(&game.game_name)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .xalign(0.0)
        .css_classes(vec!["title-4"])
        .build();
    info_box.append(&title_label);

    // Platform
    if !game.profile_platform.is_empty() {
        let platform_label = gtk::Label::builder()
            .label(&game.profile_platform)
            .xalign(0.0)
            .css_classes(vec!["dim-label", "caption"])
            .build();
        info_box.append(&platform_label);
    }

    // Main completion time
    if game.comp_main > 0 {
        let time_box = gtk::Box::new(Orientation::Horizontal, 6);
        time_box.set_margin_top(6);
        
        let time_icon = gtk::Label::builder()
            .label("⏱")
            .build();
        time_box.append(&time_icon);
        
        let time_label = gtk::Label::builder()
            .label(format!("Main: {}", format_time(game.main_story_hours())))
            .xalign(0.0)
            .build();
        time_box.append(&time_label);
        
        info_box.append(&time_box);
    }

    card.append(&info_box);

    // Make the card clickable to open the game URL
    let gesture = gtk::GestureClick::new();
    let game_url = game.game_url();
    gesture.connect_released(move |_gesture, _n, _x, _y| {
        // Open URL using safer method
        let _ = gio::AppInfo::launch_default_for_uri(&game_url, None::<&gio::AppLaunchContext>);
    });
    card.add_controller(gesture);

    card
}
