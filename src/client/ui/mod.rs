pub mod components;

use bevy::prelude::*;
use kayak_ui::prelude::{*, widgets::*};

use crate::client::{graphical::GameCamera, ui::components::turn_timer::{TurnTimerWidget, TurnTimerWidgetState, turn_timer_widget_render}};

use super::ClientState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_startup_system(startup);
    }
}

pub struct ProjectCalamityConsts;

impl ProjectCalamityConsts {
    pub const BUTTON_BACKGROUND: Color = Color::Rgba { red: 0f32, green: 0f32, blue: 0f32, alpha: 0.4f32 };
}

#[derive(Component, Default, Reflect)]
pub struct ScalableComponent {
    base_pos: Vec2,
    base_scale: Vec2,
    actual_pos: Vec2,
    actual_scale: Vec2
}

#[derive(Component, Reflect)]
struct UIScalingInfo {
    initial_scale: Vec2,
}

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<ClientState>>,
) {
    font_mapping.set_default(asset_server.load("fonts/atkinson_hyperlegible_regular.kayak_font"));
    font_mapping.add("regular", asset_server.load("fonts/atkinson_hyperlegible_regular.kayak_font"));
    font_mapping.add("bold", asset_server.load("fonts/atkinson_hyperlegible_bold.kayak_font"));

    let camera_entity = commands
        .spawn(Camera2dBundle::default())
        .insert(CameraUIKayak)
        .insert(GameCamera)
        .id();

    let mut widget_context = KayakRootContext::new(camera_entity);
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_data::<TurnTimerWidget, TurnTimerWidgetState>();
    widget_context.add_widget_system(
        TurnTimerWidget::default().get_name(),
        widget_update::<TurnTimerWidget, TurnTimerWidgetState>,
        turn_timer_widget_render,
    );

    
    
    let parent_id = None;

    rsx! {
        <KayakAppBundle>
            // Note: This is for in-game UI
            // <ElementBundle
            //     styles = {
            //         KStyle {
            //             padding: StyleProp::Value(Edge::all(Units::Pixels(10f32))),
            //             ..default()
            //         }
            //     }
            // >
            //     <TurnTimerWidgetBundle/>
            // </ElementBundle>
        </KayakAppBundle>
    };

    commands.spawn((widget_context, EventDispatcher::default()));

    // SKIP UI
    info!("Skipping UI");
    state.0 = ClientState::Game;
}