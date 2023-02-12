mod config;
mod error;
mod plug;
mod xtokio;

use std::borrow::{Borrow, BorrowMut};
use std::env::set_var;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, RwLock};
use lazy_static::lazy_static;

use log::{debug, error, info, LevelFilter, warn};
use neovim_lib::session::ClientConnection;

use nvim_oxi::{self as oxi, print, Dictionary, Function, Error};
use oxi::api;
use oxi::Object;

use crate::plug::github;
use crate::config::Config;
use crate::plug::plug::PlugManager;

struct MyVim {
    plug_manager: PlugManager,
}

lazy_static! {
static ref CLIENT: RwLock<MyVim> = RwLock::new(client_setup(Config::default()));
}

// debug func
fn hello_world(_: ()) -> oxi::Result<()> {
    print!("hello world");
    info!("hello world log");
    Ok(())
}

fn plug_add(plug: String) -> oxi::Result<()> {
    let mut client = CLIENT.write().unwrap();
    client.plug_manager.add_plug(plug.as_str());
    Ok(())
}

fn plug_install(_: ()) -> oxi::Result<()> {
    let client = CLIENT.read().unwrap();
    client.plug_manager.plug_install();
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


fn client_setup(conf: Config) -> MyVim {
    simple_logging::log_to_file(conf.log_dir.clone(), LevelFilter::Info).expect("set log failed");
    info!("setup conf {:?}",conf);
    MyVim { plug_manager: PlugManager::new(conf.plug_dir.as_str()) }
}

fn setup(conf: Config) -> oxi::Result<()> {
    let mut c = CLIENT.write().unwrap();
    *c = client_setup(conf);
    Ok(())
}

#[oxi::module]
fn myvim() -> oxi::Result<Dictionary> {
    Ok(Dictionary::from_iter([
        ("get_tags", Object::from(Function::from_fn(get_tags))),
        // fun: Fn(A) -> Result<R, E> + 'static,
        // A: Poppable,
        // R: Pushable,
        ("setup", Object::from(Function::from_fn(setup))),
        ("hello", Object::from(Function::from_fn(hello_world))),
        // ("plug", Object::from(Function::from_fn(plug))),
        ("plug", Object::from(Function::from_fn(plug_add))),
        ("plug_install", Object::from(Function::from_fn(plug_install))),
    ]))
}
