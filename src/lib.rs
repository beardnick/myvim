mod config;
mod error;
mod plug;
mod xtokio;
mod client;

use std::cell::RefCell;
use std::iter::FromIterator;
use std::rc::Rc;

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
use crate::client::Client;

use crate::plug::github;
use crate::config::Config;


fn get_tags(repo: String) -> oxi::Result<Vec<String>> {
    Ok(tokio_block!(github::get_tags(repo.as_str())))
}

fn setup(preference: &RefCell<Client>, conf: Config) -> oxi::Result<()> {
    let mut client  = preference.borrow_mut();
    client.conf = conf;
    simple_logging::log_to_file(client.conf.log_dir.clone(), LevelFilter::Info);
    Ok(())
}

#[oxi::module]
fn myvim() -> oxi::Result<Dictionary> {

    let mut client = RefCell::new(Client::default());

    let pub_setup = move |conf: Config| { setup( &client, conf) };
    Ok(Dictionary::from_iter([
        ("get_tags", Object::from(Function::from_fn(get_tags))),
        ("setup", Object::from(Function::from_fn(pub_setup))),
    ]))
}
