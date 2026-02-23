use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use tauri::command;
use tauri::Manager;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct Tab { id: i64, name: String, content: String, path: Option<String> }

#[derive(serde::Serialize)]
struct Item { name: String, is_folder: bool, path: String, is_root: bool }

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct ThemeMeta { name: String, author: String }

struct State {
    tabs: Mutex<Vec<Tab>>,
    active_theme: Mutex<Option<String>>,
}

fn app_dir() -> PathBuf {
    std::env::current_exe().unwrap().parent().unwrap().to_path_buf()
}

fn themes_dir() -> PathBuf {
    app_dir().join("Themes")
}

fn scripts_dir() -> PathBuf {
    app_dir().join("Scripts")
}

fn required_keys() -> Vec<&'static str> {
    vec![
        "--bg-app","--loader","--scrollbar-thumb","--badge-bg","--badge-text",
        "--topbar-bg","--topbar-icon","--topbar-menu-bg","--topbar-menu-text",
        "--topbar-menu-hover-bg","--topbar-btn-bg","--topbar-btn-hover",
        "--leftbar-bg","--leftbar-btn-hover","--leftbar-btn-active-from",
        "--leftbar-btn-active-to","--icon-leftbar-selected-top",
        "--icon-leftbar-selected-bottom","--icon-leftbar-unselected","--uninjected",
        "--tab-bg","--tab-bg-selected","--tab-bg-unselected","--tab-hover",
        "--tab-text-selected","--tab-text-unselected","--tab-icon",
        "--tab-icon-selected","--tab-remove-icon",
        "--editor-toolbar-bg","--editor-execute-bg","--editor-execute-bg-hover",
        "--editor-execute-text","--editor-execute-icon","--editor-inject-bg",
        "--editor-inject-bg-hover","--editor-inject-text","--editor-inject-icon",
        "--editor-clear-bg","--editor-clear-bg-hover","--editor-clear-icon",
        "--editor-canvas-bg","--editor-tabbar-bg","--editor-newtab-bg",
        "--editor-newtab-bg-hover","--editor-newtab-icon",
        "--monaco-bg","--monaco-keyword","--monaco-string","--monaco-number",
        "--monaco-comment","--monaco-identifier","--monaco-delimiter",
        "--monaco-line-number","--monaco-line-number-active","--monaco-selection",
        "--monaco-scrollbar","--monaco-scrollbar-hover",
        "--explorer-bg","--explorer-label","--explorer-result-path",
        "--explorer-tree-line","--folder-expanded-bg",
        "--scriptitem-bg","--scriptitem-bg-hover","--scriptitem-bg-droptarget",
        "--scriptitem-text","--scriptitem-text-droptarget","--scriptitem-icon",
        "--scriptitem-input-bg","--scriptitem-input-text",
        "--scriptitem-ext-btn-bg","--scriptitem-ext-btn-bg-hover","--scriptitem-ext-btn-text",
        "--open-bg","--open-hover","--open-icon","--open-text",
        "--save-bg","--save-hover","--save-icon","--save-text",
        "--search-input-bg","--search-input-bg-hover","--search-input-text",
        "--search-input-icon","--search-input-placeholder",
        "--search-opendir-bg","--search-opendir-bg-hover","--search-opendir-icon",
        "--settings-sidebar-bg","--settings-sidebar-label","--settings-sidebar-item-hover",
        "--settings-content-bg","--settings-btn-unselected",
        "--settings-btn-selected-from","--settings-btn-selected-to",
        "--settings-icon-unselected","--settings-icon-selected-top",
        "--settings-icon-selected-bottom","--settings-text-unselected",
        "--settings-text-selected-top","--settings-text-selected-bottom",
        "--interface-title","--interface-desc","--core-title","--core-desc",
        "--editor-title","--editor-desc","--themes-title","--themes-desc",
        "--search-bg","--search-hover","--search-text","--search-icon",
        "--search-filter-btn-bg","--search-filter-btn-hover","--search-filter-btn-icon",
        "--item-bg","--item-icon","--item-title","--item-desc","--item-desc-icon",
        "--item-check-off","--item-check-hover","--item-check-on","--item-check-icon",
        "--item-btn-bg","--item-btn-hover","--item-btn-text",
        "--item-dropdown-btn-bg","--item-dropdown-btn-hover","--item-dropdown-btn-text",
        "--item-textbox-bg","--item-textbox-bg-hover","--item-textbox-bg-focus",
        "--item-textbox-text","--item-textbox-placeholder",
        "--item-slider-track","--item-slider-fill","--item-slider-thumb","--item-slider-value",
        "--theme-item-bg","--theme-item-icon","--theme-item-title","--theme-item-hex",
        "--theme-item-label","--theme-item-swatch-outline",
        "--theme-section-label","--theme-section-divider",
        "--theme-toolbar-btn-bg","--theme-toolbar-btn-hover","--theme-toolbar-btn-text",
        "--menu-bg","--menu-item-text","--menu-item-hover","--menu-item-selected-bg",
        "--menu-item-selected-text","--menu-item-selected-icon","--menu-separator","--menu-item-disabled",
        "--ctx-bg","--ctx-item-text","--ctx-item-hover","--ctx-item-disabled",
        "--prompt-overlay","--prompt-bg","--prompt-title","--prompt-message",
        "--prompt-input-bg","--prompt-input-bg-hover","--prompt-input-text","--prompt-input-placeholder",
        "--prompt-btn-confirm-bg","--prompt-btn-confirm-bg-hover","--prompt-btn-confirm-text",
        "--prompt-btn-cancel-bg","--prompt-btn-cancel-bg-hover","--prompt-btn-cancel-text",
        "--prompt-ext-btn-bg","--prompt-ext-btn-bg-hover","--prompt-ext-btn-text",
        "--dragdrop-overlay-bg","--dragdrop-card-bg","--dragdrop-title","--dragdrop-desc",
        "--dragdrop-invalid-bg","--dragdrop-invalid-card","--dragdrop-invalid-title","--dragdrop-invalid-text",
        "--news-overlay","--news-bg","--cloud-bg",
    ]
}

fn validate_theme(data: &serde_json::Value) -> Result<(), String> {
    let name = data["name"].as_str().ok_or("Missing name")?;
    let author = data["author"].as_str().ok_or("Missing author")?;
    if name.is_empty() { return Err("Name is empty".into()); }
    if name.len() > 32 { return Err("Name too long (max 32 chars)".into()); }
    if author.len() > 20 { return Err("Author too long (max 20 chars)".into()); }
    if !name.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_') {
        return Err("Name contains invalid characters".into());
    }
    if !author.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_') {
        return Err("Author contains invalid characters".into());
    }
    let colors = data["colors"].as_object().ok_or("Missing colors object")?;
    for key in required_keys() {
        if !colors.contains_key(key) {
            return Err(format!("Missing color key: {}", key));
        }
    }
    Ok(())
}

#[command] fn folders() -> Vec<Item> {
    let mut out = vec![];
    let scripts = scripts_dir();
    if scripts.exists() {
        out.push(Item {
            name: "Scripts".into(),
            is_folder: true,
            path: scripts.to_string_lossy().into(),
            is_root: true,
        });
    }
    out
}

#[command] fn files(path: String) -> Vec<Item> {
    let mut out = vec![];
    if let Ok(rd) = fs::read_dir(&path) {
        for e in rd.flatten() {
            let p = e.path();
            if let Some(ext) = p.extension() {
                if ext == "lua" || ext == "txt" {
                    out.push(Item { name: p.file_name().unwrap().to_string_lossy().into(), is_folder: false, path: p.to_string_lossy().into(), is_root: false });
                }
            }
        }
    }
    out
}

#[command] fn read(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[command] fn write(path: String, content: String) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}

#[command] fn create(path: String, name: String) -> Result<String, String> {
    let p = Path::new(&path).join(&name);
    fs::write(&p, "").map_err(|e| e.to_string())?;
    Ok(p.to_string_lossy().into())
}

#[command] fn rename(old_path: String, new_name: String) -> Result<String, String> {
    let old = Path::new(&old_path);
    let new = old.parent().unwrap().join(&new_name);
    fs::rename(old, &new).map_err(|e| e.to_string())?;
    Ok(new.to_string_lossy().into())
}

#[command] fn delete(path: String) -> Result<(), String> {
    let p = Path::new(&path);
    if p.is_dir() { fs::remove_dir_all(p) } else { fs::remove_file(p) }
        .map_err(|e| e.to_string())
}

#[command] fn copy(source_path: String, target_folder: String, file_name: String) -> Result<(), String> {
    fs::copy(&source_path, Path::new(&target_folder).join(&file_name))
        .map(|_| ()).map_err(|e| e.to_string())
}

#[command] fn open_dir() -> Result<(), String> {
    let dir = scripts_dir();
    #[cfg(target_os = "windows")] Command::new("explorer").arg(&dir).spawn().map_err(|e| e.to_string())?;
    #[cfg(target_os = "macos")] Command::new("open").arg(&dir).spawn().map_err(|e| e.to_string())?;
    #[cfg(target_os = "linux")] Command::new("xdg-open").arg(&dir).spawn().map_err(|e| e.to_string())?;
    Ok(())
}

#[command] fn search(query: String) -> Vec<Item> {
    let q = query.to_lowercase();
    let mut out = vec![];
    let scripts = scripts_dir();
    if let Ok(rd) = fs::read_dir(&scripts) {
        for e in rd.flatten() {
            let p = e.path();
            if let Some(ext) = p.extension() {
                let name = p.file_name().unwrap().to_string_lossy().to_string();
                if (ext == "lua" || ext == "txt") && name.to_lowercase().contains(&q) {
                    out.push(Item { name, is_folder: false, path: p.to_string_lossy().into(), is_root: false });
                }
            }
        }
    }
    out
}

#[command] fn save_tabs(tabs: Vec<Tab>, state: tauri::State<State>) -> Result<(), String> {
    *state.tabs.lock().unwrap() = tabs;
    Ok(())
}

#[command] fn load_tabs(state: tauri::State<State>) -> Vec<Tab> {
    state.tabs.lock().unwrap().clone()
}

#[command]
async fn open_file_dialog(app: tauri::AppHandle) -> Option<(String, String, String)> {
    let scripts = scripts_dir();
    let win = app.get_webview_window("main")?;
    win.set_enabled(false).ok();
    let file = rfd::FileDialog::new()
        .add_filter("Scripts", &["lua", "txt"])
        .set_directory(&scripts)
        .pick_file();
    win.set_enabled(true).ok();
    win.set_focus().ok();
    let file = file?;
    let name = file.file_name()?.to_string_lossy().into_owned();
    let content = fs::read_to_string(&file).ok()?;
    let path = file.to_string_lossy().into_owned();
    Some((name, content, path))
}

#[command]
async fn save_file_dialog(name: String, content: String, app: tauri::AppHandle) -> Option<String> {
    let scripts = scripts_dir();
    let win = app.get_webview_window("main")?;
    let stem = std::path::Path::new(&name)
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or(name.clone());
    win.set_enabled(false).ok();
    let file = rfd::FileDialog::new()
        .add_filter("Scripts", &["lua", "txt"])
        .set_directory(&scripts)
        .set_file_name(format!("{}.lua", stem))
        .save_file();
    win.set_enabled(true).ok();
    win.set_focus().ok();
    let file = file?;
    fs::write(&file, content).ok()?;
    Some(file.to_string_lossy().into_owned())
}

#[command]
async fn save_to_scripts(name: String, content: String) -> Result<(), String> {
    let stem = std::path::Path::new(&name)
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or(name.clone());
    let path = scripts_dir().join(format!("{}.lua", stem));
    fs::write(path, content).map_err(|e| e.to_string())
}

#[command] fn save_settings(data: String) -> Result<(), String> {
    let path = app_dir().join("settings.json");
    fs::write(path, data).map_err(|e| e.to_string())
}

#[command] fn load_settings() -> Result<String, String> {
    let path = app_dir().join("settings.json");
    if path.exists() {
        fs::read_to_string(path).map_err(|e| e.to_string())
    } else {
        Ok("{}".into())
    }
}

#[command]
async fn export_settings(data: String, app: tauri::AppHandle) -> Option<String> {
    let win = app.get_webview_window("main")?;
    win.set_enabled(false).ok();
    let file = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_file_name("Settings.json")
        .save_file();
    win.set_enabled(true).ok();
    win.set_focus().ok();
    let file = file?;
    fs::write(&file, data).ok()?;
    Some(file.to_string_lossy().into_owned())
}

#[command]
async fn import_settings(app: tauri::AppHandle) -> Option<String> {
    let win = app.get_webview_window("main")?;
    win.set_enabled(false).ok();
    let file = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .pick_file();
    win.set_enabled(true).ok();
    win.set_focus().ok();
    let file = file?;
    fs::read_to_string(file).ok()
}

#[command]
async fn set_topmost(app: tauri::AppHandle, value: bool) -> Result<(), String> {
    app.get_webview_window("main")
        .ok_or("no window")?
        .set_always_on_top(value)
        .map_err(|e| e.to_string())
}

#[command]
fn export_theme(name: String, author: String, colors: std::collections::HashMap<String, String>) -> Result<(), String> {
    if name.is_empty() { return Err("Name cannot be empty".into()); }
    if name.len() > 32 { return Err("Name too long (max 32 chars)".into()); }
    if author.len() > 20 { return Err("Author too long (max 20 chars)".into()); }
    if !name.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_') {
        return Err("Name has invalid characters (only letters, numbers, spaces, - and _)".into());
    }
    if !author.is_empty() && !author.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_') {
        return Err("Author has invalid characters".into());
    }

    let file_name = format!("{}.json", name);
    let path = themes_dir().join(&file_name);
    if path.exists() {
        return Err(format!("A theme named \"{}\" already exists", name));
    }

    for key in required_keys() {
        if !colors.contains_key(key) {
            return Err(format!("Missing color key: {}", key));
        }
    }

    let data = serde_json::json!({ "name": name, "author": author, "colors": colors });
    let json = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

#[command]
async fn import_theme(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let win = app.get_webview_window("main").ok_or("no window")?;
    win.set_enabled(false).ok();
    let file = rfd::FileDialog::new()
        .add_filter("Theme", &["json"])
        .set_directory(themes_dir())
        .pick_file();
    win.set_enabled(true).ok();
    win.set_focus().ok();

    let file = file.ok_or("No file selected")?;
    let raw = fs::read_to_string(&file).map_err(|e| e.to_string())?;
    let data: serde_json::Value = serde_json::from_str(&raw).map_err(|_| "Invalid JSON".to_string())?;
    validate_theme(&data)?;
    Ok(data)
}

#[command]
fn list_themes() -> Vec<ThemeMeta> {
    let mut out = vec![];
    if let Ok(rd) = fs::read_dir(themes_dir()) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().map(|x| x == "json").unwrap_or(false) {
                if let Ok(raw) = fs::read_to_string(&p) {
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&raw) {
                        if validate_theme(&data).is_ok() {
                            let name = data["name"].as_str().unwrap_or("").to_string();
                            let author = data["author"].as_str().unwrap_or("").to_string();
                            out.push(ThemeMeta { name, author });
                        }
                    }
                }
            }
        }
    }
    out.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    out
}

#[command]
fn load_theme(name: String, state: tauri::State<State>) -> Result<serde_json::Value, String> {
    let path = themes_dir().join(format!("{}.json", name));
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let data: serde_json::Value = serde_json::from_str(&raw).map_err(|_| "Invalid JSON".to_string())?;
    validate_theme(&data)?;
    *state.active_theme.lock().unwrap() = Some(path.to_string_lossy().into_owned());
    Ok(data)
}

#[command]
fn unset_theme(state: tauri::State<State>) {
    *state.active_theme.lock().unwrap() = None;
}

#[command]
fn is_active_theme(path: String, state: tauri::State<State>) -> bool {
    state.active_theme.lock().unwrap().as_deref() == Some(&path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    fs::create_dir_all(scripts_dir()).ok();
    fs::create_dir_all(themes_dir()).ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(State {
            tabs: Mutex::new(vec![Tab {
                id: 1,
                name: "Infinite Yield".into(),
                content: "loadstring(game:HttpGet('https://raw.githubusercontent.com/EdgeIY/infiniteyield/master/source'))()".into(),
                path: None,
            }]),
            active_theme: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            folders, files, read, write, create, rename, delete, copy, open_dir, search,
            save_tabs, load_tabs, open_file_dialog, save_file_dialog, save_to_scripts,
            save_settings, load_settings, export_settings, import_settings, set_topmost,
            export_theme, import_theme, list_themes, load_theme, unset_theme, is_active_theme,
        ])
        .run(tauri::generate_context!())
        .expect("error while running");
}