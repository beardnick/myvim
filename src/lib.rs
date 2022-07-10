mod config;
mod error;
mod plug;
mod xtokio;

use log::{info, LevelFilter};
use mlua::prelude::*;
use mlua::serde::Deserializer;
use mlua::Lua;
use serde_path_to_error::deserialize;

use crate::plug::github;

//lua print(vim.inspect(require'myvim'.get_tags('neoclide/coc.nvim')))
fn get_tags(lua: &Lua, repo: String) -> LuaResult<Vec<String>> {
    let tags = tokio_block!(github::get_tags(repo.as_str()));
    Ok(tags)
}

fn setup(lua: &Lua, conf: Option<mlua::Table>) -> LuaResult<()> {
    let settings = if let Some(table) = conf {
        let deserializer = Deserializer::new(mlua::Value::Table(table));
        match deserialize::<_, config::Config>(deserializer) {
            Ok(settings) => settings,
            Err(err) => {
                return Err(err.into_inner());
            }
        }
    } else {
        config::Config::default()
    };
    info!("setup {:?}", settings);
    Ok(())
}

#[mlua::lua_module]
fn myvim(lua: &Lua) -> LuaResult<LuaTable> {
    let conf = config::Config {
        log_dir: String::from("/tmp/myvim.log"),
    };
    simple_logging::log_to_file(conf.log_dir, LevelFilter::Info);
    lua.create_table_from([
        ("get_tags", lua.create_function(get_tags)?),
        ("setup", lua.create_function(setup)?),
    ])
}
