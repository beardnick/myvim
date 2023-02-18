use std::error::Error;
use std::{fs, thread};
use std::fmt::format;
use std::path::Path;
use futures::future::join_all;
use futures::TryFutureExt;
use git2::Repository;
use log::{error, info};
use tokio::runtime::Runtime;
use crate::error::PlugError::CloneFailed;

pub struct PlugManager {
    plug_dir: String,
    plugs: Vec<String>,
    base_url: String,
}

impl PlugManager {
    pub fn new(plug_dir: &str) -> Self {
        Self {
            plug_dir: shellexpand::tilde(plug_dir).to_string(),
            plugs: vec![],
            base_url: String::from("https://github.com"),
        }
    }
    pub fn add_plug(&mut self, s: &str) -> &mut Self {
        info!("add plug {}",s);
        self.plugs.push(String::from(s));
        self
    }
    pub fn plug_install(&self) -> Result<&Self, Box<dyn Error>> {
        let path = Path::new(&self.plug_dir);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        let mut tasks: Vec<_> = vec![];
        for p in self.plugs.iter() {
            let plug_dir = self.plug_dir.clone();
            let base_url = self.base_url.clone();
            let plug_path = p.clone();
            tasks.push(thread::spawn(move || {
                info!("clone plug {} into {} start",plug_path,plug_dir);
                match Repository::clone(
                    format!("{}/{}.git", base_url, plug_path).as_str(),
                    Path::new(&format!("{}/{}", plug_dir, plug_path.clone().split("/").collect::<Vec<&str>>()[1])),
                ) {
                    Ok(_) => {
                        info!("clone plug {} into {} done",plug_path,plug_dir);
                    }
                    Err(e) => {
                        error!("clone err {}",e);
                    }
                }
            }));
        }
        for x in tasks {
            match x.join() {
                Ok(_) => {}
                Err(e) => {
                    return Err(Box::new(CloneFailed(format!("{:?}",e))));
                }
            };
        }
        info!("all clone done");
        Ok(self)
    }
}