use bevy::{prelude::*, utils::{HashMap, Uuid}};
use bevy_quinnet::{server::{QuinnetServerPlugin, Server, ServerConfiguration, certificate::CertificateRetrievalMode}};

use crate::common::{networking::schema::{ClientMessages, ServerMessages, SentPlayerInfoRequestPacket, PlayerTileInfo, Player}, config::Config, logic::{PlayerTeam, TeamColour, TileInfo, TileFeature, Gameboard, Unit, Geography, UnitAction}};

use super::{ServerState, ServerGameManager};

pub struct ServerNetworkPlugin;

impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(QuinnetServerPlugin::default())
            .init_resource::<ClientIDMap>()
            .init_resource::<NetworkState>()
            .init_resource::<PlayerMoves>()
            .add_startup_system(start_listener)
            .add_system(handle_client_messages);
    }
}

#[derive(Default, Resource)]
pub struct PlayerMoves {
    pub map: HashMap<PlayerTeam, Vec<UnitAction>>
}

#[derive(Default, Resource)]
pub struct ClientIDMap {
    pub map: HashMap<u64, Uuid>
}

#[derive(Default, Resource)]
pub struct NetworkState {
    sent_init_gameboard: bool
}

fn start_listener(mut server: ResMut<Server>, config: Res<Config>) {
    let connection_string = &config.connection_address;
    server.start_endpoint(
        ServerConfiguration::from_string(connection_string).unwrap(),
        CertificateRetrievalMode::GenerateSelfSigned { server_hostname: connection_string.to_string() }
    ).unwrap();
    println!("Server endpoint started at {:?}", connection_string);
    info!("Server endpoint started at {:?}", connection_string);
}

fn handle_client_messages(
    mut commands: Commands, 
    mut server: ResMut<Server>, 
    mut clients: ResMut<ClientIDMap>, 
    sent_q: Query<&SentPlayerInfoRequestPacket>,
    server_state: Res<State<ServerState>>,
    mut game_manager: ResMut<ServerGameManager>,
    mut player_moves: ResMut<PlayerMoves>
) {
    let endpoint = server.endpoint_mut();
    if server_state.0 == ServerState::Lobby {
        for client_id in endpoint.clients() {
            if let None = clients.map.get(&client_id) {
                if sent_q.iter().filter(|c| c.0 == client_id).collect::<Vec<&SentPlayerInfoRequestPacket>>().len() == 0 {
                    info!("Requesting information from {:?}", &client_id);
                    endpoint.send_message(client_id, ServerMessages::PlayerInfoRequestPacket).unwrap();
                    commands.spawn(SentPlayerInfoRequestPacket(client_id));
                }
            }
        }
    }

    for client_id in endpoint.clients() {
        let client_messages = endpoint.try_receive_message_from::<ClientMessages>(client_id);
        if let Some(message) = client_messages {
            match message {
                ClientMessages::ChatMessagePacket { player, contents } => info!("{:?} » {:?}", player.username, contents),
                ClientMessages::MoveActionPacket { unit_action } => {
                    let team = &game_manager
                        .players
                        .iter()
                        .filter(|(p, _pt)| 
                            &p.id == clients
                                .map
                                .get(&client_id)
                                .unwrap()
                        ).collect::<Vec<&(Player, PlayerTeam)>>()
                        [0].1;
                    if player_moves
                        .map
                        .keys()
                        .filter(|pt| pt == &team)
                        .collect::<Vec<_>>().len() == 0 {
                        info!("Adding moves: {:?} for team {:?}", unit_action, team);
                        player_moves.map.insert(team.clone(), unit_action);
                    }
                },
                ClientMessages::ConnectionPacket { player } => {
                    if clients.map.iter().filter(|(_id, uuid)| uuid == &&player.id).collect::<Vec<_>>().len() == 0 {
                        info!("{:?}[{:?}] connected", player.username, player.id);
                        game_manager.players.push((player.clone(), PlayerTeam(TeamColour::from_int(&clients.map.len()))));
                        clients.map.insert(client_id, player.id);
                    } else {
                        endpoint.send_message(client_id, ServerMessages::DisconnectionPacket { message: "Attempted to join, but was already connected".to_string() }).unwrap();
                        info!("{:?}[{:?}] attempted connect, but was already connected", player.username, player.id);
                        println!("{:?}", clients.map);
                    }
                },
                ClientMessages::DisconnectionPacket { player } => info!("{:?} disconnected", player.username),
            }
        }
    }
}

pub fn send_gameboard(
    mut server: ResMut<Server>, 
    clients: Res<ClientIDMap>,
    tiles: Query<&TileInfo>,
    tile_features: Query<&TileFeature>,
    players: Query<(&Player, &PlayerTeam)>,
    units: Query<&Unit>,
    mut network_state: ResMut<NetworkState>,
    gameboard_q: Query<&Gameboard>
) {
    if !network_state.sent_init_gameboard && tiles.iter().len() > 0 {
        
        network_state.sent_init_gameboard = true;

        info!("Sending gameboard to players");

        players.iter().for_each(|(p, pt)| {
            let eqiv_uuid = clients
                .map
                .iter()
                .filter(|(_id, uuid)| uuid == &&p.id)
                .collect::<Vec<(&u64, &Uuid)>>()
                [0];
            
            let tiles_vec = tiles
                .iter()
                .map(|ti| 
                    return ti.player_tile_info(
                        pt.clone(), 
                        tile_features
                            .iter()
                            .filter(|tf| tf.pos == ti.pos)
                            .collect::<Vec<&TileFeature>>()
                            .get(0)
                            .clone()
                    )
                ).collect::<Vec<PlayerTileInfo>>();

            let pos_vec = tiles_vec
                .iter()
                .filter(|t| t.geography != Geography::Fog)
                .map(|u| u.pos)
                .collect::<Vec<[i32; 2]>>();

            server
                .endpoint_mut()
                .send_message(
                    eqiv_uuid.0.clone(), 
                    ServerMessages::CompleteGameStatePacket { 
                        tiles: tiles_vec, 
                        units: units.iter().filter(|u| pos_vec.contains(&u.pos)).map(|u| u.clone()).collect::<Vec<Unit>>(), 
                        players: Vec::new(),
                        gameboard: gameboard_q.single().clone()
                    }
                ).unwrap();
        })
    }
}