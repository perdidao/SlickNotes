# SlickNotes — Markdown Editor/Previewer

## Overview

SlickNotes is a simple, lightweight markdown editor and previewer for Linux. It reads `.md` files from a user-specified folder, lists them by title in a sidebar, and allows editing with a toggle between edit and preview modes.

## Architecture

Single Tauri v2 application with two layers:

- **Rust backend** — Tauri commands for filesystem operations: list `.md` files in a directory, read file content, write file content. Users pick a folder via a native directory picker dialog.
- **Vue 3 frontend** — Single-page app with sidebar (file list) and main content area (editor/preview toggle).

No database. No complex state management. The app works directly with the filesystem.

## Tech Stack

| Layer       | Technology                          |
|-------------|-------------------------------------|
| Framework   | Tauri v2                            |
| Frontend    | Vue 3 (Composition API, `<script setup>`) |
| Build       | Vite                                |
| Markdown    | markdown-it                         |
| File I/O    | Rust std::fs                        |
| Folder pick | Tauri dialog plugin                 |
| Persistence | Tauri store plugin (last folder)    |
| Styling     | Plain CSS                           |

## Components & Layout

### Sidebar

- Lists all `.md` files in the selected folder (non-recursive, top-level only)
- Each entry displays the filename without the `.md` extension
- Clicking a file selects it and loads it into the main area
- Visual indicator for the currently selected file

### Main Area

- **Edit mode:** Plain `<textarea>` with raw markdown content
- **Preview mode:** Rendered HTML from the markdown source via markdown-it
- **Toggle button** to switch between edit and preview

### Top Bar

- "Open Folder" button to pick a different directory
- Save status indicator ("Saved" / "Unsaved")
- Ctrl+S for manual save

## Data Flow

1. **App launch** — If a previous folder path is stored (Tauri store plugin), load it. Otherwise, prompt the user to pick a folder.
2. **Folder selected** — Rust command scans the directory, returns a list of `{filename, path}` objects to Vue.
3. **File selected** — Vue calls a Rust command with the file path, receives the markdown string, populates the textarea.
4. **Editing** — Vue tracks a dirty flag. On each keystroke, a debounce timer resets. After 2 seconds of inactivity, Vue calls a Rust write command. Ctrl+S bypasses the debounce and saves immediately.
5. **Toggle preview** — Vue renders the current textarea content client-side with markdown-it. No round-trip to Rust.
6. **Switch files** — If the current file is dirty, auto-save it before loading the new file.

## Saving Behavior

- Auto-save with ~2 second debounce after typing stops
- Ctrl+S for immediate manual save
- Visual indicator shows current save state (saved/unsaved)

## State Management

Vue's built-in `ref`/`reactive` is sufficient. No Pinia or Vuex.

Key reactive state:

- `currentFolder: string` — path to the open directory
- `files: Array<{filename, path}>` — file list from Rust
- `selectedFile: {filename, path} | null` — currently open file
- `content: string` — textarea content
- `isDirty: boolean` — unsaved changes flag
- `mode: 'edit' | 'preview'` — current view mode

## Rust Commands

- `list_files(folder: String) -> Vec<FileEntry>` — list `.md` files in folder
- `read_file(path: String) -> String` — read file content
- `write_file(path: String, content: String)` — write content to file

## Scope Boundaries

**In scope:**
- Open a folder and list its `.md` files
- Edit markdown files in a plain textarea
- Preview rendered markdown
- Auto-save and manual save
- Persist last-opened folder

**Out of scope:**
- Creating/deleting/renaming files
- Recursive folder scanning
- Syntax highlighting in the editor
- Multiple open files/tabs
- File watching for external changes
- Themes or customization
