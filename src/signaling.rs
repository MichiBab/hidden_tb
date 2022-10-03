use std::sync::RwLock;

use once_cell::sync::OnceCell;

pub fn set_exit_called() {
    if let Ok(mut exit_called_lock) = get_exit_called_lock().write() {
        *exit_called_lock = true;
        return;
    }
    panic!("Could not set exit called");
}

pub fn get_exit_called() -> bool {
    if let Ok(exit_called_lock) = get_exit_called_lock().read() {
        return *exit_called_lock;
    }
    panic!("Could not get exit called");
}

fn get_exit_called_lock() -> &'static RwLock<bool> {
    static INSTANCE: OnceCell<RwLock<bool>> = OnceCell::new();
    INSTANCE.get_or_init(|| RwLock::new(false))
}
