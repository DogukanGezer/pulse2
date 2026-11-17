use super::types::{Disk, Memory, Swap, SystemStats, Thread};
use std::thread::sleep;
use sysinfo::{Disks, System};

pub fn system_info() -> Result<SystemStats, String> {
    let mut sys_info = System::new_all().map_err(|e| e.to_string())?;
    sys_info.refresh_all();

    let disk_info = Disks::new_with_refreshed_list().map_err(|e| e.to_string())?;
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys_info.refresh_cpu_all();
    let cpus = sys_info.cpus();

    let memory: Memory = Memory {
        total: sys_info.total_memory(),
        used: sys_info.used_memory() as f64,
        free: sys_info.free_memory() as f64,
    };

    let swap: Swap = Swap {
        total: sys_info.total_swap(),
        used: sys_info.used_swap() as f64,
        free: sys_info.free_swap() as f64,
    };

    let disk: Disk = Disk {
        total: disk_info.iter().map(|d| d.total_space()).sum(),
        used: disk_info
            .iter()
            .map(|d| d.total_space() - d.available_space())
            .sum::<u64>() as f64,
        free: disk_info.iter().map(|d| d.available_space()).sum::<u64>() as f64,
    };

    let mut threads: Vec<Thread> = Vec::new();
    for cpu in cpus {
        let thread = Thread {
            name: cpu.name().to_string(),
            cpu_usage: cpu.usage() as f64,
        };

        threads.push(thread);
    }

    let system_stats: SystemStats = SystemStats {
        memory,
        swap,
        disk,
        threads,
    };

    Ok(system_stats)
}
