mod commands;
mod services;
mod models;
mod utils;

use commands::server as server_commands;
use commands::java as java_commands;
use commands::config as config_commands;
use commands::system as system_commands;
use commands::player as player_commands;
use commands::settings as settings_commands;
use commands::update as update_commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            server_commands::create_server,
            server_commands::import_server,
            server_commands::start_server,
            server_commands::stop_server,
            server_commands::send_command,
            server_commands::get_server_list,
            server_commands::get_server_status,
            server_commands::delete_server,
            server_commands::get_server_logs,
            java_commands::detect_java,
            java_commands::validate_java_path,
            config_commands::read_config,
            config_commands::write_config,
            config_commands::read_server_properties,
            config_commands::write_server_properties,
            system_commands::get_system_info,
            system_commands::pick_jar_file,
            system_commands::pick_java_file,
            player_commands::get_whitelist,
            player_commands::get_banned_players,
            player_commands::get_ops,
            player_commands::add_to_whitelist,
            player_commands::remove_from_whitelist,
            player_commands::ban_player,
            player_commands::unban_player,
            player_commands::add_op,
            player_commands::remove_op,
            player_commands::kick_player,
            player_commands::export_logs,
            settings_commands::get_settings,
            settings_commands::save_settings,
            settings_commands::reset_settings,
            settings_commands::export_settings,
            settings_commands::import_settings,
            update_commands::check_update,
            update_commands::open_download_url,
        ])
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let settings = services::global::settings_manager().get();
                if settings.close_servers_on_exit {
                    services::global::server_manager().stop_all_servers();
                }
            }
        })
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running Sea Lantern");
}
