pub struct EventManager {}

impl EventManager {
    fn new() -> Self {
        Self {}
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<EventManager> =
            once_cell::sync::Lazy::new(|| EventManager::new());
        unsafe { &mut *INSTANCE }
    }
}
