![SlickNotes](https://i.imgur.com/4FWeVdg.png)

# SlickNotes

A lightweight markdown editor and previewer for Linux. Open a folder, browse your notes in the sidebar, and edit or preview them with a single click.

## Features

- Browse `.md` files from any folder
- Edit with syntax-highlighted markdown (CodeMirror 6)
- Toggle between edit and preview modes
- Auto-save with visual indicator
- Create, rename, and delete notes
- Remembers your last opened folder
- Dark theme

## Tech Stack

- [Tauri v2](https://v2.tauri.app/) (Rust backend)
- [Vue 3](https://vuejs.org/) (Composition API)
- [CodeMirror 6](https://codemirror.net/) (markdown editor)
- [markdown-it](https://github.com/markdown-it/markdown-it) (preview rendering)

## Installation

See [INSTALL.md](INSTALL.md) for build, install, and uninstall instructions.

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/)
- System libraries:
  ```bash
  sudo apt install -y libwebkit2gtk-4.1-dev libgtk-3-dev
  ```

### Setup

```bash
git clone https://github.com/perdidao/slicknotes.git
cd slicknotes
pnpm install
```

### Run in dev mode

```bash
pnpm tauri dev
```

This starts the Vite dev server with hot reload for the frontend and compiles the Rust backend. Changes to Vue files are reflected instantly; changes to Rust files trigger a recompile.

### Project structure

```
src/                  # Vue frontend
  components/         # Sidebar, Editor, Preview
  composables/        # Shared state (useFiles.js)
src-tauri/            # Rust backend
  src/lib.rs          # Tauri commands (file operations)
```

### Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) with:
- [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## License

[MIT](LICENSE)
