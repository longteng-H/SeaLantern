use crate::models::settings::AppSettings;
use crate::services::global;

#[tauri::command]
pub fn get_settings() -> AppSettings {
    global::settings_manager().get()
}

#[tauri::command]
pub fn save_settings(settings: AppSettings) -> Result<(), String> {
    global::settings_manager().update(settings)
}

#[tauri::command]
pub fn reset_settings() -> Result<AppSettings, String> {
    global::settings_manager().reset()
}

#[tauri::command]
pub fn export_settings() -> Result<String, String> {
    let s = global::settings_manager().get();
    serde_json::to_string_pretty(&s).map_err(|e| format!("Export failed: {}", e))
}

#[tauri::command]
pub fn import_settings(json: String) -> Result<AppSettings, String> {
    let s: AppSettings = serde_json::from_str(&json)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    global::settings_manager().update(s.clone())?;
    Ok(s)
}
