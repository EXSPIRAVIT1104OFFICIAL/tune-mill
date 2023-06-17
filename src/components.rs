use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Grid;

#[derive(Component)]
pub struct MainText;

#[derive(Component)]
pub struct SubText;

#[derive(Component)]
pub struct ActionText;

#[derive(Component)]
pub struct ActionIcon;

#[derive(Component)]
pub struct InfoText;

#[derive(Component)]
pub struct ModeIcon;

#[derive(Component)]
pub struct RandomRect;

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct Wave;

#[derive(Component)]
pub struct Note {
    pub control_voltage: f32,
    pub scientific_pitch_notation: String,
    pub hertz: i16,
}
