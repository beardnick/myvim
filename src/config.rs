use serde::Deserialize;
use nvim_oxi::{lua, Object};
use nvim_oxi::conversion::{self, Error, FromObject, ToObject};
use nvim_oxi::lua::ffi::lua_State;
use nvim_oxi::lua::Poppable;
use nvim_oxi::serde::Deserializer;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub log_dir: String,
}

impl FromObject for Config {
    fn from_object(object: Object) -> Result<Self, Error> {
        Self::deserialize(Deserializer::new(object)).map_err(Into::into)
    }
}

impl Poppable for Config {
    unsafe fn pop(lua_state: *mut lua_State) -> Result<Self, lua::Error> {
        let obj = Object::pop(lua_state)?;
        Self::from_object(obj)
            .map_err(lua::Error::pop_error_from_err::<Self, _>)
    }
}

impl Default for Config {
    fn default() -> Self {
        return Config {
            log_dir: String::from("/tmp/myvim.log"),
        };
    }
}
