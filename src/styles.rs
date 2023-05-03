use bevy::prelude::*;

pub const MAIN_TEXT_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::FlexStart,
    position_type: PositionType::Absolute,
    margin: UiRect { 
        left: Val::Auto,
        right: Val::Auto, 
        top: Val::Auto, 
        bottom: Val::Auto,
    },
    size: Size::new(Val::Percent(100.), Val::Px(30.)),
    ..Style::DEFAULT
};

pub const WHITE : &str = "#F2F2F7";
pub const LIGHT_GREY : &str = "#8E8E93";

pub fn color(hex_code : &str) -> Color {
    Color::hex(hex_code).unwrap()
}