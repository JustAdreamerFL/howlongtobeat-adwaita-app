# Installation Guide

## System Requirements

- Linux distribution (any modern distro)
- GTK4 >= 4.10
- LibAdwaita >= 1.5
- Rust >= 1.70 (for building from source)

## Method 1: Build with Cargo (Recommended for Development)

### Install Dependencies

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install libgtk-4-dev libadwaita-1-dev pkg-config build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Fedora
```bash
sudo dnf install gtk4-devel libadwaita-devel pkg-config gcc
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Arch Linux
```bash
sudo pacman -S gtk4 libadwaita pkgconf base-devel rust
```

### Build and Run
```bash
git clone https://github.com/JustAdreamerFL/howlongtobeat-adwaita-app.git
cd howlongtobeat-adwaita-app
cargo build --release
./target/release/howlongtobeat
```

## Method 2: Build with Meson (Recommended for System Installation)

### Install Dependencies
Same as above, plus:
```bash
# Ubuntu/Debian
sudo apt install meson ninja-build

# Fedora
sudo dnf install meson ninja-build

# Arch Linux
sudo pacman -S meson ninja
```

### Build and Install
```bash
git clone https://github.com/JustAdreamerFL/howlongtobeat-adwaita-app.git
cd howlongtobeat-adwaita-app
meson setup build
meson compile -C build
sudo meson install -C build
```

This will install the application system-wide and integrate it with your desktop environment.

## Running the Application

After system-wide installation, you can:
- Launch from your application menu/launcher
- Run from terminal: `howlongtobeat`

For development builds with cargo:
```bash
cargo run --release
```

## Troubleshooting

### Missing GTK4/LibAdwaita
If you see errors about missing libraries, ensure you have GTK4 and LibAdwaita installed:
```bash
pkg-config --modversion gtk4 libadwaita-1
```

### Compilation Errors
Make sure you have the latest Rust toolchain:
```bash
rustup update stable
```

### Runtime Issues
Check that your system has proper graphics acceleration and that GTK4 is working:
```bash
gtk4-demo
```
