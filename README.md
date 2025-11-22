# HowLongToBeat - LibAdwaita App

A native Linux application for searching game completion times from HowLongToBeat.com. Built with Rust, GTK4, and LibAdwaita.

![HowLongToBeat App](screenshot.png)

## Features

- 🔍 Search for games by title
- ⏱️ View completion times (Main Story, Main + Extras, Completionist, All Styles)
- 📊 See rating counts for each completion category
- 🎮 Platform information
- 🔗 Direct links to HowLongToBeat.com game pages
- 🎨 Beautiful LibAdwaita interface following GNOME Human Interface Guidelines

## Installation

### Dependencies

- GTK4 (>= 4.10)
- LibAdwaita (>= 1.5)
- Rust (>= 1.70)

#### Ubuntu/Debian
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev pkg-config
```

#### Fedora
```bash
sudo dnf install gtk4-devel libadwaita-devel
```

#### Arch Linux
```bash
sudo pacman -S gtk4 libadwaita
```

### Building from Source

```bash
git clone https://github.com/JustAdreamerFL/howlongtobeat-adwaita-app.git
cd howlongtobeat-adwaita-app
cargo build --release
```

The binary will be available at `target/release/howlongtobeat`.

## Running

```bash
cargo run --release
```

Or run the compiled binary directly:

```bash
./target/release/howlongtobeat
```

## Architecture

This application is a port of the [Raycast HowLongToBeat extension](https://github.com/raycast/extensions/tree/main/extensions/how-long-to-beat) to a native Linux application.

- **Language**: Rust
- **UI Framework**: GTK4 with LibAdwaita
- **API Client**: Asynchronous HTTP client using reqwest
- **Build System**: Cargo

## Project Structure

```
.
├── src/
│   ├── main.rs          # Application entry point
│   ├── api.rs           # HowLongToBeat API client
│   └── ui.rs            # GTK4/LibAdwaita user interface
├── Cargo.toml           # Rust dependencies
└── README.md
```

## Credits

- Built by AI (as stated in the original README)
- Ported from the [Raycast HowLongToBeat extension](https://github.com/raycast/extensions/tree/main/extensions/how-long-to-beat)
- Data provided by [HowLongToBeat.com](https://howlongtobeat.com/)

## License

This project is open source and available under the MIT License.
