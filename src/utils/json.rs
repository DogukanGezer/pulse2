use serde_json;

use crate::services::types::AlertRule;

pub fn from_str(vector_string: &String) -> Result<Vec<AlertRule>, String> {
    let json_content = serde_json::from_str(vector_string).map_err(|e| e.to_string())?;

    Ok(json_content)
}
