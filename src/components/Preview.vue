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
