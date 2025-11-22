# Quick Start Guide

## What is HowLongToBeat?

HowLongToBeat is a native Linux desktop application that helps you find out how long it takes to complete video games. Search for any game and see:
- Main story completion time
- Main + extras completion time
- 100% completion time
- All play styles average time

## Prerequisites

You need:
- A Linux distribution (Ubuntu, Fedora, Arch, etc.)
- GTK4 and LibAdwaita installed
- Rust toolchain (for building from source)

## 5-Minute Setup

### Ubuntu/Debian Users

```bash
# 1. Install dependencies
sudo apt update
sudo apt install libgtk-4-dev libadwaita-1-dev pkg-config build-essential

# 2. Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. Clone and build
git clone https://github.com/JustAdreamerFL/howlongtobeat-adwaita-app.git
cd howlongtobeat-adwaita-app
cargo build --release

# 4. Run!
./target/release/howlongtobeat
```

### Fedora Users

```bash
# 1. Install dependencies
sudo dnf install gtk4-devel libadwaita-devel pkg-config gcc rust cargo

# 2. Clone and build
git clone https://github.com/JustAdreamerFL/howlongtobeat-adwaita-app.git
cd howlongtobeat-adwaita-app
cargo build --release

# 3. Run!
./target/release/howlongtobeat
```

### Arch Linux Users

```bash
# 1. Install dependencies
sudo pacman -S gtk4 libadwaita pkgconf base-devel rust

# 2. Clone and build
git clone https://github.com/JustAdreamerFL/howlongtobeat-adwaita-app.git
cd howlongtobeat-adwaita-app
cargo build --release

# 3. Run!
./target/release/howlongtobeat
```

## Using the Application

1. **Launch the app** - Click the application icon or run from terminal
2. **Search** - Type a game name in the search bar at the top
3. **Browse results** - Results appear as you type
4. **View details** - Click the arrow next to a game to expand and see completion times
5. **Visit website** - Click "View on HowLongToBeat" to see more details

## Tips

- The app searches as you type, so wait a moment between keystrokes
- Expand any game row to see detailed completion times
- Rating counts show how many people submitted times
- Times are shown in hours (h) or minutes (m) depending on game length

## Keyboard Shortcuts

- `Ctrl+Q` - Quit the application
- `Ctrl+F` - Focus search bar
- `Escape` - Clear search

## Troubleshooting

**App won't start?**
- Make sure GTK4 is installed: `pkg-config --modversion gtk4`
- Check LibAdwaita is available: `pkg-config --modversion libadwaita-1`

**Build errors?**
- Update Rust: `rustup update stable`
- Ensure all dependencies are installed

**Search not working?**
- Check your internet connection
- The app needs access to howlongtobeat.com

**UI looks wrong?**
- Make sure you're using GTK4 (not GTK3)
- Update your system: `sudo apt update && sudo apt upgrade`

## What's Next?

- Read the full [README.md](README.md) for more information
- Check [INSTALL.md](INSTALL.md) for detailed installation instructions
- See [CONTRIBUTING.md](CONTRIBUTING.md) if you want to help improve the app
- Report issues on [GitHub](https://github.com/JustAdreamerFL/howlongtobeat-adwaita-app/issues)

## Support

Having trouble? Open an issue on GitHub with:
- Your Linux distribution and version
- GTK4 and LibAdwaita versions (`pkg-config --modversion gtk4 libadwaita-1`)
- Error messages or unexpected behavior
- Steps to reproduce the issue

Enjoy using HowLongToBeat! 🎮
