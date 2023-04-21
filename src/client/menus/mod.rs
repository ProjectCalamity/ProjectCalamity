use bevy::prelude::*;
use kayak_ui::prelude::{*, widgets::*};

use super::{ClientState};

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_system(skip_menu.in_schedule(OnEnter(ClientState::MainMenu)))
            .add_system(setup_scaling.in_set(OnUpdate(ClientState::MainMenu)))
            .add_system(update_scaling.in_set(OnUpdate(ClientState::MainMenu)));
    }
}

fn skip_menu(mut state: ResMut<NextState<ClientState>>) {
    info!("Skipping menu, directly entering game");
    state.0 = Some(ClientState::Game);
}

#[derive(Component, Reflect)]
pub struct UICamera;

#[derive(Component, Reflect)]
pub struct UICameraScalingInfo {
    width_perc: f32, // From 0-1
    height_perc: f32, // From 0-1
}

pub struct UIScalable;

fn setup_scaling(mut commands: Commands, orth_q: Query<(Entity, &OrthographicProjection, With<UICamera>, Without<UICameraScalingInfo>)>) {
    if orth_q.iter().len() <= 0 {
        return;
    }

    let orth = orth_q.single().1;
    let mut width_perc = 1f32;
    let mut height_perc = 1f32;
    
    if orth.area.width() > orth.area.height() {
        height_perc *= orth.area.width() / orth.area.height();
    } else if orth.area.width() < orth.area.height() {
        width_perc *= orth.area.height() / orth.area.width();
    }
    
    commands
        .entity(orth_q.single().0)
        .insert(UICameraScalingInfo { width_perc, height_perc });
}

fn update_scaling(mut orth_q: Query<(Entity, &OrthographicProjection, &mut UICameraScalingInfo, With<UICamera>)>) {
    if orth_q.iter().len() <= 0 {
        return;
    }
    let orth = orth_q.single().1;
    let mut width_perc = 1f32;
    let mut height_perc = 1f32;
    
    if orth.area.width() > orth.area.height() {
        height_perc *= orth.area.width() / orth.area.height();
    } else if orth.area.width() < orth.area.height() {
        width_perc *= orth.area.height() / orth.area.width();
    }

    let mut csi = orth_q.single_mut().2;
    csi.width_perc = width_perc;
    csi.height_perc = height_perc;
}