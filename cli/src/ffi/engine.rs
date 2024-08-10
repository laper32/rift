#[allow(non_snake_case)]
extern "C" {
    fn __InitRiftEngine() -> bool;
    fn __ShutdownRiftEngine();
}

pub fn init_engine() -> bool {
    unsafe { __InitRiftEngine() }
}

pub fn shutdown_engine() {
    unsafe { __ShutdownRiftEngine() }
}
