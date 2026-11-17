use pulse2::services::{
    alert, fs, process, store, system, time,
    types::{AlertRule, CreateProcess, Instance, SystemStats},
    watcher::watcher,
};
use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_notification::NotificationExt;

mod types;
use types::AppState;

//Process Operations
#[tauri::command]
fn list_processes() -> Result<Vec<Instance>, String> {
    process::list_processes()
}

#[tauri::command]
fn restart_process(process_name: String) -> Result<(), String> {
    process::restart_process(&process_name)
}

#[tauri::command]
fn stop_process(process_name: String) -> Result<(), String> {
    process::stop_process(&process_name)
}

#[tauri::command]
fn start_process(process_name: String) -> Result<(), String> {
    process::start_process(&process_name)
}

#[tauri::command]
fn delete_process(process_name: String) -> Result<(), String> {
    process::delete_process(&process_name)
}

#[tauri::command]
fn log_process(process_name: String, lines: u8) -> Result<String, String> {
    process::list_logs(&process_name, lines)
}

#[tauri::command]
fn create_process(input: CreateProcess) -> Result<(), String> {
    process::create_process(&input)
}

#[tauri::command]
fn create_from_ecosystem(content: String, is_js: bool, path: String) -> Result<(), String> {
    process::create_from_ecosystem(&content, is_js, &path)
}

#[tauri::command]
fn dump_all(path: String) -> Result<(), String> {
    process::dump_all(&path)
}

#[tauri::command]
fn restore_all(path: String) -> Result<(), String> {
    process::restore_all(&path)
}

//System Operations
#[tauri::command]
fn get_system_info() -> Result<SystemStats, String> {
    system::system_info()
}

//Fs Operations
#[tauri::command]
fn read_file(file_path: String) -> Result<String, String> {
    fs::read_file(&file_path)
}

//Rule Operations
#[tauri::command]
fn list_alert_rules() -> Result<Vec<AlertRule>, String> {
    alert::list_alert()
}

#[tauri::command]
fn create_alert_rule(new_rule: AlertRule) -> Result<AlertRule, String> {
    alert::insert_alert(&new_rule)
}

#[tauri::command]
fn update_alert_rule(updated_rule: AlertRule) -> Result<AlertRule, String> {
    alert::update_alert(&updated_rule)
}

#[tauri::command]
fn delete_alert_rule(id: String) -> Result<(), String> {
    alert::delete_alert(&id)
}

#[tauri::command]
fn get_rules_path(state: tauri::State<AppState>) -> String {
    state.config_path.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            list_processes,
            restart_process,
            stop_process,
            start_process,
            log_process,
            delete_process,
            create_process,
            create_from_ecosystem,
            get_system_info,
            read_file,
            dump_all,
            restore_all,
            list_alert_rules,
            create_alert_rule,
            update_alert_rule,
            delete_alert_rule,
            get_rules_path
        ])
        .setup(|app| {
            let dir = app.path().app_config_dir()?;
            const FILE_NAME: &str = "rules.json";

            std::fs::create_dir_all(&dir)?;
            let config_path = dir.join(FILE_NAME).to_string_lossy().to_string();

            if !std::path::Path::new(&config_path).exists() {
                std::fs::write(&config_path, "[]").map_err(|err| err.to_string())?;
            }

            store::init(&config_path)?;

            let handle = app.handle().clone();
            std::thread::spawn(move || {
                watcher(|rule, instance, message| {
                    // uygulama içi toast + Recent alerts
                    let _ = handle.emit(
                        "pm2-alert",
                        serde_json::json!({
                            "rule_id": rule.id,
                            "rule_name": rule.name,
                            "alert_type": rule.alert_type,
                            "process_name": instance.name,
                            "message": message,
                            "ts": time::generate_timestamp(),
                        }),
                    );

                    // işletim sistemi bildirimi
                    let _ = handle
                        .notification()
                        .builder()
                        .title(format!("pm2: {}", rule.name))
                        .body(message)
                        .show();
                })
            });

            app.manage(AppState {
                config_path: config_path,
            });

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
