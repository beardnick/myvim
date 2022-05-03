mod error;
mod plug;
mod xtokio;

use mlua::prelude::*;
use mlua::Lua;

use crate::plug::github;

//lua print(vim.inspect(require'myvim'.get_tags('neoclide/coc.nvim')))
fn get_tags(lua: &Lua, repo: String) -> LuaResult<Vec<String>> {
    let result = github::repo(repo.as_str());
    let (owner, rep) = match result {
        Ok((owner, rep)) => (owner, rep),
        Err(e) => {
            return Ok(Vec::new());
        }
    };
    let tags = tokio_block!(github::get_tags(owner, rep));
    let tags = match tags {
        Ok(tags) => tags,
        Err(e) => {
            return Ok(Vec::new());
        }
    };
    Ok(tags
        .into_iter()
        .map(|tag| tag.name)
        .collect::<Vec<String>>())
}

#[mlua::lua_module]
fn myvim(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    lua.create_table_from([("get_tags", lua.create_function(get_tags)?)])
}
