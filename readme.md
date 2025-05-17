cargo watch -x run

# Clipboard Layout Inverter

A cross-platform Rust application that monitors the system clipboard and automatically inverts text between QWERTY and Russian keyboard layouts. Built with `eframe` (egui) for the GUI and `copypasta` for clipboard access.

## Features
- Real-time clipboard monitoring
- Automatic conversion between QWERTY and Russian keyboard layouts
- Copy inverted text back to clipboard with one click
- Cross-platform support (Windows, Linux, macOS)

## Prerequisites
To build and run this application, you need:
- [Rust](https://www.rust-lang.org/tools/install) (stable, latest version recommended)
- Git
- For Linux: `libx11-dev` and `libxcb-shape0-dev` (for clipboard support)
  ```bash
  sudo apt-get install libx11-dev libxcb-shape0-dev
  ```

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/clipboard-inverter.git
   cd clipboard-inverter
   ```
2. Build and run the application:
   ```bash
   cargo run --release
   ```

## Usage
1. Launch the application.
2. The app monitors your clipboard in real-time.
3. When text is copied, it appears in the "Clipboard" section.
4. The inverted text (e.g., QWERTY to Russian or vice versa) appears in the "Inverted Layout" section.
5. Click the "Copy Inverted Text" button to copy the inverted text to your clipboard.
6. Errors (e.g., clipboard access issues) are displayed in red above the text areas.

## Building for Different Platforms
This project uses GitHub Actions to build binaries for Windows, Linux, and macOS, which are available in the [Releases](https://github.com/Ledich19/retype/tags) section.

### Download Pre-Built Binaries
- Go to the [Releases](https://github.com/Ledich19/retype/tags) page.
- Download the appropriate binary for your platform:
  - `clipboard-inverter-windows-amd64.exe` for Windows
  - `clipboard-inverter-linux-amd64` for Linux
  - `clipboard-inverter-darwin-amd64` for macOS
- Run the binary directly (no Rust installation required).

### Manual Cross-Compilation
To cross-compile locally, use `cross-rs`:
1. Install Docker or Podman.
2. Install `cross`:
   ```bash
   cargo install cross
   ```
3. Build for a specific platform, e.g., Linux:
   ```bash
   cross build --target x86_64-unknown-linux-gnu --release
   ```
   Find binaries in `target/<target>/release/`.




Укажи версию в Cargo.toml:
toml

Копировать
[package]
name = "clipboard-inverter"
version = "1.0.0"
Создай тег:
bash

Копировать
git add Cargo.toml
git commit -m "Set version to 1.0.0"
git tag 1.0.0
git push origin 1.0.0



git tag -d 1.0.0
git push origin :1.0.0
git tag 1.0.0
git push origin 1.0.0