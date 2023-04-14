mod common;
mod client;
mod server;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use common::{logic::GameLogicPlugin, config::{Config, RunEnvironment}};
use client::graphical::GraphicalPlugin;

fn main() {

    let config = Config::load();

    // Server startup
    match config.env {
        RunEnvironment::Client => {
            App::new()
                .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
                .add_plugin(GameLogicPlugin)
                .add_plugin(GraphicalPlugin)
                .run();
        },
        RunEnvironment::ClientDebug => {
            App::new()
                .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
                .add_plugin(WorldInspectorPlugin::default())
                .add_plugin(GameLogicPlugin)
                .add_plugin(GraphicalPlugin)
                .run();
        },
        RunEnvironment::Server => {
            App::new()
                .add_plugin(GameLogicPlugin)
                .run();

        },
    };
}