use bevy::prelude::*;

pub mod graphical;
pub mod menus;


#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientState {
    #[default]
    MainMenu,
    Game
}