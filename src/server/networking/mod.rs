use bevy::{prelude::*, utils::{HashMap, Uuid}};
use bevy_quinnet::server::{QuinnetServerPlugin, Server, ServerConfiguration, certificate::CertificateRetrievalMode};

use crate::common::{networking::schema::{ClientMessages, ServerMessages, SentPlayerInfoRequestPacket, PlayerTileInfo, Player}, config::Config, logic::{PlayerTeam, TeamColour, TileInfo, TileFeature}};

use super::{ServerState, ServerGameManager, GameboardGeneratedEvent};

pub struct ServerNetworkPlugin;

impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(QuinnetServerPlugin::default())
            .init_resource::<ClientIDMap>()
            .add_startup_system(start_listener)
            .add_system(handle_client_messages);
    }
}

#[derive(Resource, Default)]
pub struct ClientIDMap {
    map: HashMap<u64, Uuid>
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
    mut game_manager: ResMut<ServerGameManager>
) {
    if server_state.0 == ServerState::Lobby {
        let endpoint = server.endpoint_mut();
        for client_id in endpoint.clients() {
            if let None = clients.map.get(&client_id) {
                if sent_q.iter().filter(|c| c.0 == client_id).collect::<Vec<&SentPlayerInfoRequestPacket>>().len() == 0 {
                    info!("Requesting information from {:?}", &client_id);
                    endpoint.send_message(client_id, ServerMessages::PlayerInfoRequestPacket).unwrap();
                    commands.spawn(SentPlayerInfoRequestPacket(client_id));
                }
            }
            while let Some(message) = endpoint.try_receive_message_from::<ClientMessages>(client_id) {
                match message {
                    ClientMessages::ChatMessagePacket { player, contents } => info!("{:?} » {:?}", player.username, contents),
                    ClientMessages::MoveActionPacket { player, unit_action } => info!("{:?} Requests Movement: {:?}", player.username, unit_action),
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
}

pub fn send_gameboard(
    server: Res<Server>, 
    clients: Res<ClientIDMap>,
    mut gg_evr: EventReader<GameboardGeneratedEvent>,
    tiles: Query<&TileInfo>,
    tile_features: Query<&TileFeature>,
    players: Query<(&Player, &PlayerTeam)>
) {
    gg_evr.iter().for_each(|_gge| {
        info!("Sending gameboard to players");
        players.iter().for_each(|(p, pt)| {
            let eqiv_uuid = clients.map.iter().filter(|(_id, uuid)| uuid == &&p.id).collect::<Vec<(&u64, &Uuid)>>()[0];
            let tiles_vec = tiles.iter().map(|ti| {
                let eqiv_tf = tile_features.iter().filter(|tf| tf.pos == ti.pos).collect::<Vec<&TileFeature>>()[0];
                return ti.player_tile_info(pt.clone(), eqiv_tf.clone());
            }).collect::<Vec<PlayerTileInfo>>();
            tiles_vec.iter().for_each(|tile| server.endpoint().send_message(eqiv_uuid.0.clone(), tile.clone()).unwrap());
        })
    });

}