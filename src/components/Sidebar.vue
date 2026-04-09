<script setup>
import { ref } from 'vue'
import { useFiles } from '../composables/useFiles.js'

const { files, selectedFile, selectFile, createFile, renameFile, deleteFile } = useFiles()

const showNewInput = ref(false)
const newFileName = ref('')
const editingFile = ref(null)
const editingName = ref('')
const contextMenu = ref({ show: false, x: 0, y: 0, file: null })

function startCreate() {
  showNewInput.value = true
  newFileName.value = ''
}

async function confirmCreate() {
  const name = newFileName.value.trim()
  if (!name) {
    showNewInput.value = false
    return
  }
  try {
    await createFile(name)
  } catch (e) {
    alert(e)
  }
  showNewInput.value = false
}

function cancelCreate() {
  showNewInput.value = false
}

function onContextMenu(event, file) {
  event.preventDefault()
  contextMenu.value = { show: true, x: event.clientX, y: event.clientY, file }
}

function closeContextMenu() {
  contextMenu.value = { show: false, x: 0, y: 0, file: null }
}

function startRename() {
  editingFile.value = contextMenu.value.file
  editingName.value = contextMenu.value.file.filename
  closeContextMenu()
}

async function confirmRename() {
  const name = editingName.value.trim()
  if (!name || name === editingFile.value.filename) {
    editingFile.value = null
    return
  }
  try {
    await renameFile(editingFile.value, name)
  } catch (e) {
    alert(e)
  }
  editingFile.value = null
}

function cancelRename() {
  editingFile.value = null
}

async function doDelete() {
  const file = contextMenu.value.file
  closeContextMenu()
  if (confirm(`Delete "${file.filename}.md"?`)) {
    try {
      await deleteFile(file)
    } catch (e) {
      alert(e)
    }
  }
}
</script>

<template>
  <aside class="sidebar" @click="closeContextMenu">
    <ul>
      <li
        v-for="file in files"
        :key="file.path"
        :class="{ active: selectedFile?.path === file.path }"
        @click="selectFile(file)"
        @contextmenu="onContextMenu($event, file)"
      >
        <template v-if="editingFile?.path === file.path">
          <input
            v-model="editingName"
            class="rename-input"
            @keydown.enter="confirmRename"
            @keydown.escape="cancelRename"
            @blur="confirmRename"
            @click.stop
            ref="renameInput"
            autofocus
          />
        </template>
        <template v-else>
          {{ file.filename }}
        </template>
      </li>
      <li v-if="showNewInput" class="new-input-row">
        <input
          v-model="newFileName"
          class="new-input"
          placeholder="Note name..."
          @keydown.enter="confirmCreate"
          @keydown.escape="cancelCreate"
          @blur="confirmCreate"
          autofocus
        />
      </li>
    </ul>
    <p v-if="files.length === 0 && !showNewInput" class="empty">No .md files found</p>
    <button class="new-btn" @click="startCreate">+ New Note</button>

    <teleport to="body">
      <div
        v-if="contextMenu.show"
        class="context-menu"
        :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
      >
        <div class="context-item" @click="startRename">Rename</div>
        <div class="context-item danger" @click="doDelete">Delete</div>
      </div>
    </teleport>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 220px;
  min-width: 220px;
  border-right: 1px solid var(--border);
  overflow-y: auto;
  background: var(--bg-secondary);
  display: flex;
  flex-direction: column;
}

.sidebar ul {
  list-style: none;
  margin: 0;
  padding: 0;
  flex: 1;
}

.sidebar li {
  padding: 10px 14px;
  cursor: pointer;
  border-bottom: 1px solid var(--border);
  font-size: 14px;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sidebar li:hover {
  background: var(--bg-hover);
}

.sidebar li.active {
  background: var(--bg-active);
  color: var(--accent);
  font-weight: 600;
}

.new-input-row {
  cursor: default !important;
}

.new-input,
.rename-input {
  width: 100%;
  background: var(--bg-primary);
  border: 1px solid var(--accent);
  border-radius: 3px;
  color: var(--text-primary);
  font-size: 13px;
  padding: 4px 6px;
  outline: none;
}

.new-btn {
  padding: 10px 14px;
  border: none;
  border-top: 1px solid var(--border);
  background: var(--bg-surface);
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  text-align: left;
}

.new-btn:hover {
  background: var(--bg-hover);
  color: var(--accent);
}

.empty {
  padding: 14px;
  color: var(--text-muted);
  font-size: 13px;
}
</style>

<style>
.context-menu {
  position: fixed;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 4px 0;
  min-width: 120px;
  z-index: 1000;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
}

.context-item {
  padding: 8px 14px;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: pointer;
}

.context-item:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.context-item.danger:hover {
  background: var(--red);
  color: #fff;
}
</style>
