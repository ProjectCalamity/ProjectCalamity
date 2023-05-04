use bevy::{prelude::*, utils::tracing::log::info};
use bevy_quinnet::{client::{Client, connection::ConnectionConfiguration, certificate::CertificateVerificationMode, QuinnetClientPlugin}};

use crate::{common::{config::Config, networking::schema::{ServerMessages, ClientMessages, Player}, logic::{TileInfo, UnitAction, Unit, TileFeature}}};

use super::graphical::inputs::TurnCompletedEvent;

pub struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(QuinnetClientPlugin::default())
            .add_startup_system(open_connection)
            .add_system(handle_server_messages)
            .add_system(send_unit_actions);
    }
}

fn open_connection(mut client: ResMut<Client>, config: Res<Config>) {
    let connection_string = &config.connection_address;
    client.open_connection(
        ConnectionConfiguration::from_strings(
            &connection_string, 
            "0.0.0.0:0"
        ).unwrap(), 
        CertificateVerificationMode::SkipVerification
    ).unwrap();
    println!("Connection started to {:?}", connection_string);
}

fn send_unit_actions(mut commands: Commands, actions: Query<(Entity, &UnitAction)>, client: ResMut<Client>, mut turn_evr: EventReader<TurnCompletedEvent>, player: Query<&Player>) {
    turn_evr.iter().for_each(|_| {
        let action_vec = actions.iter().map(|(_, a)| a.clone()).collect::<Vec<UnitAction>>();
        client.connection().send_message(ClientMessages::MoveActionPacket { unit_action: action_vec.clone() }).unwrap();
        actions.iter().for_each(|(e, _a)| commands.entity(e).despawn_recursive());
    });
}

fn handle_server_messages(
    mut commands: Commands, 
    mut client: ResMut<Client>, 
    players_q: Query<(Entity, &Player)>,
    units_q: Query<(Entity, &Unit)>,
    tiles_q: Query<(Entity, &TileInfo)>,
    tile_features_q: Query<(Entity, &TileFeature)>,
) {
    while let Some(msg) = client.connection_mut().try_receive_message::<ServerMessages>() {
        match msg {
            ServerMessages::CompleteGameStatePacket { tiles, units, players, gameboard } => {
                // Despawn all tiles, players, units and features
                tiles_q.iter().for_each(|(e, _)| commands.entity(e).despawn_recursive());
                tile_features_q.iter().for_each(|(e, _)| commands.entity(e).despawn_recursive());
                units_q.iter().for_each(|(e, _)| commands.entity(e).despawn_recursive());
                players_q.iter().for_each(|(e, _)| commands.entity(e).despawn_recursive());

                tiles.iter().for_each(|tile| { 
                    // Tiles
                    commands.spawn(TileInfo { pos: tile.pos, geography: tile.geography, visible_to_players: Vec::new() })
                        .insert(Name::new(format!("Tile at ({:?})", tile.pos))); // For rendering purposes, we can ignore visible_to_players
                    
                    // Features
                    if let Some(feature) = tile.visible_features.clone() {
                        commands.spawn(feature)
                            .insert(Name::new(format!("Tile Feature at ({:?})", tile.pos))); // For rendering purposes, we can ignore visible_to_players
                    }
                });
                // Units
                units
                    .iter()
                    .for_each(|u| { 
                        commands
                            .spawn(u.clone())
                            .insert(Name::new(format!("Unit at [{:?}]", u.pos)));
                    });
                // Gameboard
                commands.spawn(gameboard.clone()).insert(Name::new("Gameboard"));

                // TODO: Spawn players
            }
            ServerMessages::PlayerTileInfoPacket { tile } => {
                // Tile
                commands.spawn(TileInfo { pos: tile.pos, geography: tile.geography, visible_to_players: Vec::new() })
                    .insert(Name::new(format!("Tile at ({:?})", tile.pos))); // For rendering purposes, we can ignore visible_to_players
                // Features
                if let Some(feature) = tile.visible_features.clone() {
                    commands.spawn(feature)
                        .insert(Name::new(format!("Tile Feature at ({:?})", tile.pos))); // For rendering purposes, we can ignore visible_to_players
                }
            },
            ServerMessages::UnitModifyPacket { prev_pos, unit } => {
                units_q
                    .iter()
                    .filter(|(_e, u)| u.pos == prev_pos)
                    .for_each(|(e, _u)| commands.entity(e).despawn());
                
                commands
                    .spawn(unit.clone())
                    .insert(Name::new(format!("Unit at ({:?})", unit.pos)));
            },
            ServerMessages::UnitRemovePacket { pos } => {
                units_q
                    .iter()
                    .filter(|(_e, u)| u.pos == pos)
                    .for_each(|(e, _u)| commands.entity(e).despawn());
            },
            ServerMessages::UnitAddPacket { unit } => {
                commands
                    .spawn(unit.clone())
                    .insert(Name::new(format!("Unit at ({:?})", unit.pos)));

            },
            ServerMessages::ChatMessagePacket { player, contents } => info!("{:?} » {:?}", player.username, contents),
            ServerMessages::PlayerInfo { player } => info!("Recieved player packet from server."),
            ServerMessages::PlayerInfoRequestPacket => {
                client.connection().send_message(ClientMessages::ConnectionPacket { player: players_q.single().1.clone() }).unwrap();
            },
            ServerMessages::DisconnectionPacket { message } => info!("Disconnected from server: {:?}", message),
            ServerMessages::TestPacket { message } => {
            }
        }
    }
}