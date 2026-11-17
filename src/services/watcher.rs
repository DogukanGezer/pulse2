use crate::services::types::{AlertRule, Statuses, ThresholdUnits};

use super::types::{AlertTypes, Instance};
use super::{process, store, system};

struct SnapShot {
    name: String,
    status: String,
    restart_time: u64,
}

pub fn watcher(on_alert: impl Fn(&AlertRule, &Instance, &str)) {
    let mut snap_shot: Vec<SnapShot> = Vec::new();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));

        let instances: Vec<Instance> = process::list_processes().unwrap_or_default();
        let rules = store::store().and_then(|s| s.list()).unwrap_or_default();

        for rule in rules {
            let instance = instances.iter().find(|i| i.name == rule.target);
            if instance.is_none() {
                continue;
            }

            let instance = instance.unwrap();

            let message = match rule.alert_type {
                AlertTypes::CpuUsage => check_cpu_usage(instance, &rule),
                AlertTypes::MemoryUsage => check_memory_usage(instance, &rule),
                AlertTypes::RestartCount => check_restart_count(instance, &rule, &snap_shot),
                AlertTypes::StatusChange => check_status_change(instance, &rule, &snap_shot),
            };

            if let Some(message) = message {
                on_alert(&rule, instance, &message);
            }
        }

        set_snap_shot(&mut snap_shot, &instances);
    }
}

fn check_cpu_usage(instance: &Instance, rule: &AlertRule) -> Option<String> {
    if instance.monit.cpu < rule.threshold {
        return None;
    }

    Some(format!(
        "CPU usage for instance '{}' is above threshold: {}%",
        instance.name, instance.monit.cpu
    ))
}

fn check_memory_usage(instance: &Instance, rule: &AlertRule) -> Option<String> {
    let system_info = system::system_info().unwrap();
    let limit = instance.pm2_env.max_memory_restart.unwrap_or(0);

    let mut max_memory = system_info.memory.total;

    if limit > 0 {
        max_memory = limit;
    }

    match rule.threshold_unit {
        ThresholdUnits::Percentage => {
            let threshold_bytes = (max_memory as f64 * (rule.threshold / 100.0)) as u64;
            if instance.monit.memory < threshold_bytes {
                return None;
            }
        }
        ThresholdUnits::Bytes => {
            let threshold_bytes = (rule.threshold) as u64; // Convert MB to bytes
            if instance.monit.memory < threshold_bytes {
                return None;
            }
        }
        _ => return None, // Unsupported unit
    }

    Some(format!(
        "Memory usage for instance '{}' is above threshold: {} {:?}",
        instance.name, rule.threshold, rule.threshold_unit
    ))
}

fn check_restart_count(
    instance: &Instance,
    rule: &AlertRule,
    snap_shot: &Vec<SnapShot>,
) -> Option<String> {
    let prev_restart = snap_shot
        .iter()
        .find(|s| s.name == instance.name)
        .map(|s| s.restart_time)?;

    let diff = instance.pm2_env.restart_time.saturating_sub(prev_restart);
    if diff < rule.threshold as u64 {
        return None;
    }

    Some(format!(
        "Restart count for instance '{}' is above threshold: {}",
        instance.name, instance.pm2_env.restart_time
    ))
}

fn check_status_change(
    instance: &Instance,
    rule: &AlertRule,
    snap_shot: &Vec<SnapShot>,
) -> Option<String> {
    let prev_status = snap_shot
        .iter()
        .find(|s| s.name == instance.name)
        .map(|s| s.status.clone())?;

    if prev_status == instance.pm2_env.status {
        return None;
    }

    let now_status = Statuses::from_str(&instance.pm2_env.status)?;

    if rule.statuses.is_none() || !rule.statuses.as_ref().unwrap().contains(&now_status) {
        return None;
    }

    Some(format!(
        "Status change for instance '{}' detected: {} -> {}",
        instance.name, prev_status, instance.pm2_env.status
    ))
}

fn set_snap_shot(snap_shot: &mut Vec<SnapShot>, instances: &Vec<Instance>) {
    for instance in instances {
        let existing_snap_shot = snap_shot.iter_mut().find(|s| s.name == instance.name);
        if let Some(s) = existing_snap_shot {
            s.status = instance.pm2_env.status.clone();
            s.restart_time = instance.pm2_env.restart_time;
        } else {
            snap_shot.push(SnapShot {
                name: instance.name.clone(),
                status: instance.pm2_env.status.clone(),
                restart_time: instance.pm2_env.restart_time,
            });
        }
    }
}
