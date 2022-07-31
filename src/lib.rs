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

// fn setup(client: &mut Client, conf: Config) -> oxi::Result<()> {
//     client.conf = conf;
//     simple_logging::log_to_file(client.conf.log_dir.clone(), LevelFilter::Info);
//     Ok(())
// }

// fn setup(state: &Rc<RefCell<Client>>, conf: Config) -> oxi::Result<()> {
//     let mut client  = state.borrow_mut();
//     client.conf = conf;
//     simple_logging::log_to_file(client.conf.log_dir.clone(), LevelFilter::Info);
//     Ok(())
// }

fn setup(state: &RefCell<Client>, conf: Config) -> oxi::Result<()> {
    let mut client  = state.borrow_mut();
    client.conf = conf;
    simple_logging::log_to_file(client.conf.log_dir.clone(), LevelFilter::Info);
    Ok(())
}

#[oxi::module]
fn myvim() -> oxi::Result<Dictionary> {
    // setup后无法进行修改 可以共享
    // let client = Rc::new(Client { conf: Config { log_dir: String::from("/tmp/myvim.log") } });

    // 使用&RefCell<Client>可以通过编译,setup后可以修改
    let mut client = RefCell::new(Client::default());

    // 使用&client后不能,使用&mut client后闭包变成FnOnce,无法通过编译
    // let mut client = Client { conf: Config { log_dir: String::from("/tmp/myvim.log") } };
    
    // 可以x
    // let mut client = Rc::new(RefCell::new(Client { conf: Config { log_dir: String::from("/tmp/myvim.log") } }));

    let pub_setup = move |conf: Config| { setup( &client, conf) };
    Ok(Dictionary::from_iter([
        ("get_tags", Object::from(Function::from_fn(get_tags))),
        ("setup", Object::from(Function::from_fn(pub_setup))),
    ]))
}
