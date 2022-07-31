mod config;
mod error;
mod plug;
mod xtokio;

use std::iter::FromIterator;

use log::{info, LevelFilter};

use nvim_oxi::{
    self as oxi,
    api::{self, Window},
    opts::*,
    print,
    types::*,
    Dictionary,
    Function,
};
use oxi::Object;

use crate::plug::github;
use crate::config::Config;


fn get_tags(repo: String) -> oxi::Result<Vec<String>>{
     Ok(tokio_block!(github::get_tags(repo.as_str())))
}


fn setup(conf:Config)->oxi::Result<()> {
    print!("{:?}",conf);
    Ok(())
}

#[oxi::module]
fn myvim() -> oxi::Result<Dictionary> {
    let conf = config::Config {
        log_dir: String::from("/tmp/myvim.log"),
    };
    simple_logging::log_to_file(conf.log_dir, LevelFilter::Info);
    Ok(Dictionary::from_iter([
        ("get_tags",Object::from(Function::from_fn(get_tags))),
        ("setup",Object::from(Function::from_fn(setup))),
    ]))
}
