use bevy::prelude::*;

pub const WHITE : &str = "#F2F2F7";
pub const LIGHT_GREY : &str = "#8E8E93";

pub fn color(hex_code : &str) -> Color {
    Color::hex(hex_code).unwrap()
}