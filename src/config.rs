use serde::Deserialize;
use nvim_oxi::{lua, Object, Result};
use nvim_oxi::conversion::{self, Error, FromObject, ToObject};
use nvim_oxi::lua::ffi::lua_State;
use nvim_oxi::lua::Poppable;
use nvim_oxi::serde::Deserializer;


#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub log_dir: String,
}

impl FromObject for Config {
    fn from_object(object: Object) -> std::result::Result<Self, Error> {
        Self::deserialize(Deserializer::new(object)).map_err(Into::into)
    }
}

impl Poppable for Config {
    unsafe fn pop(lua_state: *mut lua_State) -> std::result::Result<Self, nvim_oxi::lua::Error> {
        let obj = Object::pop(lua_state)?;
        Self::from_object(obj)
            .map_err(lua::Error::pop_error_from_err::<Self, _>)

    }
}
