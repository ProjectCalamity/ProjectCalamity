use std::{thread, io::stdin, sync::mpsc::{Receiver, self}};
use bevy::{prelude::*, utils::Uuid};
use bevy_quinnet::server::Server;

use crate::common::{networking::schema::Player, logic::{TileInfo, TileFeature}};

use super::{ServerGameManager, networking::ClientIDMap};

/*

    PROJECT CALAMITY COMMAND TREE

    debug
        players -> Returns players and their corresponding UUIDs
        reveal <uuid> -> Reveals the entire map for a player

*/

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DebugPlayersEvent>()
            .add_event::<DebugRevealEvent>()
            .insert_non_send_resource(start_console())
            .add_system(debug_players)
            .add_system(parse_command_input);
    }
}

pub struct ConsoleReciever(Receiver<String>);

// Events - by command required to trigger
pub struct DebugPlayersEvent;
pub struct DebugRevealEvent(Uuid);

pub fn start_console() -> ConsoleReciever {

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        tx.send(buf).unwrap();
    });

    return ConsoleReciever(rx);
}


pub fn parse_command_input(
    rx: NonSend<ConsoleReciever>,
    // ha. ha. ha.
    mut debug_players_evw: EventWriter<DebugPlayersEvent>,
    mut debug_reveal_evw: EventWriter<DebugRevealEvent>,
) {

    if let Ok(input) = rx.0.try_recv() {
        // First, excise the `\n` at the end of the line
        let input = input.split_at(input.len() - 1).0;
        let sub_commands = input.split_whitespace().collect::<Vec<_>>();
        if sub_commands.len() >= 1 {
            match sub_commands[0] {
                "debug" => {
                    if sub_commands.len() >= 2 {
                        match sub_commands[1] {
                            "players" => {
                                debug_players_evw.send(DebugPlayersEvent);
                            },
                            "reveal" => {
                                if sub_commands.len() == 3 {
                                    if let Ok(uuid) =  Uuid::parse_str(sub_commands[2]) {
                                        debug_reveal_evw.send(DebugRevealEvent(uuid));
                                    } else {
                                        warn!("Attempted to reveal entire map for a non-existant player");
                                    }
                                } else {
                                    warn!("Invalid synax: `reveal` requires a specified uuid");
                                }
                            }
                            _ => {
                                warn!("Attempted to execute an invalid command: {:?} not found", sub_commands[1]);
                            }
                        }
                    } else {
                        warn!("Invalid synax: `debug` requires a subcommand");
                    }
                }
                _ => {
                    warn!("Attempted to execute an invalid command: {:?} not found", sub_commands[0]);
                }
            }
        }
    }
}

fn debug_players(mut evr: EventReader<DebugPlayersEvent>, players: Query<&Player>) {
    evr.iter().for_each( |_| {
        if players.iter().len() > 0 {
            info!("Listing players");
            players.iter().for_each(|p| println!("  -> {:?} - {:?}", p.username, p.id));
        } else {
            info!("No players are connected");
        }
    });
}

fn debug_reveal(
    mut evr: EventReader<DebugRevealEvent>, 
    mut tiles: Query<&mut TileInfo>, 
    features: Query<&TileFeature>,
    players: Query<&Player>,
    sgm: Res<ServerGameManager>,
    cidm: Res<ClientIDMap>,
    server: Res<Server>
) {
    evr.iter().for_each(|dre| {

        match players
            .iter()
            .filter(|p| p.id == dre.0)
            .collect::<Vec<_>>()
            .get(0) {
            Some(player) => {
                let player_team = &sgm
                    .players
                    .iter()
                    .filter(|(p, _pt)| p.id == player.id)
                    .collect::<Vec<_>>()
                    [0]
                    .1;

                let client_id = cidm
                    .map
                    .iter()
                    .filter(|(_id, p)| p == &&player.id)
                    .collect::<Vec<_>>()
                    [0]
                    .0
                    .clone();

                tiles
                    .iter_mut()
                    .for_each(|mut t| {
                        
                        let feature = match features
                            .iter()
                            .filter(|f| f.pos == t.pos)
                            .collect::<Vec<_>>()
                            .get(0) {
                            Some(f) => Some(f.clone().clone()),
                            None => None,
                        };
                        t.reveal(player_team.clone(), client_id, &server, feature);
                    });
                info!("Revealed the entire map for player {:?}", player.id);
            },
            None => {
                warn!("Attempted to reveal board for non existent player {:?}", dre.0);
            }
        }
    })
}