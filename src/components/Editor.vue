<script setup>
import { ref, watch, onMounted, onUnmounted, shallowRef } from 'vue'
import { useFiles } from '../composables/useFiles.js'
import { EditorView, keymap, placeholder } from '@codemirror/view'
import { EditorState } from '@codemirror/state'
import { markdown, markdownLanguage } from '@codemirror/lang-markdown'
import { languages } from '@codemirror/language-data'
import { oneDark } from '@codemirror/theme-one-dark'
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
import {
  syntaxHighlighting,
  defaultHighlightStyle,
  bracketMatching,
} from '@codemirror/language'

const { content, onContentChange, saveNow } = useFiles()
const editorRef = ref(null)
const view = shallowRef(null)

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
      syntaxHighlighting(defaultHighlightStyle),
      oneDark,
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
      EditorView.theme({
        '&': {
          flex: '1',
          fontSize: '14px',
        },
        '.cm-content': {
          fontFamily: 'monospace',
          padding: '16px 0',
          lineHeight: '1.6',
        },
        '.cm-gutters': {
          display: 'none',
        },
        '.cm-scroller': {
          padding: '0 16px',
        },
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
