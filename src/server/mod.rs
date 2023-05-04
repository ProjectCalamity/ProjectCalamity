pub mod networking;
pub mod logic;
pub mod console;

use std::time::Duration;
use bevy::{prelude::*, log::LogPlugin, utils::{HashMap, Uuid}, time::TimePlugin};

use crate::{common::{logic::{GameLogicPlugin, PlayerTeam, gameboard_gen::generate_gameboard}, networking::schema::Player, config::Config}, server::logic::TurnTimer};
use self::{networking::{ServerNetworkPlugin, send_gameboard}, logic::ServerLogicPlugin};


pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ServerGameManager::default())
            .add_state::<ServerState>()
            .add_plugin(GameLogicPlugin)
            .add_plugin(ServerLogicPlugin)
            .add_plugin(LogPlugin::default())
            .add_plugin(TimePlugin)
            .add_plugin(ServerNetworkPlugin)
            .set_runner(server_runner)
            .add_system(manage_lobby)
            .add_system(generate_gameboard.in_set(OnUpdate(ServerState::Game)))
            .add_system(send_gameboard);
    }
}

pub fn server_runner(mut app: App) {
    loop {
        app.update()
    }
}

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ServerState {
    #[default]
    Lobby,
    Game
}

#[derive(Debug, Default, Resource)]
pub struct ServerGameManager {
    pub players: Vec<(Player, PlayerTeam)>
}

impl ServerGameManager {
    pub fn client_id(&self, team: &PlayerTeam, clients: &HashMap<u64, Uuid>) -> u64 {
        return clients
            .iter()
            .filter(|(_id, uuid)| uuid == &&self.players
                .iter()
                .filter(|(_p, pt)| pt == team)
                .collect::<Vec<_>>()
                [0]
                .0
                .id
            ).collect::<Vec<_>>()
            [0]
            .0
            .clone();
    }
}

pub fn manage_lobby(
    mut commands: Commands,
    mut server_state: ResMut<State<ServerState>>, 
    game_manager: Res<ServerGameManager>, 
    config: Res<Config>,
) {
    let sc = config.server_config.as_ref().unwrap();
    if server_state.0 == ServerState::Lobby && game_manager.players.len() == sc.max_players as usize {
        server_state.0 = ServerState::Game;
        game_manager.players.iter().for_each(|(p, pt)| {
            commands.spawn(p.clone()).insert(pt.clone()).insert(Name::new(format!("Player {:?}", p.username)));
        });
        info!("Lobby is full, starting game");
        commands.spawn(TurnTimer { 
            timer: Timer::new(Duration::from_secs(60), TimerMode::Repeating) 
        });
    }
}