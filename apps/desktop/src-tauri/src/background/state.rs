use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub struct BackgroundScanState {
    pub enabled: AtomicBool,
    pub interval_minutes: AtomicU64,
    pub is_scanning: AtomicBool,
}

impl Default for BackgroundScanState {
    fn default() -> Self {
        Self {
            enabled: AtomicBool::new(true),
            interval_minutes: AtomicU64::new(30),
            is_scanning: AtomicBool::new(false),
        }
    }
}

impl BackgroundScanState {
    pub fn set_enabled(&self, val: bool) {
        self.enabled.store(val, Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn set_interval(&self, minutes: u64) {
        self.interval_minutes.store(minutes, Ordering::Relaxed);
    }

    pub fn interval(&self) -> u64 {
        self.interval_minutes.load(Ordering::Relaxed)
    }

    pub fn mark_scanning(&self, val: bool) {
        self.is_scanning.store(val, Ordering::Relaxed);
    }

    pub fn is_scanning(&self) -> bool {
        self.is_scanning.load(Ordering::Relaxed)
    }
}
