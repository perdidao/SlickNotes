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
  const selected = await open({ directory: true, defaultPath: currentFolder.value || undefined })
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
}

async function selectFile(file) {
  if (isDirty.value && selectedFile.value) {
    await saveFile()
  }
  selectedFile.value = file
  content.value = await invoke('read_file', { path: file.path })
  isDirty.value = false
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

async function refreshFiles() {
  if (!currentFolder.value) return
  files.value = await invoke('list_files', { folder: currentFolder.value })
}

async function createFile(filename) {
  if (!currentFolder.value) return
  const file = await invoke('create_file', {
    folder: currentFolder.value,
    filename,
  })
  await refreshFiles()
  await selectFile(file)
}

async function renameFile(file, newName) {
  const renamed = await invoke('rename_file', {
    path: file.path,
    newName,
  })
  await refreshFiles()
  if (selectedFile.value?.path === file.path) {
    selectedFile.value = renamed
  }
}

async function deleteFile(file) {
  await invoke('delete_file', { path: file.path })
  if (selectedFile.value?.path === file.path) {
    selectedFile.value = null
    content.value = ''
    isDirty.value = false
  }
  await refreshFiles()
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
    createFile,
    renameFile,
    deleteFile,
  }
}
