use bevy::{utils::Uuid, prelude:: Component};
use serde::{Serialize, Deserialize};

use crate::common::logic::{Geography, Unit, UnitAction, Gameboard, TileFeature};

// Building blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTileInfo {
    pub pos: [i32; 2],
    pub geography: Geography,
    pub visible_features: Option<TileFeature>
}

#[derive(Clone, Debug, Deserialize, Component, Serialize)]
pub struct Player {
    pub username: String,
    pub id: Uuid
}

// Sent by server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessages {
    TestPacket {
        message: String,
    },
    CompleteGameStatePacket {
        tiles: Vec<PlayerTileInfo>,
        units: Vec<Unit>,
        players: Vec<Player>,
        gameboard: Gameboard
    },
    PlayerTileInfoPacket {
        tile: PlayerTileInfo
    },
    UnitModifyPacket {
        prev_pos: [i32; 2],
        unit: Unit
    },
    UnitRemovePacket {
        pos: [i32; 2],
    },
    UnitAddPacket {
        unit: Unit
    },
    ChatMessagePacket {
        player: Player,
        contents: String
    },
    PlayerInfo {
        player: Player
    },
    PlayerInfoRequestPacket,
    DisconnectionPacket {
        message: String
    }
}

// Sent by client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessages {
    ChatMessagePacket {
        player: Player,
        contents: String,
    },
    MoveActionPacket {
        unit_action: Vec<UnitAction>
    },
    ConnectionPacket {
        player: Player
    },
    DisconnectionPacket {
        player: Player
    }
}

// For internal use, so as not to send packets every gametick for the duration of a packet roundtrip
#[derive(Component)]
pub struct SentPlayerInfoRequestPacket(pub u64);