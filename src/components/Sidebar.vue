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
