use bevy::{prelude::*, utils::HashMap};
use bevy_quinnet::server::Server;

use crate::common::logic::{Unit, TileInfo, TileFeature, TurnExecuteStages, UnitActions};

use super::{networking::{PlayerMoves, ClientIDMap}, ServerGameManager, ServerState};

pub struct ServerLogicPlugin;

impl Plugin for ServerLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TurnReadyEvent>()
            .add_system(check_turn_execute_readiness)
            .add_system(execute_turn);
    }
}

pub struct TurnReadyEvent;

#[derive(Component, Default, FromReflect, Reflect)]
pub struct TurnTimer {
    pub timer: Timer
}

fn check_turn_execute_readiness(
    mut turn_ready_evw: EventWriter<TurnReadyEvent>,
    time: Res<Time>,
    mut turn_timer_q: Query<&mut TurnTimer>,
    player_moves: Res<PlayerMoves>,
    server_manager: ResMut<ServerGameManager>,
    state: Res<State<ServerState>>
) {
    if state.0 == ServerState::Game {
        for mut turn_timer in turn_timer_q.iter_mut() {
            turn_timer.timer.tick(time.delta());
            if turn_timer.timer.finished() {
                info!("Duration has elapsed, move is executing");
                turn_ready_evw.send(TurnReadyEvent);
            }
        }
        if player_moves.map.len() == server_manager.players.len() {
            info!("All players have sent turn, move executing");
            turn_ready_evw.send(TurnReadyEvent);
            turn_timer_q.single_mut().timer.reset();
        }
    }
}

fn execute_turn(
    turn_ready_evr: EventReader<TurnReadyEvent>,
    mut player_moves: ResMut<PlayerMoves>,
    server: Res<Server>,
    mut units: Query<&mut Unit>,
    mut tiles: Query<&mut TileInfo>,
    mut tile_features: Query<&mut TileFeature>,
    game_manager: Res<ServerGameManager>,
    clients: Res<ClientIDMap>
) {
    if turn_ready_evr.len() <= 0 {
        return;
    }

    let stages = [TurnExecuteStages::PreTurn, TurnExecuteStages::MidTurn, TurnExecuteStages::AfterTurn];

    let mut units_vec = units.iter_mut().collect::<Vec<_>>();
    let mut tiles_vec = tiles.iter_mut().collect::<Vec<_>>();
    let mut tile_features_vec = tile_features.iter_mut().collect::<Vec<_>>();

    let mut actions = Vec::new();

    for (_team, player_actions) in &player_moves.map {
        player_actions
            .iter()
            .for_each(|a| actions.push(a.clone()));
    }

    info!("Unit actions: {:?}", actions.iter().collect::<Vec<_>>());

    for stage in stages {
        for action in actions.iter() {
            if action.turn_stage.0 == stage {
                // Move
                if action.action_type == UnitActions::Move {
                    if actions
                    .iter()
                    .filter(|a| a.action_pos == action.action_pos)
                    .collect::<Vec<_>>()
                    .len() == 1 {
                        // Action doesn't conflict with any other action
                        action.apply(
                            &server, 
                            &mut units_vec,
                            &mut tiles_vec,
                            &mut tile_features_vec,
                            &game_manager,
                            &clients,
                        );
                    } else {
                        info!("Bouncing moves to {:?} due to movement conflict", action.action_pos);
                    }
                    // Otherwise, bounce (i.e. do nothing)
                }
                
                // Special
                // TODO

                // Attack
                if action.action_type == UnitActions::Attack {
                    if &units_vec
                        .iter()
                        .filter(|u| u.pos == action.action_pos)
                        .collect::<Vec<_>>()
                        .len() == &(1 as usize) {
                        // There's a unit to attack
                        action.apply(
                            &server, 
                            &mut units_vec,
                            &mut tiles_vec,
                            &mut tile_features_vec,
                            &game_manager,
                            &clients,
                        );
                    }
                    // Otherwise, the attack doesn't go through.
                }
            }
        }
    }
    player_moves.map = HashMap::new();
}