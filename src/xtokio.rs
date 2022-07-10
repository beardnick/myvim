#[macro_export]
macro_rules! tokio_block {
    ($func:expr) => {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on($func)
    };
}
