use std::time::Duration;

use bevy::prelude::*;
use kayak_ui::prelude::{*, widgets::*};

use crate::client::ui::ProjectCalamityConsts;

#[derive(Component, Clone, PartialEq, Default)]
pub struct TurnTimerWidget;

impl Widget for TurnTimerWidget { }

#[derive(Component, Default, PartialEq, Clone)]

pub struct TurnTimerWidgetState {
    pub duration: Duration
}

#[derive(Bundle)]
pub struct TurnTimerWidgetBundle {
    pub props: TurnTimerWidget,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for TurnTimerWidgetBundle {
    fn default() -> Self {
        Self {
            props: TurnTimerWidget::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: TurnTimerWidget::default().get_name(),
        }
    }
}

pub fn turn_timer_widget_render(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    children_q: Query<&KChildren>,
    duration_q: Query<&TurnTimerWidgetState>
) -> bool {
    if let Ok(children) = children_q.get(entity) {
        let state_entity = widget_context.use_state(&mut commands, entity, TurnTimerWidgetState::default());
        if let Ok(duration) = duration_q.get(state_entity) {
            let background_styles = KStyle {
                background_color: StyleProp::Value(ProjectCalamityConsts::BUTTON_BACKGROUND),
                width: StyleProp::Value(Units::Pixels(75f32)),
                height: StyleProp::Value(Units::Pixels(75f32)),
                border_radius: Corner::all(20f32).into(),
                padding_top: StyleProp::Value(Units::Pixels(2f32)),
                ..Default::default()
            };

            let parent_id = Some(entity);

            rsx! {
                <BackgroundBundle
                    styles={background_styles}
                    children={children.clone()}
                >
                    <ElementBundle
                        styles={KStyle {
                            padding_top: StyleProp::Value(Units::Pixels(10f32)),
                            ..Default::default()
                        }}
                    >
                        <TextWidgetBundle
                            text={TextProps {
                                content: "TURN TIME REMAINING".into(),
                                size: 10f32,
                                alignment: Alignment::Middle,
                                ..Default::default()
                            }}
                        />
                        <TextWidgetBundle
                            text={TextProps {
                                content: format!("{:?}", duration.duration).into(),
                                size: 24f32,
                                alignment: Alignment::Middle,
                                ..Default::default()
                            }}
                        />
                    </ElementBundle>
                </BackgroundBundle>
            };
        }
    }
    return true;
}
