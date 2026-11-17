use crate::services::fs::backup_file;

use super::super::utils::file;
use super::{
    fs,
    types::{CreateProcess, Instance},
};
use std::process::Command;

pub fn list_processes() -> Result<Vec<Instance>, String> {
    let result = Command::new("pm2")
        .arg("jlist")
        .output()
        .expect("Failed to execute command");

    let json = String::from_utf8_lossy(&result.stdout);
    let instances: Vec<Instance> = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    Ok(instances)
}

pub fn start_process(process_name: &String) -> Result<(), String> {
    if process_name == &String::from("Null") {
        return Err("Process name is required".into());
    }

    let result = Command::new("pm2")
        .args(["start", process_name, "--silent"])
        .output()
        .map_err(|e| e.to_string())?;
 
    if result.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&result.stderr).trim().to_string())
    }
}

pub fn stop_process(process_name: &String) -> Result<(), String> {
    if process_name == &String::from("Null") {
        return Err("Process name is required".into());
    }

    let result = Command::new("pm2")
        .args(["stop", process_name, "--silent"])
        .output()
        .map_err(|e| e.to_string())?;

    if result.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&result.stderr).trim().to_string())
    }
}

pub fn restart_process(process_name: &String) -> Result<(), String> {
    if process_name == &String::from("Null") {
        return Err("Process name is required".into());
    }

    let result = Command::new("pm2")
        .args(["restart", process_name, "--silent"])
        .output()
        .map_err(|e| e.to_string())?;

    if result.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&result.stderr).trim().to_string())
    }
}

pub fn delete_process(process_name: &String) -> Result<(), String> {
    if process_name == &String::from("Null") {
        return Err("Process name is required".into());
    }

    let result = Command::new("pm2")
        .args(["delete", process_name, "--silent"])
        .output()
        .map_err(|e| e.to_string())?;

    if result.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&result.stderr).trim().to_string())
    }
}

pub fn create_process(input: &CreateProcess) -> Result<(), String> {
    let mut cmd: Command = Command::new("pm2");

    for (key, value) in &input.env {
        cmd.env(key, value);
    }

    cmd.args(["start", &input.script, "--name", &input.name]);

    if input.cwd != "" {
        cmd.args(["--cwd", &input.cwd]);
    }

    if input.instances.is_some() {
        cmd.args(["--instances", &input.instances.clone().unwrap()]);
    }

    if input.auto_restart == Some(false) {
        cmd.args(["--no-autorestart"]);
    }

    if input.watch == Some(true) {
        cmd.args(["--watch"]);
    }

    if input.arguments.is_some() {
        cmd.args(["--", &input.arguments.clone().unwrap()]);
    }

    let result = cmd.output().map_err(|e| e.to_string())?;

    if result.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&result.stderr).trim().to_string())
    }
}

fn prepare_file_path(path: &String, is_js: bool) -> Result<String, String> {
    if path.trim().is_empty() {
        let home = file::get_home_directory()
            .map_err(|e| e.to_string())
            .unwrap();

        let extension = if is_js { "js" } else { "json" };
        let file_path = home + "/pulse2-ecosystem.config." + extension;

        return Ok(file_path);
    }

    if file::safe_path(path).is_err() {
        return Err("Invalid file path".into());
    }

    Ok(path.clone())
}

pub fn create_from_ecosystem(content: &String, is_js: bool, path: &String) -> Result<(), String> {
    if content.trim().is_empty() {
        return Err("Ecosystem content is required".into());
    }

    let file_path = prepare_file_path(path, is_js).map_err(|e| e.to_string())?;

    if !path.trim().is_empty() {
        backup_file(&file_path).map_err(|e| e.to_string())?;
    }

    fs::write_file(&file_path, &content).map_err(|e| e.to_string())?;

    let result = Command::new("pm2")
        .args(["startOrReload", &file_path])
        .output()
        .map_err(|e| e.to_string())?;

    if path.trim().is_empty() {
        fs::delete_file(&file_path).map_err(|e| e.to_string())?;
    }

    if !result.status.success() {
        return Err(String::from_utf8_lossy(&result.stderr).trim().to_string());
    }

    Ok(())
}

pub fn list_logs(process_name: &String, lines: u8) -> Result<String, String> {
    if process_name == &String::from("Null") {
        return Err("Process name is required".into());
    }

    let result = Command::new("pm2")
        .args([
            "logs",
            process_name,
            "--lines",
            &lines.to_string(),
            "--nostream",
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if !result.status.success() {
        return Err(String::from_utf8_lossy(&result.stderr).trim().to_string());
    }

    Ok(result.stdout.iter().map(|&c| c as char).collect::<String>())
}

pub fn dump_all(path: &String) -> Result<(), String> {
    let result = Command::new("pm2")
        .args(["save", "--silent"])
        .output()
        .map_err(|e| e.to_string())?;

    if !result.status.success() {
        return Err(String::from_utf8_lossy(&result.stderr).trim().to_string());
    }

    let home = file::get_home_directory().map_err(|e| e.to_string())?;
    let dump_path = format!("{}/.pm2/dump.pm2", home);

    fs::copy_file(&dump_path, &path).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn restore_all(path: &String) -> Result<(), String> {
    let home = file::get_home_directory().map_err(|e| e.to_string())?;
    let dump_path = format!("{}/.pm2/dump.pm2", home);

    fs::backup_file(&dump_path).map_err(|e| e.to_string())?;
    fs::copy_file(&path, &dump_path).map_err(|e| e.to_string())?;

    let result = Command::new("pm2")
        .args(["resurrect", "--silent"])
        .output()
        .map_err(|e| e.to_string())?;

    if !result.status.success() {
        return Err(String::from_utf8_lossy(&result.stderr).trim().to_string());
    }

    Ok(())
}
