use bevy::{prelude::*, utils::Uuid};

use crate::common::{logic::GameLogicPlugin, networking::schema::Player, config::Config};

use self::{menus::MenusPlugin, graphical::GraphicalPlugin, networking::ClientNetworkPlugin};

pub mod graphical;
pub mod menus;
pub mod networking;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_state::<ClientState>()
            // TODO: Make this dynamic
            .add_plugin(MenusPlugin)
            .add_plugin(GameLogicPlugin)
            .add_plugin(GraphicalPlugin)
            .add_plugin(ClientNetworkPlugin)
            .add_startup_system(create_player);
    }
}

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientState {
    #[default]
    MainMenu,
    Game
}

fn create_player(mut commands: Commands, conf: Res<Config>) {
    commands.spawn(Player {
        username: conf.client_config.as_ref().unwrap().username.clone(),
        id: Uuid::new_v4(),
    });
}