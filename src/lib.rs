use core::num;

use mlua::Lua;
use mlua::chunk;
use mlua::prelude::*;

// 可以正常使用
fn hello(lua:&Lua,name:String) -> LuaResult<LuaTable> {
    let t = lua.create_table()?;
    t.set("name",name.clone())?;
    let _globals = lua.globals(); // why globals
    lua.load(chunk!{
        print("Hello " .. $name)
    }).exec()?;
    Ok(t)
}

// cannot use in neovim
// not equivalent to load lua chunk
//fn hello(lua:&Lua,name:String) -> LuaResult<()> {
//    print!("Hello {}",name); //  print to stdout not equal to load lua chunk
//    println!("Hello {}",name);
//    Ok(())
//}

// require'myvim'.sum({1,2,3})
fn sum(_lua:&Lua,numbers:Vec<i32>) -> LuaResult<i32> {
    Ok(numbers.iter().sum())
}

// require'myvim'.product({1,2,3})
fn product(_lua:&Lua,numbers:Vec<i32>) -> LuaResult<i32> {
    Ok(numbers.iter().product())
}


#[mlua::lua_module]
fn myvim(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("hello",lua.create_function(hello)?)?;
    lua.create_table_from([
                          ("hello",lua.create_function(hello)?),
                          ("sum",lua.create_function(sum)?),
                          ("product",lua.create_function(product)?),
                          ])

}
