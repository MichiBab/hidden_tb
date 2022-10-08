use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use once_cell::sync::OnceCell;

#[derive(Default)]
pub struct Signaling {
    exit_called: AtomicBool,
    settings_called: AtomicBool,
}

impl Signaling {
    pub fn set_exit_called(&self) {
        self.exit_called.store(true, Ordering::SeqCst);
    }

    pub fn get_exit_called(&self) -> bool {
        self.exit_called.load(Ordering::SeqCst)
    }

    pub fn set_settings_called(&self) {
        self.settings_called.store(true, Ordering::SeqCst);
    }

    pub fn get_settings_called(&self) -> bool {
        self.settings_called.load(Ordering::SeqCst)
    }
}

pub fn get_signaling_struct() -> Arc<Signaling> {
    get_settings_called_lock().clone()
}

fn get_settings_called_lock() -> &'static Arc<Signaling> {
    static SETTINGS_INSTANCE: OnceCell<Arc<Signaling>> = OnceCell::new();
    SETTINGS_INSTANCE.get_or_init(|| Arc::new(Signaling::default()))
}
