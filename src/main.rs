mod common;
mod client;
mod server;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use client::ClientPlugin;
use common::config::{Config, RunEnvironment};
use server::ServerPlugin;

fn main() {

    let config = Config::load();

    let mut app = App::new();
    // Server startup
    match config.env {
        RunEnvironment::Client => {
            app
                .add_plugin(ClientPlugin);
            if config.debug {
                app.add_plugin(WorldInspectorPlugin::default());
            }
        },
        RunEnvironment::Server => {
            app
                .add_plugin(ServerPlugin);
        },
    };
    app
        .insert_resource(config)
        .run();
}