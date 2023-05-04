use bevy::prelude::*;
use kayak_ui::prelude::{*, widgets::*};

pub struct MainMenu;

#[derive(Component, Clone, PartialEq, Default)]
pub struct PCButtonProps {

}

impl Widget for PCButtonProps { }

#[derive(Bundle)]
pub struct PCButtonBundle {
    pub props: PCButtonProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName
}

impl Default for PCButtonBundle {
    fn default() -> Self {
        return Self { 
            props: Default::default(), 
            styles: Default::default(), 
            computed_styles: Default::default(), 
            children: Default::default(), 
            on_event: Default::default(), 
            widget_name: PCButtonProps::default().get_name() 
        }
    }
}

pub fn render_button(
    In(entity): In<Entity>,
    widget_context: ResMut<KayakWidgetContext>,
    mut commands: Commands,
    mut query: Query<&KChildren>
) -> bool {

    info!("RUNNING");
    if let Ok(children) = query.get(entity) {
        let background_styles = KStyle {
            background_color: StyleProp::Value(Color::BLACK),
            border_radius: Corner::all(50.0).into(),
            ..default()
        };

        let parent_id = Some(entity);

        rsx! {
            <BackgroundBundle
                styles = {background_styles}
                children = {children.clone()}
            />
        };
    }
    return true;
}