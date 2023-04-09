mod visuals;
mod game;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlaygroundPlugin;
use visuals::VisualPlaygroundPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(GamePlaygroundPlugin)
        .add_plugin(VisualPlaygroundPlugin)
        .run();
}