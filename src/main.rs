mod plugin;
mod handlers;

use plugin::Plugin;
use neovim_lib::NeovimApi;
use crate::handlers::register;

fn main() {
    let mut plugin = Plugin::New();
    register(&mut plugin);
    plugin.Start();
}
