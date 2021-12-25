use crate::plugin::Plugin;
use neovim_lib::{NeovimApi, Value};

pub fn register(plugin:  &mut Plugin){
    plugin.handle(String::from("Test"),sum);
}

pub fn sum(p : &mut Plugin, args :Vec<Value>){
    let nums = args.iter()
        .map(|v| v.as_i64().unwrap())
        .collect::<Vec<i64>>();
    let sum = nums.iter().sum::<i64>();
    p.nvim
        .command(&format!("echo \"{}\"",sum))
        .unwrap();
}
