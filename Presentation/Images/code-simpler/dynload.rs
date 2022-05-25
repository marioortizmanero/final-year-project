#[repr(C)]
pub struct Config { /* only `repr(C)` fields */ }

pub extern "C" fn init_plugin(conf: Config, arr: *const i32, len: u32) {
    // ...
}







