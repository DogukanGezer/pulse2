use super::fs;
use super::types::AlertRule;
use std::sync::{Mutex, OnceLock};

static RULE_STORE: OnceLock<RuleStore> = OnceLock::new();

pub fn init(path: &str) -> Result<(), String> {
    let store = RuleStore {
        path: Mutex::new(path.to_string()),
        rules: Mutex::new(Vec::new()),
    };

    store.load(path);

    RULE_STORE
        .set(store)
        .map_err(|_| "Failed to initialize RuleStore".to_string())
}

pub fn store() -> Result<&'static RuleStore, String> {
    RULE_STORE
        .get()
        .ok_or("RuleStore is not initialized".to_string())
}

pub struct RuleStore {
    pub rules: Mutex<Vec<AlertRule>>,
    pub path: Mutex<String>,
}

impl RuleStore {
    pub fn load(&self, file_path: &str) {
        let content = fs::read_file(file_path)
            .map_err(|e| e.to_string())
            .unwrap_or_default();

        let rules: Vec<AlertRule> = serde_json::from_str(&content).unwrap_or_default();

        *self.path.lock().unwrap() = file_path.to_string();
        *self.rules.lock().unwrap() = rules;
    }

    pub fn list(&self) -> Result<Vec<AlertRule>, String> {
        let rules = self.rules.lock().map_err(|e| e.to_string())?.clone();

        Ok(rules)
    }

    pub fn insert(&self, alert_rule: AlertRule) -> Result<(), String> {
        let mut rules = self.rules.lock().map_err(|e| e.to_string())?;
        rules.push(alert_rule.clone());

        self.save(&rules)?;
        Ok(())
    }

    pub fn update(&self, updated_rule: AlertRule) -> Result<(), String> {
        let mut rules = self.rules.lock().map_err(|e| e.to_string())?;

        let founded_rule = rules
            .iter_mut()
            .find(|rule| rule.id == updated_rule.id)
            .ok_or(format!(
                "Alert rule with id '{}' not found",
                updated_rule.id.clone().unwrap_or_default()
            ))?;

        *founded_rule = updated_rule;

        self.save(&rules)?;
        Ok(())
    }

    pub fn delete(&self, id: String) -> Result<(), String> {
        let mut rules = self.rules.lock().map_err(|e| e.to_string())?;
        let index = rules
            .iter()
            .position(|rule| rule.id == Some(id.clone()))
            .ok_or(format!("Alert rule with id '{}' not found", id))?;

        rules.remove(index);

        self.save(&rules)?;
        Ok(())
    }

    pub fn save(&self, rules: &Vec<AlertRule>) -> Result<(), String> {
        let new_content = serde_json::to_string_pretty(rules).map_err(|e| e.to_string())?;
        let path = self.path.lock().map_err(|e| e.to_string())?;
        fs::write_file(&path, &new_content)?;

        Ok(())
    }
}
