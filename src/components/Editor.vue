<script setup>
import { ref, watch, onMounted, onUnmounted, shallowRef } from 'vue'
import { useFiles } from '../composables/useFiles.js'
import { EditorView, keymap, placeholder } from '@codemirror/view'
import { EditorState } from '@codemirror/state'
import { markdown, markdownLanguage } from '@codemirror/lang-markdown'
import { languages } from '@codemirror/language-data'
import { HighlightStyle, syntaxHighlighting, bracketMatching } from '@codemirror/language'
import { tags } from '@lezer/highlight'
import {
  defaultKeymap,
  indentWithTab,
  history,
  historyKeymap,
} from '@codemirror/commands'
import {
  closeBrackets,
  closeBracketsKeymap,
} from '@codemirror/autocomplete'

const { content, onContentChange, saveNow } = useFiles()
const editorRef = ref(null)
const view = shallowRef(null)

const editorTheme = EditorView.theme({
  '&': {
    flex: '1',
    fontSize: '15px',
    backgroundColor: 'var(--bg-primary)',
    color: 'var(--text-primary)',
  },
  '.cm-content': {
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    padding: '16px 0',
    lineHeight: '1.7',
    caretColor: 'var(--cursor)',
  },
  '&.cm-focused .cm-cursor': {
    borderLeftColor: 'var(--cursor)',
  },
  '&.cm-focused .cm-selectionBackground, .cm-selectionBackground': {
    backgroundColor: 'var(--selection)',
  },
  '.cm-activeLine': {
    backgroundColor: 'var(--bg-hover)',
  },
  '.cm-gutters': {
    display: 'none',
  },
  '.cm-scroller': {
    padding: '0 16px',
  },
}, { dark: true })

const editorHighlight = HighlightStyle.define([
  { tag: tags.heading1, color: 'var(--text-primary)', fontWeight: 'bold', fontSize: '2em' },
  { tag: tags.heading2, color: 'var(--accent)', fontWeight: 'bold', fontSize: '1.6em' },
  { tag: tags.heading3, color: 'var(--accent)', fontWeight: 'bold', fontSize: '1.15em' },
  { tag: [tags.heading4, tags.heading5, tags.heading6], color: 'var(--accent)', fontWeight: 'bold' },
  { tag: tags.strong, color: 'var(--text-primary)', fontWeight: 'bold' },
  { tag: tags.emphasis, color: 'var(--text-primary)', fontStyle: 'italic' },
  { tag: tags.link, color: 'var(--cyan)', textDecoration: 'underline' },
  { tag: tags.url, color: 'var(--cyan)' },
  { tag: [tags.processingInstruction, tags.monospace], color: 'var(--text-secondary)', backgroundColor: 'var(--code-bg)', borderRadius: '3px', padding: '1px 4px' },
  { tag: tags.quote, color: 'var(--text-secondary)', fontStyle: 'italic' },
  { tag: tags.strikethrough, textDecoration: 'line-through', color: 'var(--text-muted)' },
  { tag: [tags.meta, tags.comment], color: 'var(--text-muted)' },
  { tag: tags.contentSeparator, color: 'var(--border)' },
])

// Track whether we're updating from external content change
let updatingFromOutside = false

function createEditor() {
  if (view.value) {
    view.value.destroy()
  }

  const state = EditorState.create({
    doc: content.value || '',
    extensions: [
      history(),
      closeBrackets(),
      bracketMatching(),
      markdown({ base: markdownLanguage, codeLanguages: languages }),
      editorTheme,
      syntaxHighlighting(editorHighlight),
      EditorView.lineWrapping,
      placeholder('Select a file to start editing...'),
      keymap.of([
        ...closeBracketsKeymap,
        ...historyKeymap,
        indentWithTab,
        ...defaultKeymap,
        {
          key: 'Mod-s',
          run: () => {
            saveNow()
            return true
          },
        },
      ]),
      EditorView.updateListener.of((update) => {
        if (update.docChanged && !updatingFromOutside) {
          onContentChange(update.state.doc.toString())
        }
      }),
    ],
  })

  view.value = new EditorView({
    state,
    parent: editorRef.value,
  })
}

// When content changes externally (file switch), update the editor
watch(content, (newContent) => {
  if (!view.value) return
  const current = view.value.state.doc.toString()
  if (current !== newContent) {
    updatingFromOutside = true
    view.value.dispatch({
      changes: {
        from: 0,
        to: current.length,
        insert: newContent || '',
      },
    })
    updatingFromOutside = false
  }
})

onMounted(() => {
  createEditor()
})

onUnmounted(() => {
  if (view.value) {
    view.value.destroy()
  }
})
</script>

<template>
  <div class="editor" ref="editorRef"></div>
</template>

<style scoped>
.editor {
  flex: 1;
  display: flex;
  overflow: hidden;
}
</style>
