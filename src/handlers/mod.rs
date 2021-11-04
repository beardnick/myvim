use crate::plugin::Plugin;

pub fn RegisterHandlers(&mut p: Plugin) {
    p.Handle("hello", |&mut p: Plugin| {
        p.nvim
            .command(&format!(`echo "hello world"`))
            .unwrap();
    })
}
