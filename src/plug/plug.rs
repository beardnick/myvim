use std::error::Error;
use std::{fs};
use std::fmt::format;
use std::path::Path;
use git2::Repository;
use log::info;

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
        for p in self.plugs.iter() {
            info!("clone plug {} into {}",p,self.plug_dir);
            Repository::clone(
                format!("{}/{}.git", self.base_url.as_str(), p.as_str()).as_str(),
                Path::new(&format!("{}/{}", self.plug_dir, p)),
            )?;
        }
        Ok(self)
    }
}