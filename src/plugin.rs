use neovim_lib::{Neovim, NeovimApi, Session};
use std::collections::HashMap;

pub struct Plugin {
    pub nvim: Neovim,
    handlers: HashMap<String, fn(&mut Plugin)>,
}

impl Plugin {
    pub fn New() -> Plugin {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let handlers = HashMap::new();
        return Plugin { nvim, handlers };
    }
    pub fn Start(&mut self) {
        let recv = self.nvim.session.start_event_loop_channel();
        for (event, values) in recv {
            let f = self.handlers[&event];
            f(self);
        }
    }
    pub fn Handle(&mut self, msg: String, handler: fn(&mut Plugin)) {
        self.handlers.insert(msg, handler);
    }
}
