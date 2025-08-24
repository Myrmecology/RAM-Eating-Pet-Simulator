// src/system/monitor.rs
// RAM Eating Pet Simulator - System Monitoring

use anyhow::Result;
use sysinfo::{System, Pid, ProcessExt, SystemExt};
use std::sync::{Arc, Mutex};

/// System monitor for tracking RAM usage
pub struct SystemMonitor {
    system: Arc<Mutex<System>>,
}

impl SystemMonitor {
    /// Create a new system monitor
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        SystemMonitor {
            system: Arc::new(Mutex::new(system)),
        }
    }
    
    /// Update system information
    pub fn update(&self) -> Result<()> {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_memory();
        sys.refresh_processes();
        Ok(())
    }
    
    /// Get total system RAM in MB
    pub fn get_total_ram_mb(&self) -> usize {
        let sys = self.system.lock().unwrap();
        (sys.total_memory() / 1024) as usize
    }
    
    /// Get used system RAM in MB
    pub fn get_used_ram_mb(&self) -> usize {
        let sys = self.system.lock().unwrap();
        (sys.used_memory() / 1024) as usize
    }
    
    /// Get free system RAM in MB
    pub fn get_free_ram_mb(&self) -> usize {
        let sys = self.system.lock().unwrap();
        (sys.available_memory() / 1024) as usize
    }
    
    /// Get RAM usage percentage
    pub fn get_ram_usage_percent(&self) -> f32 {
        let sys = self.system.lock().unwrap();
        let total = sys.total_memory() as f32;
        let used = sys.used_memory() as f32;
        
        if total > 0.0 {
            (used / total) * 100.0
        } else {
            0.0
        }
    }
    
    /// Get current process RAM usage in MB
    pub fn get_process_ram_mb(&self) -> Result<usize> {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_processes();
        
        let pid = Pid::from(std::process::id() as i32);
        
        if let Some(process) = sys.process(pid) {
            Ok((process.memory() / 1024) as usize)
        } else {
            // Fallback: estimate based on our allocations
            Ok(50) // Base overhead estimate
        }
    }
    
    /// Get system information summary
    pub fn get_system_info(&self) -> SystemInfo {
        let sys = self.system.lock().unwrap();
        
        SystemInfo {
            total_ram_mb: (sys.total_memory() / 1024) as usize,
            used_ram_mb: (sys.used_memory() / 1024) as usize,
            free_ram_mb: (sys.available_memory() / 1024) as usize,
            cpu_count: sys.cpus().len(),
            system_name: sys.name().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: sys.kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            os_version: sys.os_version().unwrap_or_else(|| "Unknown".to_string()),
            host_name: sys.host_name().unwrap_or_else(|| "Unknown".to_string()),
        }
    }
    
    /// Check if system is under memory pressure
    pub fn is_memory_pressure(&self) -> bool {
        self.get_free_ram_mb() < 500 || self.get_ram_usage_percent() > 90.0
    }
    
    /// Get top memory consuming processes
    pub fn get_top_processes(&self, count: usize) -> Vec<ProcessInfo> {
        let sys = self.system.lock().unwrap();
        let mut processes: Vec<ProcessInfo> = sys.processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                memory_mb: (process.memory() / 1024) as usize,
            })
            .collect();
        
        processes.sort_by(|a, b| b.memory_mb.cmp(&a.memory_mb));
        processes.truncate(count);
        processes
    }
    
    /// Monitor RAM changes over time
    pub fn get_ram_delta(&self, previous_free: usize) -> i32 {
        let current_free = self.get_free_ram_mb();
        current_free as i32 - previous_free as i32
    }
}

/// System information summary
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub total_ram_mb: usize,
    pub used_ram_mb: usize,
    pub free_ram_mb: usize,
    pub cpu_count: usize,
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
}

impl SystemInfo {
    /// Get a formatted summary
    pub fn summary(&self) -> String {
        format!(
            "System: {} | OS: {} | CPUs: {} | RAM: {}/{} MB",
            self.system_name,
            self.os_version,
            self.cpu_count,
            self.used_ram_mb,
            self.total_ram_mb
        )
    }
}

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory_mb: usize,
}

/// RAM usage tracker for historical data
pub struct RamTracker {
    history: Vec<(std::time::Instant, usize)>,
    max_history: usize,
}

impl RamTracker {
    /// Create a new RAM tracker
    pub fn new(max_history: usize) -> Self {
        RamTracker {
            history: Vec::with_capacity(max_history),
            max_history,
        }
    }
    
    /// Record current RAM usage
    pub fn record(&mut self, monitor: &SystemMonitor) {
        let now = std::time::Instant::now();
        let used = monitor.get_used_ram_mb();
        
        self.history.push((now, used));
        
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }
    
    /// Get average RAM usage over the history
    pub fn get_average(&self) -> usize {
        if self.history.is_empty() {
            return 0;
        }
        
        let sum: usize = self.history.iter().map(|(_, usage)| usage).sum();
        sum / self.history.len()
    }
    
    /// Get RAM usage trend (positive = increasing, negative = decreasing)
    pub fn get_trend(&self) -> i32 {
        if self.history.len() < 2 {
            return 0;
        }
        
        let first = self.history.first().unwrap().1 as i32;
        let last = self.history.last().unwrap().1 as i32;
        last - first
    }
    
    /// Get peak RAM usage
    pub fn get_peak(&self) -> usize {
        self.history.iter().map(|(_, usage)| *usage).max().unwrap_or(0)
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_monitor() {
        let monitor = SystemMonitor::new();
        
        // Should be able to get RAM info
        let total = monitor.get_total_ram_mb();
        assert!(total > 0);
        
        let free = monitor.get_free_ram_mb();
        assert!(free > 0);
        assert!(free <= total);
    }
    
    #[test]
    fn test_system_info() {
        let monitor = SystemMonitor::new();
        let info = monitor.get_system_info();
        
        assert!(info.total_ram_mb > 0);
        assert!(!info.system_name.is_empty());
        assert!(info.cpu_count > 0);
    }
    
    #[test]
    fn test_ram_tracker() {
        let mut tracker = RamTracker::new(10);
        let monitor = SystemMonitor::new();
        
        tracker.record(&monitor);
        assert!(tracker.get_average() > 0);
    }
}