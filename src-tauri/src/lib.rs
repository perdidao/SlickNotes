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

#[tauri::command]
fn create_file(folder: String, filename: String) -> Result<FileEntry, String> {
    let file_path = Path::new(&folder).join(format!("{}.md", filename));
    if file_path.exists() {
        return Err(format!("{}.md already exists", filename));
    }
    fs::write(&file_path, "").map_err(|e| e.to_string())?;
    Ok(FileEntry {
        filename,
        path: file_path.to_string_lossy().to_string(),
    })
}

#[tauri::command]
fn rename_file(path: String, new_name: String) -> Result<FileEntry, String> {
    let old_path = Path::new(&path);
    let parent = old_path.parent().ok_or("Invalid file path")?;
    let new_path = parent.join(format!("{}.md", new_name));
    if new_path.exists() {
        return Err(format!("{}.md already exists", new_name));
    }
    fs::rename(old_path, &new_path).map_err(|e| e.to_string())?;
    Ok(FileEntry {
        filename: new_name,
        path: new_path.to_string_lossy().to_string(),
    })
}

#[tauri::command]
fn delete_file(path: String) -> Result<(), String> {
    fs::remove_file(&path).map_err(|e| format!("Failed to delete {}: {}", path, e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![list_files, read_file, write_file, create_file, rename_file, delete_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
