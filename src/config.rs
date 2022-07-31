use serde::Deserialize;
use nvim_oxi::{FromObject, Object, object,Result};

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub log_dir: String,
}
impl FromObject for Config {
    fn from_obj(obj: Object) -> Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}

// global mut is unsafe in rust
// static mut conf: Config = Config::default();

//pub fn setup(c: Config) {
//    conf = c;
//}
