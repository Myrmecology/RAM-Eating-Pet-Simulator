// src/system/mod.rs
// RAM Eating Pet Simulator - System Module

pub mod memory;
pub mod monitor;

use anyhow::Result;

/// System utilities and helpers
pub struct SystemUtils;

impl SystemUtils {
    /// Check if we're running with sufficient privileges
    pub fn check_privileges() -> Result<bool> {
        // On Windows, we don't need special privileges for memory allocation
        #[cfg(target_os = "windows")]
        {
            Ok(true)
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            Ok(true) // We're just allocating heap memory, no special privileges needed
        }
    }
    
    /// Get the process ID
    pub fn get_pid() -> u32 {
        std::process::id()
    }
    
    /// Get current executable name
    pub fn get_process_name() -> String {
        std::env::current_exe()
            .ok()
            .and_then(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "ram_pet.exe".to_string())
    }
    
    /// Check if system has enough RAM for safe operation
    pub fn check_ram_safety(required_mb: usize, min_free_mb: usize) -> Result<bool> {
        let monitor = monitor::SystemMonitor::new();
        let free_ram = monitor.get_free_ram_mb();
        
        Ok(free_ram >= required_mb + min_free_mb)
    }
    
    /// Format bytes to human readable string
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        
        if bytes == 0 {
            return "0 B".to_string();
        }
        
        let base = 1024_f64;
        let bytes_f64 = bytes as f64;
        let exponent = (bytes_f64.ln() / base.ln()).floor() as i32;
        let unit_index = exponent.min(UNITS.len() as i32 - 1) as usize;
        let size = bytes_f64 / base.powi(exponent);
        
        if size >= 100.0 {
            format!("{:.0} {}", size, UNITS[unit_index])
        } else if size >= 10.0 {
            format!("{:.1} {}", size, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }
    
    /// Get system uptime
    pub fn get_uptime() -> Result<std::time::Duration> {
        use sysinfo::System;
        let sys = System::new_all();
        Ok(std::time::Duration::from_secs(System::uptime()))
    }
}

/// System health status
#[derive(Debug, Clone)]
pub struct SystemHealth {
    pub ram_usage_percent: f32,
    pub free_ram_mb: usize,
    pub total_ram_mb: usize,
    pub process_ram_mb: usize,
    pub is_healthy: bool,
}

impl SystemHealth {
    /// Check current system health
    pub fn check() -> Result<Self> {
        let monitor = monitor::SystemMonitor::new();
        let total = monitor.get_total_ram_mb();
        let free = monitor.get_free_ram_mb();
        let used = monitor.get_used_ram_mb();
        let process = monitor.get_process_ram_mb()?;
        
        let usage_percent = (used as f32 / total as f32) * 100.0;
        let is_healthy = free > 512 && usage_percent < 90.0;
        
        Ok(SystemHealth {
            ram_usage_percent: usage_percent,
            free_ram_mb: free,
            total_ram_mb: total,
            process_ram_mb: process,
            is_healthy,
        })
    }
    
    /// Get a warning message if system is unhealthy
    pub fn get_warning(&self) -> Option<String> {
        if !self.is_healthy {
            if self.free_ram_mb < 512 {
                Some(format!("⚠️ WARNING: Only {} MB RAM free!", self.free_ram_mb))
            } else if self.ram_usage_percent > 90.0 {
                Some(format!("⚠️ WARNING: RAM usage at {:.1}%!", self.ram_usage_percent))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(SystemUtils::format_bytes(0), "0 B");
        assert_eq!(SystemUtils::format_bytes(1024), "1.00 KB");
        assert_eq!(SystemUtils::format_bytes(1048576), "1.00 MB");
        assert_eq!(SystemUtils::format_bytes(1073741824), "1.00 GB");
    }
    
    #[test]
    fn test_get_pid() {
        let pid = SystemUtils::get_pid();
        assert!(pid > 0);
    }
    
    #[test]
    fn test_system_health() {
        let health = SystemHealth::check();
        assert!(health.is_ok());
    }
}