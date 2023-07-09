mod client;
mod common;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use client::{ClientPlugin, SingleplayerPlugin};
use common::config::{Config, RunEnvironment};

fn main() {
    let config = Config::load();

    let mut app = App::new();
    // Server startup
    match config.env {
        RunEnvironment::Singleplayer | RunEnvironment::Client => {
            app.add_plugin(ClientPlugin);
            if config.debug {
                app.add_plugin(WorldInspectorPlugin::default());
            }
            if config.env == RunEnvironment::Singleplayer {
                app.add_plugin(SingleplayerPlugin);
            }
        }
        RunEnvironment::Server => panic!("Server is currently not supported"),
    };
    app.insert_resource(config).run();
}
