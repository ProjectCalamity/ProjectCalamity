use std::{thread, io::stdin, sync::mpsc::{Receiver, self}};
use bevy::{prelude::*, utils::Uuid};

use crate::common::{networking::schema::Player, logic::{TileInfo, TileFeature}};

use super::{ServerGameManager, networking::{ClientIDMap, SendGameboardEvent, PlayerMoves}};

/*

    PROJECT CALAMITY COMMAND TREE

    debug
        help -> Lists all available commands
        players -> Returns players and their corresponding UUIDs
        reveal <uuid> -> Reveals the entire map for a player
        turn
            status -> Returns the status of each player's move, and the remaining time
            execute -> Force-executes the turn immidately
*/

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DebugPlayersEvent>()
            .add_event::<DebugRevealEvent>()
            .add_event::<DebugTurnExecuteEvent>()
            .add_event::<DebugTurnStatusEvent>()
            .insert_non_send_resource(start_console())
            .add_system(debug_players)
            .add_system(debug_reveal)
            .add_system(debug_turn_execute)
            .add_system(debug_turn_status)
            .add_system(parse_command_input);
    }
}

pub struct ConsoleReciever(Receiver<String>);

// Events - by command required to trigger
pub struct DebugPlayersEvent;
pub struct DebugRevealEvent(Uuid);
pub struct DebugTurnStatusEvent;
pub struct DebugTurnExecuteEvent;

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
    mut debug_turn_execute_evw: EventWriter<DebugTurnExecuteEvent>,
    mut debug_turn_status_evw: EventWriter<DebugTurnStatusEvent>,
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
                            "turn" => {
                                if sub_commands.len() >= 3 {
                                    match sub_commands[2] {
                                        "status" => {
                                            debug_turn_status_evw.send(DebugTurnStatusEvent);
                                        },
                                        "execute" => {
                                            debug_turn_execute_evw.send(DebugTurnExecuteEvent);
                                        }
                                        _ => {
                                            warn!("Attempted to execute an invalid command: {:?} not found", sub_commands[2]);
                                        }
                                    }
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
    mut send_gameboard_evw: EventWriter<SendGameboardEvent>,
    cidm: Res<ClientIDMap>,
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
                        t.reveal(player_team.clone(), feature);
                    });
                send_gameboard_evw.send(SendGameboardEvent(Some(vec![client_id])));
                info!("Revealed the entire map for player {:?}", player.id);
            },
            None => {
                warn!("Attempted to reveal board for non existent player {:?}", dre.0);
            }
        }
    })
}

fn debug_turn_execute(
    
) {

}

fn debug_turn_status(
    mut evr: EventReader<DebugTurnStatusEvent>, 
    player_moves: Res<PlayerMoves>,
    sgm: Res<ServerGameManager>
) {
    evr.iter().for_each( |_| {
        info!("Listing turn status");
        sgm.players.iter().for_each(|(p, t)| {
            let associated_move_vec = player_moves
                .map
                .iter()
                .filter(|(team, _)| team == &t)
                .collect::<Vec<_>>();
            let associated_move = associated_move_vec.get(0);
            let mut status = "Moves not submitted";
            if let Some(_) = associated_move {
                status = "Moves submitted."
            }
            println!("  -> {:?} [{:?}]: {:?}", p.username, t.0, status);
        })
    })
}