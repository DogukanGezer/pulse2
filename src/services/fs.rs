use super::super::utils::file;
use std::fs;

pub fn read_file(file_path: &str) -> Result<String, String> {
    file::safe_path(file_path).map_err(|e| e.to_string())?;
    let content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

    Ok(content)
}

pub fn write_file(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn delete_file(path: &str) -> Result<(), String> {
    fs::remove_file(path).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn backup_file(file_path: &str) -> Result<String, String> {
    let backup_path = format!("{}.bak", file_path);

    copy_file(file_path, &backup_path).map_err(|e| e.to_string())?;

    Ok(backup_path)
}

pub fn copy_file(src: &str, dest: &str) -> Result<(), String> {
    fs::copy(src, dest).map_err(|e| format!("Failed to copy file: {}", e))?;

    Ok(())
}
