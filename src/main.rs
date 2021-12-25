mod plugin;
mod handlers;
mod myplug;


use plugin::Plugin;
use crate::handlers::register;

fn main() {
    let mut plugin = Plugin::new();
    register(&mut plugin);
    plugin.start();
}
