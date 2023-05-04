pub mod components;

use bevy::prelude::*;
use kayak_ui::prelude::{*, widgets::*};

use crate::client::ui::components::{button::QuadBundle, main_menu::{PCButtonProps, render_button, PCButtonBundle}};

use self::components::button::{Quad, update_quad};

use super::ClientState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system(skip_ui.in_schedule(OnEnter(ClientState::Game)));
            // .register_type::<Quad>()
            // .register_type::<ScalableComponent>()
            // .register_type::<UIScalingInfo>()
            // .add_plugin(KayakContextPlugin)
            // .add_plugin(KayakWidgets)
            // .add_startup_system(setup)
            // .add_system(update_ui_scaling);
    }
}

fn skip_ui(mut state: ResMut<State<ClientState>>) {
    info!("Skipping UI Stage");

    state.0 = ClientState::Game;
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

fn update_ui_scaling(
    camera: Query<(&UIScalingInfo, &OrthographicProjection, With<CameraUIKayak>)>,
    mut components: Query<&mut ScalableComponent>

) {
    let (scale, projection, ()) = camera.single();

    let (m_x, m_y) = (projection.area.width() / scale.initial_scale.x, projection.area.height() / scale.initial_scale.y);
    // info!("Scaling by {:?} {:?} | {:?}", m_x, m_y, scale.initial_scale);

    // for mut component in components.iter_mut() {
    //     component.actual_pos.x = component.base_pos.x * m_x;
    //     component.actual_pos.y = component.base_pos.y * m_x;

    //     component.actual_scale.x = component.base_scale.x * m_x;
    //     component.actual_scale.y = component.base_scale.x * m_y;
    // }

}

fn setup(
    mut commands: Commands
) {

    let camera_bundle = Camera2dBundle::default();

    let area = camera_bundle.projection.area;

    let camera_entity = commands
        .spawn(camera_bundle)
        .insert(CameraUIKayak)
        .insert(UIScalingInfo { 
            initial_scale: Vec2 { 
                x: area.width(),
                y: area.height(),  
            }
        })
        .id();

    let mut widget_context = KayakRootContext::new(camera_entity);
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_system(
        PCButtonProps::default().get_name(),
        widget_update::<PCButtonProps, EmptyState>,
        render_button
    );
    let parent_id = None;

    rsx! {
        <KayakAppBundle>
            <PCButtonBundle>
                <TextWidgetBundle
                    text={TextProps {
                        content: "Click me!".into(),
                        ..Default::default()
                    }}
                />
            </PCButtonBundle>
        </KayakAppBundle>
    };
}