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
  color: var(--text-primary);
}

.preview :deep(h1), .preview :deep(h2), .preview :deep(h3) {
  color: var(--accent);
}

.preview :deep(h1) { font-size: 2em; margin: 0.25em 0 0.5em; color: var(--text-primary); }
.preview :deep(h2) { font-size: 1.6em; margin: 0.25em 0 0.5em; }
.preview :deep(h3) { font-size: 1.15em; margin: 0.25em 0 0.5em; }
.preview :deep(a) { color: var(--accent); }
.preview :deep(code) {
  background: var(--code-bg);
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.9em;
}
.preview :deep(pre) {
  background: var(--code-bg);
  padding: 12px;
  border-radius: 4px;
  overflow-x: auto;
}
.preview :deep(blockquote) {
  border-left: 3px solid var(--bg-active);
  margin: 0;
  padding-left: 12px;
  color: var(--text-secondary);
}
.preview :deep(img) {
  max-width: 100%;
  border-radius: 4px;
  margin: 12px 0;
}
.preview :deep(ul), .preview :deep(ol) {
  margin: 8px 0 24px;
  padding-left: 20px;
}
.preview :deep(li) {
  margin: 4px 0;
}
.preview :deep(hr) {
  border: none;
  border-top: 1px solid var(--bg-active);
  margin: 24px 0;
}
.preview :deep(table) {
  width: 100%;
  border-collapse: collapse;
  margin: 12px 0;
}
.preview :deep(th), .preview :deep(td) {
  border: 1px solid var(--bg-active);
  padding: 8px;
  text-align: left;
}
.preview :deep(th) {
  background: var(--bg-active);
}
.preview :deep(p) {
  margin: 8px 0 24px;
}
</style>
