use bevy::prelude::*;

pub const CENTER: Style = Style {
    position_type: PositionType::Absolute,
    margin: UiRect {
        left: Val::Auto,
        right: Val::Auto,
        top: Val::Auto,
        bottom: Val::Auto,
    },
    position: UiRect {
        left: Val::Px(0.0),
        top: Val::Px(0.0),
        ..UiRect::DEFAULT
    },
    size: Size::new(Val::Px(480.), Val::Px(480.)),
    ..Style::DEFAULT
};
pub const CENTER_FLEX: Style = Style {
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
pub const MAIN_TEXT_OFFSET: Style = Style {
    position: UiRect {
        bottom: Val::Px(2.5),
        ..UiRect::DEFAULT
    },
    ..Style::DEFAULT
};
pub const WHITE: &str = "#F2F2F7";
pub const LIGHT_GREY: &str = "#8E8E93";
pub const GREY: &str = "#3A3A3C";
pub const BLACK: &str = "#1C1C1E";

pub fn color(hex_code: &str) -> Color {
    Color::hex(hex_code).unwrap()
}
