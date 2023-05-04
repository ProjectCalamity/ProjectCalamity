use bevy::prelude::*;
use kayak_ui::prelude::*;

use crate::client::ui::ScalableComponent;

#[derive(Clone, Component, Default, PartialEq, Reflect)]
pub struct Quad {
    pub pos: Vec2,
    pub size: Vec2,
    pub colour: Color
}

impl Widget for Quad {}

#[derive(Bundle)]
pub struct QuadBundle {
    pub quad: Quad,
    pub style: KStyle,
    pub computed_style: ComputedStyles,
    pub widget_name: WidgetName,
    pub scale: ScalableComponent
}

impl Default for QuadBundle {
    fn default() -> Self {
        return Self { 
            quad: Default::default(), 
            style: KStyle { 
                render_command: StyleProp::Value(RenderCommand::Quad),
                min_height: StyleProp::Value(Units::Pixels(200f32)),
                min_width: StyleProp::Value(Units::Pixels(200f32)),
                max_height: StyleProp::Value(Units::Pixels(200f32)),
                max_width: StyleProp::Value(Units::Pixels(200f32)),
                z_index: StyleProp::Value(100),
                ..default() }, 
            computed_style: ComputedStyles(KStyle { 
                render_command: StyleProp::Value(RenderCommand::Quad),
                min_height: StyleProp::Value(Units::Pixels(200f32)),
                min_width: StyleProp::Value(Units::Pixels(200f32)),
                max_height: StyleProp::Value(Units::Pixels(200f32)),
                max_width: StyleProp::Value(Units::Pixels(200f32)),
                z_index: StyleProp::Value(100),
                ..default() }), 
            widget_name: Quad::default().get_name(),
            scale: Default::default(),
        }
    }
}

pub fn update_quad(
    In((_widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut query: Query<(&Quad, &KStyle, &ScalableComponent, &mut ComputedStyles)>
) -> bool {

    if let Ok((quad, _style, scale, mut computed_style)) = query.get_mut(entity) {
        println!("Running. {:?}", computed_style.0.render_command);

        *computed_style = KStyle::default()
            .with_style(
                KStyle {
                    render_command: StyleProp::Value(RenderCommand::Quad),
                    left: StyleProp::Value(Units::Pixels(scale.actual_pos.x)),
                    top: StyleProp::Value(Units::Pixels(scale.actual_pos.y)),
                    width: StyleProp::Value(Units::Pixels(scale.actual_scale.x)),
                    height: StyleProp::Value(Units::Pixels(scale.actual_scale.y)),
                    background_color: StyleProp::Value(quad.colour),
                    ..Default::default()
                }
            )
            .into();
    
        
    }

    return true;
}