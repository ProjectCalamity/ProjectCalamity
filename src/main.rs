mod common;
mod client;
mod server;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use common::{logic::GameLogicPlugin, config::{Config, RunEnvironment}};
use client::{graphical::GraphicalPlugin, ClientState, menus::MenusPlugin};

fn main() {

    let config = Config::load();

    let mut app = App::new();
    // Server startup
    match config.env {
        RunEnvironment::Client => {
            app
                .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
                .add_state::<ClientState>()
                .add_plugin(MenusPlugin)
                .add_plugin(GameLogicPlugin)
                .add_plugin(GraphicalPlugin)
        },
        RunEnvironment::Server => {
            app
                .add_plugin(GameLogicPlugin)
        },
    };

    if config.debug {
        app.add_plugin(WorldInspectorPlugin::default());
    }

    app.run();
}