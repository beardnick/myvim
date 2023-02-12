mod config;
mod error;
mod plug;
mod xtokio;
mod client;

use std::env::set_var;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use log::{debug, error, info, LevelFilter, warn};

use nvim_oxi::{self as oxi, print, Dictionary, Function, Error};
use oxi::api;
use oxi::Object;
use crate::client::Client;

use crate::plug::github;
use crate::config::Config;


// no argument
fn hello_world(_: ()) -> oxi::Result<()> {
    print!("hello world");
    info!("hello world log");
    Ok(())
}

fn plug(var: String) -> oxi::Result<()> {
    let p = Path::new(var.as_str());
    if !p.exists() {
        warn!("plug path {} not exists, ignore",p.display());
        return Ok(());
    }
    let rtp = api::get_option::<oxi::String>("runtimepath")?;

    info!("runtimepath add {}",var);
    let rtp = vec![rtp.to_string(), var].join(",");
    api::set_option("runtimepath", oxi::String::from(rtp))?;
    Ok(())
}

fn get_tags(repo: String) -> oxi::Result<Vec<String>> {
    Ok(tokio_block!(github::get_tags(repo.as_str())))
}

fn setup(conf: Config) -> oxi::Result<()> {
    simple_logging::log_to_file(conf.log_dir.clone(), LevelFilter::Info);
    Ok(())
}

#[oxi::module]
fn myvim() -> oxi::Result<Dictionary> {
    // 如何传入setup()或者setup({})
    setup(Config::default()).expect("myvim setup failed");
    Ok(Dictionary::from_iter([
        ("get_tags", Object::from(Function::from_fn(get_tags))),
        // fun: Fn(A) -> Result<R, E> + 'static,
        // A: Poppable,
        // R: Pushable,
        ("setup", Object::from(Function::from_fn(setup))),
        ("hello", Object::from(Function::from_fn(hello_world))),
        ("plug", Object::from(Function::from_fn(plug))),
    ]))
}
