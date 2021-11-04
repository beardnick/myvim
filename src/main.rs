mod plugin;

use plugin::Plugin;
use neovim_lib::NeovimApi;

fn main() {
    let mut plugin = Plugin::New();

    plugin.Handle(String::from("Test"), |p,values|{
        let nums = values.iter()
            .map(|v| v.as_i64().unwrap())
            .collect::<Vec<i64>>();
        let sum = nums.iter().sum::<i64>();
        p.nvim
            .command(&format!("echo \"{}\"",sum))
            .unwrap();
    });
    plugin.Start();
}
