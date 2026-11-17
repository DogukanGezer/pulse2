use super::store;
use super::time;
use super::types::AlertRule;
use nanoid;

pub fn list_alert() -> Result<Vec<AlertRule>, String> {
    let rule_store = store::store()?;
    rule_store.list()
}

pub fn update_alert(updated_rule: &AlertRule) -> Result<AlertRule, String> {
    let rule_store = store::store()?;
    let rules: Vec<AlertRule> = rule_store.list()?;

    alert_conditions(&rules, &updated_rule)?;

    let founded_rule = rules
        .iter()
        .find(|rule| rule.id == updated_rule.id)
        .ok_or(format!(
            "Alert rule with id '{}' not found",
            updated_rule.id.clone().unwrap_or_default()
        ))?;

    let rule: AlertRule = AlertRule {
        triggered_at: founded_rule.triggered_at,
        created_at: founded_rule.created_at,
        updated_at: Some(time::generate_timestamp()),
        ..updated_rule.clone()
    };

    rule_store.update(rule.clone())?;

    Ok(rule)
}

pub fn insert_alert(new_rule: &AlertRule) -> Result<AlertRule, String> {
    let rule_store = store::store()?;
    let rules: Vec<AlertRule> = rule_store.list()?;

    alert_conditions(&rules, &new_rule)?;

    let id: String = generated_id()?;
    let new_rule: AlertRule = AlertRule {
        id: Some(id),
        created_at: Some(time::generate_timestamp()),
        updated_at: Some(time::generate_timestamp()),
        triggered_at: None,
        ..new_rule.clone()
    };

    rule_store.insert(new_rule.clone())?;

    Ok(new_rule)
}

pub fn delete_alert(id: &String) -> Result<(), String> {
    let rule_store = store::store()?;
    let rules: Vec<AlertRule> = rule_store.list()?;

    if rules
        .iter()
        .find(|rule| rule.id.as_ref() == Some(id))
        .is_none()
    {
        return Err(format!("Alert rule with id '{}' not found", id));
    }

    rule_store.delete(id.clone())?;

    Ok(())
}

// helper functions for creating alert rules
// helper function to generate a unique id for the alert rule
fn generated_id() -> Result<String, String> {
    let id = nanoid::nanoid!(6);
    Ok(id)
}

// helper function to check if an alert rule with the same name or target already exists
fn alert_conditions(rules: &Vec<AlertRule>, new_rule: &AlertRule) -> Result<(), String> {
    if new_rule.name.is_empty() || new_rule.target.is_empty() {
        return Err("Alert rule name and target cannot be empty".to_string());
    }

    if rules
        .iter()
        .any(|rule| rule.name == new_rule.name && new_rule.id.is_none())
    {
        return Err(format!(
            "Alert rule with name '{}' already exists",
            new_rule.name
        ));
    }

    if rules
        .iter()
        .any(|rule| rule.target == new_rule.target && rule.alert_type == new_rule.alert_type)
        && new_rule.id.is_none()
    {
        return Err(format!(
            "Alert rule with target '{}' and type '{:?}' already exists",
            new_rule.target, new_rule.alert_type
        ));
    }

    if rules
        .iter()
        .any(|rule| rule.name == new_rule.name && rule.id != new_rule.id)
        && new_rule.id.is_some()
    {
        return Err(format!(
            "Alert rule with name '{}' already exists",
            new_rule.name
        ));
    }

    if rules.iter().any(|rule| {
        rule.id != new_rule.id
            && rule.alert_type == new_rule.alert_type
            && rule.target == new_rule.target
    }) && new_rule.id.is_some()
    {
        return Err(format!(
            "Alert rule with target '{}' and type '{:?}' already exists",
            new_rule.target, new_rule.alert_type
        ));
    }

    Ok(())
}
