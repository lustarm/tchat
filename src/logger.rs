use env_logger;

/* == Helper == */
pub fn logger_init() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
}

