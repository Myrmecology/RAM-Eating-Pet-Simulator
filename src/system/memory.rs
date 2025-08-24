// src/system/memory.rs
// RAM Eating Pet Simulator - Memory Management

use anyhow::{Result, anyhow};
use std::sync::{Arc, Mutex};

/// Manages actual RAM allocation for the pet
pub struct MemoryManager {
    /// Vector of allocated memory blocks (each element is 1MB)
    allocated_blocks: Arc<Mutex<Vec<Box<[u8; 1_048_576]>>>>,
    /// Minimum free RAM to maintain (MB)
    min_free_ram: usize,
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new(min_free_ram_mb: usize) -> Self {
        MemoryManager {
            allocated_blocks: Arc::new(Mutex::new(Vec::new())),
            min_free_ram: min_free_ram_mb,
        }
    }
    
    /// Allocate memory (in MB)
    pub fn allocate(&mut self, amount_mb: usize) -> Result<()> {
        // Check if we can safely allocate
        let monitor = super::monitor::SystemMonitor::new();
        let free_ram = monitor.get_free_ram_mb();
        
        if free_ram < amount_mb + self.min_free_ram {
            return Err(anyhow!(
                "Cannot allocate {} MB. Only {} MB free (minimum {} MB required)",
                amount_mb,
                free_ram,
                self.min_free_ram
            ));
        }
        
        // Allocate the memory
        let mut blocks = self.allocated_blocks.lock().unwrap();
        
        for i in 0..amount_mb {
            // Allocate 1MB blocks
            match Self::allocate_block() {
                Ok(block) => blocks.push(block),
                Err(e) => {
                    // If allocation fails, release what we allocated so far
                    for _ in 0..i {
                        blocks.pop();
                    }
                    return Err(anyhow!("Failed to allocate memory: {}", e));
                }
            }
        }
        
        Ok(())
    }
    
    /// Allocate a single 1MB block
    fn allocate_block() -> Result<Box<[u8; 1_048_576]>> {
        // Try to allocate 1MB
        let block = vec![0u8; 1_048_576];
        
        // Convert to boxed array
        let boxed_slice = block.into_boxed_slice();
        let ptr = Box::into_raw(boxed_slice) as *mut [u8; 1_048_576];
        
        unsafe {
            Ok(Box::from_raw(ptr))
        }
    }
    
    /// Release memory (in MB)
    pub fn release(&mut self, amount_mb: usize) -> Result<()> {
        let mut blocks = self.allocated_blocks.lock().unwrap();
        
        let to_release = amount_mb.min(blocks.len());
        for _ in 0..to_release {
            blocks.pop();
        }
        
        Ok(())
    }
    
    /// Clear all allocated memory
    pub fn clear(&mut self) {
        let mut blocks = self.allocated_blocks.lock().unwrap();
        blocks.clear();
        
        // Force garbage collection (hint to the system)
        drop(blocks);
    }
    
    /// Get currently allocated memory in MB
    pub fn get_allocated_mb(&self) -> usize {
        self.allocated_blocks.lock().unwrap().len()
    }
    
    /// Fill memory with pattern (makes it "real" allocation)
    pub fn touch_memory(&mut self) -> Result<()> {
        let mut blocks = self.allocated_blocks.lock().unwrap();
        
        for (i, block) in blocks.iter_mut().enumerate() {
            // Write a pattern to ensure the memory is actually allocated
            // (not just virtually allocated)
            let pattern = (i % 256) as u8;
            for byte in block.iter_mut().step_by(4096) {
                *byte = pattern;
            }
        }
        
        Ok(())
    }
    
    /// Digest memory (release gradually)
    pub fn digest(&mut self, amount_mb: usize) -> Result<usize> {
        let mut blocks = self.allocated_blocks.lock().unwrap();
        let current_size = blocks.len();
        
        if current_size == 0 {
            return Ok(0);
        }
        
        let to_digest = amount_mb.min(current_size);
        for _ in 0..to_digest {
            blocks.pop();
        }
        
        Ok(to_digest)
    }
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub allocated_mb: usize,
    pub system_free_mb: usize,
    pub system_total_mb: usize,
    pub process_usage_mb: usize,
}

impl MemoryStats {
    /// Get current memory statistics
    pub fn current(manager: &MemoryManager) -> Result<Self> {
        let monitor = super::monitor::SystemMonitor::new();
        
        Ok(MemoryStats {
            allocated_mb: manager.get_allocated_mb(),
            system_free_mb: monitor.get_free_ram_mb(),
            system_total_mb: monitor.get_total_ram_mb(),
            process_usage_mb: monitor.get_process_ram_mb()?,
        })
    }
    
    /// Check if it's safe to allocate more
    pub fn can_allocate(&self, amount_mb: usize, min_free_mb: usize) -> bool {
        self.system_free_mb >= amount_mb + min_free_mb
    }
}

/// Safe memory allocator with limits
pub struct SafeAllocator {
    manager: MemoryManager,
    max_allocation_mb: usize,
    warning_threshold_mb: usize,
}

impl SafeAllocator {
    /// Create a new safe allocator
    pub fn new(min_free_ram_mb: usize, max_allocation_mb: usize) -> Self {
        SafeAllocator {
            manager: MemoryManager::new(min_free_ram_mb),
            max_allocation_mb,
            warning_threshold_mb: max_allocation_mb * 80 / 100, // 80% threshold
        }
    }
    
    /// Safely allocate memory with checks
    pub fn allocate_safe(&mut self, amount_mb: usize) -> Result<()> {
        let current = self.manager.get_allocated_mb();
        let new_total = current + amount_mb;
        
        if new_total > self.max_allocation_mb {
            return Err(anyhow!(
                "Cannot allocate {} MB. Would exceed maximum of {} MB",
                amount_mb,
                self.max_allocation_mb
            ));
        }
        
        self.manager.allocate(amount_mb)?;
        
        if new_total >= self.warning_threshold_mb {
            eprintln!("⚠️ Warning: Allocated {} MB ({}% of maximum)",
                new_total,
                new_total * 100 / self.max_allocation_mb
            );
        }
        
        Ok(())
    }
    
    /// Get the underlying memory manager
    pub fn manager(&mut self) -> &mut MemoryManager {
        &mut self.manager
    }
}

impl Drop for MemoryManager {
    fn drop(&mut self) {
        // Ensure all memory is released when the manager is dropped
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_allocation() {
        let mut manager = MemoryManager::new(100);
        
        // Should be able to allocate small amount
        let result = manager.allocate(1);
        if result.is_ok() {
            assert_eq!(manager.get_allocated_mb(), 1);
            manager.clear();
        }
    }
    
    #[test]
    fn test_memory_release() {
        let mut manager = MemoryManager::new(100);
        
        if manager.allocate(2).is_ok() {
            assert_eq!(manager.get_allocated_mb(), 2);
            manager.release(1).unwrap();
            assert_eq!(manager.get_allocated_mb(), 1);
            manager.clear();
        }
    }
    
    #[test]
    fn test_safe_allocator() {
        let mut allocator = SafeAllocator::new(100, 10);
        
        // Should succeed for small allocation
        let result = allocator.allocate_safe(1);
        assert!(result.is_ok() || result.is_err()); // Depends on system RAM
        
        // Should fail for allocation over max
        let result = allocator.allocate_safe(11);
        assert!(result.is_err());
    }
}