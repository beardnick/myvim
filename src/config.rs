use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub log_dir: String,
}

// global mut is unsafe in rust
// static mut conf: Config = Config::default();

//pub fn setup(c: Config) {
//    conf = c;
//}
