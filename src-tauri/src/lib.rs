use kirei::{cleaner, scanner, config};
use std::path::PathBuf;

#[tauri::command]
async fn get_config() -> Result<config::Config, String> {
    Ok(config::load_config().await)
}

#[tauri::command]
async fn save_config(config: config::Config) -> Result<(), String> {
    config::save_config(&config).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn reset_app(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    // Close the main window to prevent interaction during wipe
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }

    // Wipe Config
    if let Some(config_dir) = directories::ProjectDirs::from("com", "kirei", "kirei")
        .map(|p| p.config_dir().to_path_buf()) 
    {
         println!("Wiping config dir: {:?}", config_dir);
         let _ = std::fs::remove_dir_all(&config_dir);
    } else {
        println!("Could not resolve config dir");
    }

    // Wipe Cache (if any)
    if let Some(cache_dir) = directories::ProjectDirs::from("com", "kirei", "kirei")
        .map(|p| p.cache_dir().to_path_buf())
    {
        println!("Wiping cache dir: {:?}", cache_dir);
        let _ = std::fs::remove_dir_all(&cache_dir);
    }

    // Quit App
    app.exit(0);
    Ok(())
}

#[tauri::command]
async fn check_permissions() -> Result<bool, String> {
    // Heuristic: Try to read a protected directory like ~/Library/Safari
    let home = directories::UserDirs::new()
        .ok_or("Could not find home directory")?
        .home_dir()
        .to_path_buf();
    
    let protected_path = home.join("Library/Safari");
    
    match std::fs::read_dir(&protected_path) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
async fn open_privacy_settings() -> Result<(), String> {
    use std::process::Command;
    // URL Scheme for macOS Settings > Privacy & Security > Full Disk Access
    let output = Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        // Fallback to generic Security pane
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security")
            .output()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn scan() -> Result<Vec<scanner::ScanResult>, String> {
    scanner::Scanner::new()
        .await
        .scan_all()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn clean(paths: Vec<PathBuf>) -> Result<u64, String> {
    println!("Frontend requested clean for paths: {:?}", paths);
    match cleaner::Cleaner::move_to_trash(&paths) {
        Ok(size) => {
            println!("Cleaned {} bytes", size);
            Ok(size)
        },
        Err(e) => {
            println!("Clean failed: {}", e);
            Err(e.to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan, clean, get_config, save_config, reset_app, check_permissions, open_privacy_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
