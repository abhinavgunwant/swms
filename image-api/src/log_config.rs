use std::path::Path;

use log::{ warn, LevelFilter };
use log4rs::{
    append::console::ConsoleAppender, config::{ Appender, Config, Root },
    encode::pattern::PatternEncoder, init_config, init_file
};

const DEFAULT_LOG_FILE: &str = "log4rs.yml";

pub fn init_logger() {
    if Path::new(DEFAULT_LOG_FILE).exists() {
        init_file(DEFAULT_LOG_FILE, Default::default()).unwrap();
    } else {
        let date_pattern_encoder = Box::new(
            PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} | {h({({l}):5.5})} | {f:>20.128}:{L:4.7} - {m}{n}")
        );

        let stdout = ConsoleAppender::builder()
            .encoder(date_pattern_encoder)
            .build();

        let stdout_appender = Appender::builder()
            .build("stdout", Box::new(stdout));

        let config = Config::builder()
            .appender(stdout_appender)
            .build(Root::builder().appender("stdout").build(LevelFilter::Info))
            .unwrap();

        init_config(config).unwrap();

        warn!("Logging to `stdout`");
        warn!("Please refer to the \"logging\" section in the documentation");
    }
}

