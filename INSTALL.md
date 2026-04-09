# SlickNotes - Install & Uninstall

## Prerequisites

- Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Node.js (v18+) and pnpm
- System libraries: `sudo apt install -y libwebkit2gtk-4.1-dev libgtk-3-dev`

## Build

```bash
source "$HOME/.cargo/env"
pnpm install
pnpm tauri build
```

The release bundle is generated at `src-tauri/target/release/bundle/`.

## Install

```bash
sudo dpkg -i src-tauri/target/release/bundle/deb/slick-notes_*.deb
```

SlickNotes will appear in your application launcher after installation.

## Uninstall

```bash
sudo apt remove slick-notes
```

## Changing the App Icon

Place a PNG image (512x512 or larger) and run:

```bash
pnpm tauri icon /path/to/your-icon.png
```

This regenerates all icon sizes in `src-tauri/icons/`.
