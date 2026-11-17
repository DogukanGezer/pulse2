use std::env;
use std::path::Path;

const FILE_EXTENSIONS: [&str; 2] = ["js", "json"];

pub fn get_home_directory() -> Result<String, String> {
    let home =
        env::var("HOME").map_err(|_| "Failed to get HOME environment variable".to_string())?;
    Ok(home)
}

pub fn safe_path(input: &str) -> Result<String, String> {
    let path = Path::new(input)
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize path: {}", e))?;

    if !path.is_file() {
        return Err(format!("Path is not a file: {}", path.display()));
    }

    let extension= &path.extension();
    if extension.is_some() && !FILE_EXTENSIONS.contains(&extension.unwrap().to_str().unwrap()) {
        return Err(format!("Path is not a JS or JSON file: {}", path.display()));
    }

    let home = get_home_directory()?;
    if !path.starts_with(&home) {
        return Err(format!(
            "Path is not within HOME directory: {}",
            path.display()
        ));
    }

    Ok(path.display().to_string())
}