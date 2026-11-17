use std::collections::HashMap;

use serde::{self, Deserialize, Serialize};

// Process Operations
#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub memory: u64,
    pub cpu: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Env {
    pub namespace: String,
    pub status: String,
    pub restart_time: u64,
    pub unstable_restarts: u16,
    pub kill_retry_time: u16,
    pub exec_mode: String,
    pub autorestart: bool,
    pub autostart: bool,
    pub instances: Option<u8>,
    pub cwd: Option<String>,
    pub name: String,
    pub pm_exec_path: String,
    pub pm_out_log_path: String,
    pub pm_err_log_path: String,
    pub pm_pid_path: String,
    #[serde(rename = "NODE_ENV")]
    pub node_env: Option<String>,
    #[serde(rename = "PORT")]
    pub port: Option<serde_json::Value>,
    pub created_at: Option<u64>,
    pub pm_uptime: Option<u64>,
    pub max_memory_restart: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProcess {
    pub name: String,
    pub script: String,
    pub cwd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpreter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<String>,
    pub env: HashMap<String, String>,
    pub auto_restart: Option<bool>,
    pub watch: Option<bool>,
}

// System Operations

#[derive(Debug, Serialize, Deserialize)]
pub struct Instance {
    pub pid: u32,
    pub name: String,
    pub pm_id: u32,
    pub monit: Monitor,
    pub pm2_env: Env,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Thread {
    pub name: String,
    pub cpu_usage: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Swap {
    pub total: u64,
    pub used: f64,
    pub free: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Disk {
    pub total: u64,
    pub used: f64,
    pub free: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Memory {
    pub total: u64,
    pub used: f64,
    pub free: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SystemStats {
    pub memory: Memory,
    pub swap: Swap,
    pub disk: Disk,
    pub threads: Vec<Thread>,
}

// Alert Operations
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AlertTypes {
    CpuUsage,
    MemoryUsage,
    RestartCount,
    StatusChange,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ThresholdUnits {
    Percentage,
    Bytes,
    Count,
    None,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Statuses {
    Stopped,
    Errored,
    Online,
}

impl Statuses{
    pub fn from_str(s: &str) -> Option<Statuses> {
        match s.to_lowercase().as_str() {
            "stopped" => Some(Statuses::Stopped),
            "errored" => Some(Statuses::Errored),
            "online" => Some(Statuses::Online),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlertRule {
    pub id: Option<String>,
    pub alert_type: AlertTypes,
    pub name: String,
    pub target: String,
    pub threshold: f64,
    pub threshold_unit: ThresholdUnits,
    pub alert_cooldown: Option<u64>,
    pub statuses: Option<Vec<Statuses>>,
    pub is_enabled: bool,
    pub triggered_at: Option<u64>,
    pub created_at: Option<u64>,
    pub updated_at: Option<u64>,
}
