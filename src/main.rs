mod plugin;

use plugin::Plugin;
use neovim_lib::NeovimApi;

fn main() {
    let mut plugin = Plugin::New();

    plugin.Handle(String::from("Test"), |p|{
        p.nvim
            .command(&format!("echo \"{}\"","hello world"))
            .unwrap();
    });
    plugin.Start();
}
