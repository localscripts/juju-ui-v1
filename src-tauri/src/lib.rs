use std::process::{Command, Stdio, Child};
use std::io::{Write, BufRead, BufReader, Read};
use std::sync::Mutex;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::command;
use tauri::Manager;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Tab {
    id: i64,
    name: String,
    content: String,
    path: Option<String>,
}

struct AppState {
    tabs: Mutex<Vec<Tab>>,
}

struct Lsp {
    proc: Option<Child>,
    id: i32,
}

#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
fn lsp_notif(method: String, params: String, state: tauri::State<Mutex<Lsp>>) -> Result<(), String> {
    let mut lsp = state.lock().unwrap();
    let proc = lsp.proc.as_mut().ok_or("LSP not running")?;
    
    let notif = format!(r#"{{"jsonrpc":"2.0","method":"{}","params":{}}}"#, method, params);
    let msg = format!("Content-Length: {}\r\n\r\n{}", notif.len(), notif);
    
    proc.stdin.as_mut().ok_or("No stdin")?.write_all(msg.as_bytes()).map_err(|e| e.to_string())?;
    proc.stdin.as_mut().unwrap().flush().ok();
    
    Ok(())
}

fn get_lsp_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .resolve("luau-lsp", tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve sidecar: {}", e))
}

#[command]
fn lsp_req(method: String, params: String, state: tauri::State<Mutex<Lsp>>, app: tauri::AppHandle) -> Result<String, String> {
    let mut lsp = state.lock().unwrap();
    
    if lsp.proc.is_none() {
        let path = get_lsp_path(&app)?;
        let child = Command::new(&path)
            .arg("lsp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed starting LSP: {}", e))?;
        
        lsp.proc = Some(child);
    }

    lsp.id += 1;
    let id = lsp.id;
    let proc = lsp.proc.as_mut().ok_or("LSP not running")?;

    let req = format!(r#"{{"jsonrpc":"2.0","id":{},"method":"{}","params":{}}}"#, id, method, params);
    let msg = format!("Content-Length: {}\r\n\r\n{}", req.len(), req);
    
    proc.stdin.as_mut().ok_or("No stdin")?.write_all(msg.as_bytes()).map_err(|e| e.to_string())?;
    proc.stdin.as_mut().unwrap().flush().ok();

    let mut reader = BufReader::new(proc.stdout.as_mut().ok_or("No stdout")?);
    
    loop {
        let mut header = String::new();
        reader.read_line(&mut header).map_err(|e| e.to_string())?;
        
        if header.trim().is_empty() { continue; }
        
        let len: usize = header.trim()
            .strip_prefix("Content-Length: ")
            .ok_or("Invalid header")?
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;

        let mut empty = String::new();
        reader.read_line(&mut empty).ok();

        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf).map_err(|e| e.to_string())?;
        
        let res = String::from_utf8_lossy(&buf).to_string();
        
        if res.contains(&format!(r#""id":{}"#, id)) {
            return Ok(res);
        }
    }
}

#[derive(serde::Serialize)]
struct FileItem {
    name: String,
    is_folder: bool,
    path: String,
    is_root: bool,
}

fn get_app_dir() -> Result<PathBuf, String> {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .ok_or("Can't find app dir".to_string())
}

#[command]
fn get_folders() -> Result<Vec<FileItem>, String> {
    let dir = get_app_dir()?;
    let mut folders = Vec::new();
    
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        
        if path.is_dir() {
            folders.push(FileItem {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
                is_folder: true,
                path: path.to_string_lossy().to_string(),
                is_root: true,
            });
        }
    }
    
    folders.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(folders)
}

#[command]
fn get_items(path: String) -> Result<Vec<FileItem>, String> {
    let mut items = Vec::new();
    
    for entry in fs::read_dir(&path).map_err(|e| e.to_string())? {
        let p = entry.map_err(|e| e.to_string())?.path();
        
        if p.is_dir() {
            items.push(FileItem {
                name: p.file_name().unwrap().to_string_lossy().to_string(),
                is_folder: true,
                path: p.to_string_lossy().to_string(),
                is_root: false,
            });
        } else if let Some(ext) = p.extension() {
            if ext == "lua" || ext == "txt" {
                items.push(FileItem {
                    name: p.file_name().unwrap().to_string_lossy().to_string(),
                    is_folder: false,
                    path: p.to_string_lossy().to_string(),
                    is_root: false,
                });
            }
        }
    }
    
    Ok(items)
}

#[command]
fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[command]
fn open_dir() -> Result<(), String> {
    let dir = get_app_dir()?;
    
    #[cfg(target_os = "windows")]
    Command::new("explorer").arg(&dir).spawn().map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "macos")]
    Command::new("open").arg(&dir).spawn().map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(&dir).spawn().map_err(|e| e.to_string())?;
    
    Ok(())
}

#[command]
fn create_file(path: String, name: String) -> Result<String, String> {
    let file_path = Path::new(&path).join(&name);
    fs::write(&file_path, "").map_err(|e| e.to_string())?;
    Ok(file_path.to_string_lossy().to_string())
}

#[command]
fn rename_item(old_path: String, new_name: String) -> Result<String, String> {
    let old = Path::new(&old_path);
    let new = old.parent().unwrap().join(&new_name);
    fs::rename(old, &new).map_err(|e| e.to_string())?;
    Ok(new.to_string_lossy().to_string())
}

#[command]
fn delete_item(path: String) -> Result<(), String> {
    let p = Path::new(&path);
    if p.is_dir() {
        fs::remove_dir_all(p).map_err(|e| e.to_string())
    } else {
        fs::remove_file(p).map_err(|e| e.to_string())
    }
}

#[command]
fn save_tabs(tabs: Vec<Tab>, state: tauri::State<AppState>) -> Result<(), String> {
    let mut app_tabs = state.tabs.lock().unwrap();
    *app_tabs = tabs;
    Ok(())
}

#[command]
fn load_tabs(state: tauri::State<AppState>) -> Result<Vec<Tab>, String> {
    let tabs = state.tabs.lock().unwrap();
    Ok(tabs.clone())
}

#[command]
fn search_files(query: String) -> Result<Vec<FileItem>, String> {
    let dir = get_app_dir()?;
    let q = query.to_lowercase();
    let mut results = Vec::new();
    
    fn scan(path: &Path, q: &str, results: &mut Vec<FileItem>) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                let name = p.file_name().unwrap().to_string_lossy().to_string();
                
                if p.is_dir() {
                    if name.to_lowercase().contains(q) {
                        results.push(FileItem {
                            name: name.clone(),
                            is_folder: true,
                            path: p.to_string_lossy().to_string(),
                            is_root: false,
                        });
                    }
                    scan(&p, q, results);
                } else if let Some(ext) = p.extension() {
                    if (ext == "lua" || ext == "txt") && name.to_lowercase().contains(q) {
                        results.push(FileItem {
                            name,
                            is_folder: false,
                            path: p.to_string_lossy().to_string(),
                            is_root: false,
                        });
                    }
                }
            }
        }
    }
    
    scan(&dir, &q, &mut results);
    Ok(results)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if let Some(scripts) = get_app_dir().ok().map(|d| d.join("Scripts")) {
        fs::create_dir_all(scripts).ok();
    }
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(Lsp { proc: None, id: 0 }))
        .manage(AppState {
            tabs: Mutex::new(vec![Tab {
                id: 1,
                name: "Infinite Yield".to_string(),
                content: "-- idk".to_string(),
                path: None,
            }]),
        })
        .invoke_handler(tauri::generate_handler![
            greet, lsp_req, lsp_notif, get_folders, get_items, read_file, search_files, open_dir,
            create_file, rename_item, delete_item, save_tabs, load_tabs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}