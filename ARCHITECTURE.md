# Architecture Documentation

## Overview

HowLongToBeat is a native Linux desktop application built with Rust, GTK4, and LibAdwaita. It provides a modern, native interface for searching game completion times from the HowLongToBeat.com API.

## Technology Stack

### Core Technologies
- **Language**: Rust (2021 edition)
- **UI Framework**: GTK4 with LibAdwaita
- **HTTP Client**: reqwest (with tokio async runtime)
- **Serialization**: serde/serde_json
- **Build System**: Cargo (with optional Meson for system installation)

### Key Dependencies
```toml
gtk4 = "0.9"          # GTK4 Rust bindings
libadwaita = "0.7"    # LibAdwaita Rust bindings
reqwest = "0.12"      # HTTP client
serde = "1.0"         # Serialization framework
tokio = "1"           # Async runtime
anyhow = "1.0"        # Error handling
```

## Project Structure

```
howlongtobeat-adwaita-app/
├── src/
│   ├── main.rs       # Application entry point and setup
│   ├── api.rs        # HowLongToBeat API client and data structures
│   └── ui.rs         # GTK4/LibAdwaita user interface components
├── data/
│   ├── *.desktop     # Desktop entry file
│   ├── *.metainfo.xml # AppData metadata
│   └── meson.build   # Data installation rules
├── build-aux/
│   └── meson_post_install.py # Post-installation script
├── Cargo.toml        # Rust dependencies
├── meson.build       # Meson build configuration
└── README.md         # User documentation
```

## Module Architecture

### main.rs
**Purpose**: Application initialization and lifecycle management

**Key Components**:
- Application ID: `com.github.justadreamerfl.HowLongToBeat`
- Creates `adw::Application` instance
- Connects activation signal to UI builder
- Manages application lifecycle

**Code Flow**:
```rust
main() -> Create Application -> Connect activate signal -> Run event loop
                                          ↓
                                    build_ui() -> Create AppWindow -> Present
```

### api.rs
**Purpose**: HowLongToBeat API client and data models

**Key Structures**:

1. **SearchRequest**: Request payload for game search
   - `search_terms`: Array of search keywords
   - `search_options`: Filtering and sorting options
   - `size`: Number of results (default: 20)

2. **SearchResponse**: API response wrapper
   - `data`: Array of game results
   - Pagination metadata

3. **Game**: Individual game data
   - Basic info: `game_id`, `game_name`, `game_image`
   - Platform: `profile_platform`
   - Completion times (in seconds):
     - `comp_main`: Main story
     - `comp_plus`: Main + extras
     - `comp_100`: Completionist
     - `comp_all`: All styles
   - Rating counts for each category

4. **HltbClient**: HTTP client wrapper
   - Configured with proper User-Agent
   - Async search method
   - POST requests to `https://howlongtobeat.com/api/search`

**Helper Methods**:
- `Game::main_story_hours()`: Convert seconds to hours
- `Game::image_url()`: Construct full image URL
- `Game::game_url()`: Construct game page URL

### ui.rs
**Purpose**: User interface components and interaction logic

**Key Components**:

1. **AppWindow**: Main application window
   - `adw::ApplicationWindow`: Top-level window
   - `gtk::SearchEntry`: Search input field
   - `gtk::ListBox`: Results container
   - `gtk::Stack`: Switch between empty state and results
   - `adw::StatusPage`: Empty state display

2. **UI Hierarchy**:
```
ApplicationWindow
└── Box (vertical)
    ├── HeaderBar
    │   └── SearchEntry (title widget)
    └── Stack
        ├── StatusPage (empty state)
        └── ScrolledWindow
            └── ListBox
                └── ExpanderRow (per game)
                    └── Details Box
                        ├── Time rows
                        └── Link button
```

3. **Search Flow**:
   - User types in SearchEntry
   - `search_changed` signal triggered
   - Spawn async task with `glib::spawn_future_local`
   - Clear existing results, show loading spinner
   - Perform API search via `HltbClient`
   - Update UI with results or error message

4. **Result Display**:
   - Each game in an `adw::ExpanderRow`
   - Title: Game name
   - Subtitle: Platform information
   - Expandable content shows:
     - Completion times with rating counts
     - Link to game page on HowLongToBeat.com

## API Integration

### HowLongToBeat API
**Endpoint**: `https://howlongtobeat.com/api/search`

**Request Format**:
```json
{
  "searchType": "games",
  "searchTerms": ["game title"],
  "searchPage": 1,
  "size": 20,
  "searchOptions": {
    "games": {
      "userId": 0,
      "platform": "",
      "sortCategory": "popular",
      "rangeCategory": "main",
      ...
    },
    ...
  }
}
```

**Response Format**:
```json
{
  "data": [
    {
      "game_id": 12345,
      "game_name": "Example Game",
      "comp_main": 28800,  // seconds
      "comp_plus": 43200,
      "comp_100": 72000,
      ...
    }
  ],
  "count": 1,
  ...
}
```

**Important Notes**:
- Times are returned in seconds, converted to hours for display
- Requires proper User-Agent header
- Uses POST method (not GET)
- Field names use camelCase (hence `#![allow(non_snake_case)]`)

## Async Architecture

The application uses GTK's main loop integration with async Rust:

1. **Main Thread**: GTK event loop runs on main thread
2. **Tokio Runtime**: Initialized at application startup and kept alive for the entire application lifetime
3. **Async Tasks**: API calls run using tokio runtime for I/O operations
4. **Integration**: `glib::spawn_future_local` bridges GTK and tokio
5. **UI Updates**: Results posted back to main thread via GLib closures

**Flow Diagram**:
```
User Input (Main Thread)
    ↓
spawn_future_local
    ↓
Tokio Runtime → HTTP Request → API Response
    ↓
Closure on Main Thread → Update GTK Widgets
```

**Important Note**: The tokio runtime MUST be initialized before the GTK application runs. The `reqwest` HTTP client requires a tokio runtime context to execute async operations. In `main.rs`, we:
1. Create a tokio runtime
2. Enter its context with `rt.enter()`
3. Keep both the runtime and guard alive for the entire application

Without this initialization, async HTTP requests will hang indefinitely, causing the UI to show a loading spinner without ever completing the request.

## UI Design Principles

Following GNOME Human Interface Guidelines:

1. **Adaptive Layout**: Works on various screen sizes
2. **LibAdwaita Widgets**: Modern GNOME appearance
3. **Empty States**: Clear guidance when no results
4. **Loading Indicators**: Feedback during operations
5. **Error Handling**: User-friendly error messages
6. **Boxed Lists**: Grouped, card-like appearance

## Build System

### Cargo
Standard Rust build tool:
```bash
cargo build --release
cargo run --release
```

### Meson (Optional)
For system integration:
```bash
meson setup build
meson compile -C build
meson install -C build
```

Benefits:
- System-wide installation
- Desktop file integration
- Post-install hooks for cache updates

## Future Enhancement Opportunities

1. **Caching**: Cache search results locally
2. **Favorites**: Save favorite games
3. **Filters**: Filter by platform, genre
4. **Sorting**: Sort results by different criteria
5. **Images**: Display game cover art
6. **Themes**: Dark/light mode preference
7. **Offline Mode**: Browse cached results offline
8. **Export**: Export data to CSV/JSON
9. **Statistics**: Personal gaming statistics
10. **Multi-language**: Internationalization support

## Performance Considerations

- Async HTTP requests don't block UI
- Lazy loading of game details (expandable rows)
- Minimal dependencies for fast compilation
- Release builds are optimized
- GTK4 provides hardware acceleration

## Testing

Currently no automated tests. Recommendations:

1. **Unit Tests**: Test API client serialization
2. **Integration Tests**: Test search functionality
3. **UI Tests**: Test widget creation and interaction
4. **Manual Testing**: Test with real API

## Deployment

### Flatpak (Recommended for Distribution)
Could be packaged as Flatpak for easy distribution:
- Sandboxed environment
- Automatic dependency management
- Cross-distribution compatibility

### Distribution Packages
Could be packaged for:
- AUR (Arch Linux)
- Fedora COPR
- Ubuntu PPA
- Flathub

## License

MIT License - See LICENSE file for details

## Credits

- Ported from Raycast HowLongToBeat extension
- Data from HowLongToBeat.com
- Built with GNOME technologies
