use std::fs;

use bevy::prelude::warn;
use toml::Table;

#[derive(Debug, Default)]
pub struct Config {
    pub env: RunEnvironment,
    pub debug: bool
}

impl Config {
    pub fn load() -> Self {

        let mut config = Config { ..Default::default() };

        match fs::read_to_string("./config.toml") {
            Ok(conf_str) => {
                let toml = conf_str.parse::<Table>().unwrap();
                let conf_toml = &toml["configuration"];
                
                // Environment
                match conf_toml["environment"].as_str() {
                    Some("client") => {config.env = RunEnvironment::Client},
                    Some("server") => {config.env = RunEnvironment::Server},
                    _ => {warn!("Unable to read environment due to an invalid character sequence. Continuing with default values.")}
                }

                // Debug
                match conf_toml["debug"].as_str() {
                    Some("true") => {config.debug = true},
                    Some("false") => {config.debug = false},
                    _ => {warn!("Unable to read environment due to an invalid character sequence. Continuing with default values.")}
                }
            },
            Err(err) => {
                warn!("Unable to read config data [{:?}]. Continuing with default values.", err);
            }
        };

        return config;

    }
}

#[derive(Debug, Default)]
pub enum RunEnvironment {
    #[default]
    Client,
    Server
}