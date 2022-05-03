mod error;
mod plug;
mod xtokio;

use mlua::prelude::*;
use mlua::Lua;

use crate::plug::github;

//lua print(vim.inspect(require'myvim'.get_tags('neoclide/coc.nvim')))
fn get_tags(lua: &Lua, repo: String) -> LuaResult<Vec<String>> {
    let tags = tokio_block!(github::get_tags(repo.as_str()));
    Ok(tags)
}

#[mlua::lua_module]
fn myvim(lua: &Lua) -> LuaResult<LuaTable> {
    lua.create_table_from([("get_tags", lua.create_function(get_tags)?)])
}
