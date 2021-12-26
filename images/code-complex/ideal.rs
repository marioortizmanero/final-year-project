// COMMON //

pub trait Onramp { /* ... */ }

// PLUGIN //

pub struct Stdin;
impl Onramp for Stdin { /* ... */ }
pub fn onramp_init() -> Box<dyn Onramp> { /* ... */ }

// RUNTIME //

fn runtime() {
    let plugin = libloading::load("onramp.dll");
    let onramp = plugin.onramp_init();
    onramp.run()
}
