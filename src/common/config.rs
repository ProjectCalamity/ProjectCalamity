use std::fs;

use bevy::prelude::{warn, Resource};
use toml::Table;


#[derive(Debug, Default, Resource)]
pub struct Config {
    pub env: RunEnvironment,
    pub debug: bool,
    pub connection_address: String,
    pub server_config: ServerConfig,
    pub client_config: ClientConfig
}

#[derive(Debug, Default, Resource)]
pub struct ServerConfig {
    pub max_players: u32,
}

#[derive(Debug, Default, Resource)]
pub struct ClientConfig {
    pub username: String,
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
                    Some("singleplayer") => {config.env = RunEnvironment::Singleplayer}
                    _ => {warn!("Unable to read environment due to an invalid character sequence. Continuing with default values.")}
                }

                // Debug
                match conf_toml["debug"].as_str() {
                    Some("true") => {config.debug = true},
                    Some("false") => {config.debug = false},
                    _ => {warn!("Unable to read environment due to an invalid character sequence. Continuing with default values.")}
                }
                let conn_adr = conf_toml["connection_address"].to_string();

                // Oh that's some painful shit, yes daddy
                config.connection_address = conn_adr.split_at(1).1.split_at(conn_adr.len() - 2).0.to_string();

                if config.env == RunEnvironment::Server {

                    let mut server_conf = ServerConfig::default();

                    let conf_toml_server = &toml["server"];
                    let mut players = conf_toml_server["max_players"].as_integer().unwrap() as u32;
                    if players > 4 {
                        players = 4
                    } else if players < 2{
                        players = 2
                    }
                    
                    server_conf.max_players = players;
                    config.server_config = server_conf;
                } else if config.env == RunEnvironment::Client {

                    let mut client_conf = ClientConfig::default();

                    let conf_toml_client = &toml["client"];
                    client_conf.username = conf_toml_client["username"].as_str().unwrap().to_string();

                    config.client_config = client_conf;
                }
            },
            Err(err) => {
                warn!("Unable to read config data [{:?}]. Continuing with default values.", err);
            }
        };

        return config;

    }
}

#[derive(Debug, Default, PartialEq)]
pub enum RunEnvironment {
    #[default]
    Client,
    Server,
    Singleplayer
}