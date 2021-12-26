// PLUGIN //

pub extern "C" fn init_onramp() -> *mut c_void { /* ... */ }
pub extern "C" fn run_onramp(onramp: *mut c_void) { /* ... */ }

// RUNTIME //

unsafe fn runtime() {
    let plugin = libloading::load("onramp.dll");
    let onramp = plugin.onramp_init();
    plugin.onramp_run(onramp)
}
