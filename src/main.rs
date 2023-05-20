mod common;
mod client;
mod server;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use client::{ClientPlugin, SingleplayerPlugin};
use common::config::{Config, RunEnvironment};
use server::{ServerPlugin, console::ConsolePlugin};

fn main() {

    let config = Config::load();

    let mut app = App::new();
    // Server startup
    match config.env {
        RunEnvironment::Singleplayer | RunEnvironment::Client => {
            app
                .add_plugin(ClientPlugin);
            if config.debug {
                app.add_plugin(WorldInspectorPlugin::default());
            }
            if config.env == RunEnvironment::Singleplayer {
                app.add_plugin(SingleplayerPlugin);
            }
        }
        RunEnvironment::Server => {

            app
                .add_plugin(ConsolePlugin)
                .add_plugin(ServerPlugin);
        },
    };
    app
        .insert_resource(config)
        .run();
}