# SlickNotes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a lightweight Tauri v2 + Vue 3 markdown editor that reads `.md` files from a folder, lists them in a sidebar, and lets the user edit or preview them.

**Architecture:** Single Tauri v2 window. Rust backend exposes three commands (list_files, read_file, write_file). Vue 3 frontend with sidebar + editor/preview toggle. Auto-save with debounce + Ctrl+S manual save.

**Tech Stack:** Tauri v2, Vue 3 (Composition API), Vite, markdown-it, Rust std::fs, Tauri dialog plugin, Tauri store plugin

---

## File Structure

```
slicknotes/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json
│   └── src/
│       ├── lib.rs          # Tauri commands (list_files, read_file, write_file)
│       └── main.rs         # Tauri main entry point
├── src/
│   ├── App.vue             # Root component: layout with sidebar + main area
│   ├── main.js             # Vue app entry point
│   ├── style.css           # Global styles
│   ├── components/
│   │   ├── Sidebar.vue     # File list sidebar
│   │   ├── Editor.vue      # Textarea editor with save logic
│   │   └── Preview.vue     # Rendered markdown preview
│   └── composables/
│       └── useFiles.js     # Shared state & file operations (Tauri invoke wrappers)
├── index.html
├── package.json
└── vite.config.js
```

---

### Task 0: Environment Setup

**Files:**
- None (system setup)

- [ ] **Step 1: Install Rust toolchain**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

- [ ] **Step 2: Install missing Tauri v2 system dependencies**

```bash
sudo apt install -y libwebkit2gtk-4.1-dev libgtk-3-dev libappindicator3-dev
```

- [ ] **Step 3: Verify installation**

```bash
rustc --version
cargo --version
```

Expected: Version numbers printed for both.

- [ ] **Step 4: Commit (no commit — system setup only)**

---

### Task 1: Scaffold Tauri + Vue Project

**Files:**
- Create: entire `slicknotes/` project scaffold

- [ ] **Step 1: Create Tauri v2 project with Vue frontend**

```bash
cd /home/perd1dao/projects/personal/slicknotes
pnpm create tauri-app . --template vue --manager pnpm --yes
```

If the scaffold tool asks questions interactively, use these values:
- Package manager: pnpm
- Frontend: Vue
- Template: Vue (not Vue-TS)

- [ ] **Step 2: Install dependencies**

```bash
cd /home/perd1dao/projects/personal/slicknotes
pnpm install
```

- [ ] **Step 3: Add Tauri plugins for dialog and store**

```bash
cd /home/perd1dao/projects/personal/slicknotes
pnpm add @tauri-apps/plugin-dialog @tauri-apps/plugin-store
cd src-tauri
cargo add tauri-plugin-dialog tauri-plugin-store
```

- [ ] **Step 4: Add markdown-it**

```bash
cd /home/perd1dao/projects/personal/slicknotes
pnpm add markdown-it
```

- [ ] **Step 5: Initialize git and commit**

```bash
cd /home/perd1dao/projects/personal/slicknotes
git init
git add .
git commit -m "chore: scaffold Tauri v2 + Vue 3 project with plugins"
```

- [ ] **Step 6: Verify the app builds and opens**

```bash
cd /home/perd1dao/projects/personal/slicknotes
pnpm tauri dev
```

Expected: A Tauri window opens showing the default Vue template page. Close it after confirming.

---

### Task 2: Rust Backend — File Commands

**Files:**
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/main.rs` (if needed)
- Modify: `src-tauri/tauri.conf.json` (capabilities)
- Modify: `src-tauri/capabilities/default.json`

- [ ] **Step 1: Write the three Tauri commands in `lib.rs`**

Replace the contents of `src-tauri/src/lib.rs` with:

```rust
use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
pub struct FileEntry {
    pub filename: String,
    pub path: String,
}

#[tauri::command]
fn list_files(folder: String) -> Result<Vec<FileEntry>, String> {
    let path = Path::new(&folder);
    if !path.is_dir() {
        return Err(format!("{} is not a directory", folder));
    }

    let mut files: Vec<FileEntry> = Vec::new();
    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_path = entry.path();
        if file_path.is_file() {
            if let Some(ext) = file_path.extension() {
                if ext == "md" {
                    let filename = file_path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let path_str = file_path.to_string_lossy().to_string();
                    files.push(FileEntry {
                        filename,
                        path: path_str,
                    });
                }
            }
        }
    }

    files.sort_by(|a, b| a.filename.to_lowercase().cmp(&b.filename.to_lowercase()));
    Ok(files)
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("Failed to read {}: {}", path, e))
}

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, &content).map_err(|e| format!("Failed to write {}: {}", path, e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![list_files, read_file, write_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 2: Update capabilities to allow dialog and store**

Edit `src-tauri/capabilities/default.json`. Add the dialog and store permissions to the `permissions` array:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "dialog:default",
    "dialog:allow-open",
    "store:default"
  ]
}
```

- [ ] **Step 3: Verify it compiles**

```bash
cd /home/perd1dao/projects/personal/slicknotes/src-tauri
cargo build
```

Expected: Compiles without errors.

- [ ] **Step 4: Commit**

```bash
cd /home/perd1dao/projects/personal/slicknotes
git add src-tauri/
git commit -m "feat: add Rust commands for list_files, read_file, write_file"
```

---

### Task 3: Composable — useFiles

**Files:**
- Create: `src/composables/useFiles.js`

- [ ] **Step 1: Create the composable**

Create `src/composables/useFiles.js`:

```javascript
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { load } from '@tauri-apps/plugin-store'

const currentFolder = ref(null)
const files = ref([])
const selectedFile = ref(null)
const content = ref('')
const isDirty = ref(false)
const mode = ref('edit')

let saveTimeout = null
let store = null

async function getStore() {
  if (!store) {
    store = await load('settings.json', { autoSave: true })
  }
  return store
}

async function loadLastFolder() {
  const s = await getStore()
  const folder = await s.get('lastFolder')
  if (folder) {
    await openFolder(folder)
  }
}

async function pickFolder() {
  const selected = await open({ directory: true })
  if (selected) {
    await openFolder(selected)
  }
}

async function openFolder(folder) {
  currentFolder.value = folder
  const s = await getStore()
  await s.set('lastFolder', folder)
  files.value = await invoke('list_files', { folder })
  selectedFile.value = null
  content.value = ''
  isDirty.value = false
  mode.value = 'edit'
}

async function selectFile(file) {
  if (isDirty.value && selectedFile.value) {
    await saveFile()
  }
  selectedFile.value = file
  content.value = await invoke('read_file', { path: file.path })
  isDirty.value = false
  mode.value = 'edit'
}

async function saveFile() {
  if (!selectedFile.value) return
  await invoke('write_file', {
    path: selectedFile.value.path,
    content: content.value,
  })
  isDirty.value = false
}

function onContentChange(newContent) {
  content.value = newContent
  isDirty.value = true
  clearTimeout(saveTimeout)
  saveTimeout = setTimeout(() => {
    saveFile()
  }, 2000)
}

async function saveNow() {
  clearTimeout(saveTimeout)
  await saveFile()
}

function toggleMode() {
  mode.value = mode.value === 'edit' ? 'preview' : 'edit'
}

export function useFiles() {
  return {
    currentFolder,
    files,
    selectedFile,
    content,
    isDirty,
    mode,
    loadLastFolder,
    pickFolder,
    selectFile,
    onContentChange,
    saveNow,
    toggleMode,
  }
}
```

- [ ] **Step 2: Commit**

```bash
cd /home/perd1dao/projects/personal/slicknotes
git add src/composables/
git commit -m "feat: add useFiles composable for file state and operations"
```

---

### Task 4: Sidebar Component

**Files:**
- Create: `src/components/Sidebar.vue`

- [ ] **Step 1: Create the sidebar component**

Create `src/components/Sidebar.vue`:

```vue
<script setup>
import { useFiles } from '../composables/useFiles.js'

const { files, selectedFile, selectFile } = useFiles()
</script>

<template>
  <aside class="sidebar">
    <ul>
      <li
        v-for="file in files"
        :key="file.path"
        :class="{ active: selectedFile?.path === file.path }"
        @click="selectFile(file)"
      >
        {{ file.filename }}
      </li>
    </ul>
    <p v-if="files.length === 0" class="empty">No .md files found</p>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 220px;
  min-width: 220px;
  border-right: 1px solid #ddd;
  overflow-y: auto;
  background: #f7f7f7;
}

.sidebar ul {
  list-style: none;
  margin: 0;
  padding: 0;
}

.sidebar li {
  padding: 10px 14px;
  cursor: pointer;
  border-bottom: 1px solid #eee;
  font-size: 14px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sidebar li:hover {
  background: #e9e9e9;
}

.sidebar li.active {
  background: #d0d0ff;
  font-weight: 600;
}

.empty {
  padding: 14px;
  color: #999;
  font-size: 13px;
}
</style>
```

- [ ] **Step 2: Commit**

```bash
cd /home/perd1dao/projects/personal/slicknotes
git add src/components/Sidebar.vue
git commit -m "feat: add Sidebar component for file list"
```

---

### Task 5: Editor Component

**Files:**
- Create: `src/components/Editor.vue`

- [ ] **Step 1: Create the editor component**

Create `src/components/Editor.vue`:

```vue
<script setup>
import { useFiles } from '../composables/useFiles.js'
import { onMounted, onUnmounted } from 'vue'

const { content, onContentChange, saveNow } = useFiles()

function handleInput(event) {
  onContentChange(event.target.value)
}

function handleKeydown(event) {
  if ((event.ctrlKey || event.metaKey) && event.key === 's') {
    event.preventDefault()
    saveNow()
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div class="editor">
    <textarea
      :value="content"
      @input="handleInput"
      placeholder="Select a file to start editing..."
      spellcheck="false"
    ></textarea>
  </div>
</template>

<style scoped>
.editor {
  flex: 1;
  display: flex;
}

.editor textarea {
  flex: 1;
  border: none;
  outline: none;
  resize: none;
  padding: 16px;
  font-family: monospace;
  font-size: 14px;
  line-height: 1.6;
  background: #fff;
}
</style>
```

- [ ] **Step 2: Commit**

```bash
cd /home/perd1dao/projects/personal/slicknotes
git add src/components/Editor.vue
git commit -m "feat: add Editor component with auto-save and Ctrl+S"
```

---

### Task 6: Preview Component

**Files:**
- Create: `src/components/Preview.vue`

- [ ] **Step 1: Create the preview component**

Create `src/components/Preview.vue`:

```vue
<script setup>
import { computed } from 'vue'
import MarkdownIt from 'markdown-it'
import { useFiles } from '../composables/useFiles.js'

const md = new MarkdownIt()
const { content } = useFiles()

const rendered = computed(() => md.render(content.value || ''))
</script>

<template>
  <div class="preview" v-html="rendered"></div>
</template>

<style scoped>
.preview {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  font-size: 15px;
  line-height: 1.7;
}

.preview :deep(h1) { font-size: 1.8em; margin: 0.5em 0; }
.preview :deep(h2) { font-size: 1.4em; margin: 0.5em 0; }
.preview :deep(h3) { font-size: 1.2em; margin: 0.5em 0; }
.preview :deep(code) {
  background: #f0f0f0;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.9em;
}
.preview :deep(pre) {
  background: #f0f0f0;
  padding: 12px;
  border-radius: 4px;
  overflow-x: auto;
}
.preview :deep(blockquote) {
  border-left: 3px solid #ccc;
  margin: 0;
  padding-left: 12px;
  color: #666;
}
</style>
```

- [ ] **Step 2: Commit**

```bash
cd /home/perd1dao/projects/personal/slicknotes
git add src/components/Preview.vue
git commit -m "feat: add Preview component with markdown-it rendering"
```

---

### Task 7: App.vue — Wire Everything Together

**Files:**
- Modify: `src/App.vue`
- Modify: `src/style.css`
- Modify: `src/main.js` (if needed)

- [ ] **Step 1: Replace App.vue with the full layout**

Replace the contents of `src/App.vue` with:

```vue
<script setup>
import { onMounted } from 'vue'
import { useFiles } from './composables/useFiles.js'
import Sidebar from './components/Sidebar.vue'
import Editor from './components/Editor.vue'
import Preview from './components/Preview.vue'

const {
  currentFolder,
  selectedFile,
  isDirty,
  mode,
  loadLastFolder,
  pickFolder,
  toggleMode,
} = useFiles()

onMounted(() => {
  loadLastFolder()
})
</script>

<template>
  <div class="app">
    <header class="topbar">
      <button @click="pickFolder">Open Folder</button>
      <span class="folder-path" v-if="currentFolder">{{ currentFolder }}</span>
      <div class="spacer"></div>
      <span v-if="selectedFile" class="save-status" :class="{ dirty: isDirty }">
        {{ isDirty ? 'Unsaved' : 'Saved' }}
      </span>
      <button v-if="selectedFile" @click="toggleMode">
        {{ mode === 'edit' ? 'Preview' : 'Edit' }}
      </button>
    </header>
    <div class="body">
      <Sidebar />
      <main class="content">
        <Editor v-if="mode === 'edit' && selectedFile" />
        <Preview v-else-if="mode === 'preview' && selectedFile" />
        <div v-else class="placeholder">
          <p v-if="!currentFolder">Open a folder to get started.</p>
          <p v-else>Select a file from the sidebar.</p>
        </div>
      </main>
    </div>
  </div>
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.topbar {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  border-bottom: 1px solid #ddd;
  background: #fafafa;
}

.topbar button {
  padding: 5px 12px;
  border: 1px solid #ccc;
  border-radius: 4px;
  background: #fff;
  cursor: pointer;
  font-size: 13px;
}

.topbar button:hover {
  background: #eee;
}

.folder-path {
  font-size: 12px;
  color: #777;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 400px;
}

.spacer {
  flex: 1;
}

.save-status {
  font-size: 12px;
  color: #4a4;
}

.save-status.dirty {
  color: #c44;
}

.body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.content {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.placeholder {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #999;
  font-size: 15px;
}
</style>
```

- [ ] **Step 2: Replace global styles**

Replace the contents of `src/style.css` with:

```css
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #fff;
}
```

- [ ] **Step 3: Clean up main.js**

Ensure `src/main.js` contains only:

```javascript
import { createApp } from 'vue'
import App from './App.vue'
import './style.css'

createApp(App).mount('#app')
```

- [ ] **Step 4: Clean up index.html**

Ensure `index.html` body contains only:

```html
<div id="app"></div>
<script type="module" src="/src/main.js"></script>
```

- [ ] **Step 5: Remove any scaffold placeholder files**

Delete any leftover scaffold components (e.g., `src/components/Greet.vue`, `src/assets/` if unused).

```bash
rm -f src/components/Greet.vue
rm -rf src/assets/
```

- [ ] **Step 6: Verify the app runs**

```bash
cd /home/perd1dao/projects/personal/slicknotes
pnpm tauri dev
```

Expected: App opens with the top bar (Open Folder button), empty sidebar, and "Open a folder to get started" placeholder. Clicking Open Folder opens a native directory picker. Selecting a folder with `.md` files populates the sidebar. Selecting a file loads it in the editor. Toggle switches to preview. Auto-save and Ctrl+S both work.

- [ ] **Step 7: Commit**

```bash
cd /home/perd1dao/projects/personal/slicknotes
git add .
git commit -m "feat: wire up App.vue with sidebar, editor, preview, and top bar"
```

---

### Task 8: Final Polish & Tauri Config

**Files:**
- Modify: `src-tauri/tauri.conf.json`

- [ ] **Step 1: Update Tauri window config**

In `src-tauri/tauri.conf.json`, update the app metadata and window settings:

- Set `productName` to `"SlickNotes"`
- Set `identifier` to `"com.slicknotes.app"`
- Set the default window `title` to `"SlickNotes"`
- Set default window `width` to `1000` and `height` to `700`

These fields are in the Tauri v2 config structure — find them in the existing generated config and update their values.

- [ ] **Step 2: Verify final build**

```bash
cd /home/perd1dao/projects/personal/slicknotes
pnpm tauri dev
```

Expected: Window opens titled "SlickNotes" at a comfortable size. All features work end to end.

- [ ] **Step 3: Commit**

```bash
cd /home/perd1dao/projects/personal/slicknotes
git add .
git commit -m "chore: configure app metadata and window defaults"
```
