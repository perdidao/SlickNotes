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
