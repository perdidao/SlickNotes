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
