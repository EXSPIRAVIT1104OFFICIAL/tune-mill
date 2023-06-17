use crate::components::*;
use crate::main;
use crate::resources::*;
use crate::styles::*;
use crate::tween::*;
use crate::AppState;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_tweening::*;
use num_enum::TryFromPrimitiveError;
use pecs::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;
// use rppal::{gpio::Gpio, system::DeviceInfo};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    None,
    Play,
    Lock,
    Add,
    Remove,
}

impl Default for Mode {
    fn default() -> Self {
        Self::None
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(ImageBundle {
            image: UiImage {
                texture: asset_server.load("sprites/circle.png"),
                ..default()
            },
            background_color: BackgroundColor(color(BLACK)),
            z_index: ZIndex::Global(0),
            style: CENTER,
            ..default()
        })
        .insert(Background);

    commands
        .spawn(ImageBundle {
            image: UiImage {
                texture: asset_server.load("sprites/grid.png"),
                ..default()
            },
            z_index: ZIndex::Global(1),
            style: CENTER,
            ..default()
        })
        .insert(Grid);

    commands
        .spawn(NodeBundle {
            style: MAIN_TEXT_FLEX,
            z_index: ZIndex::Global(2),
            background_color: BackgroundColor(color(BLACK)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section("TUNE MILL", get_main_text_style(&asset_server))
                        .with_text_alignment(TextAlignment::Center)
                        .with_style(MAIN_TEXT_OFFSET),
                )
                .insert(MainText);

            parent
                .spawn(
                    TextBundle::from_section("TM", get_sub_text_style(&asset_server))
                        .with_text_alignment(TextAlignment::Center),
                )
                .insert(SubText);
        });

    commands
        .spawn(NodeBundle {
            style: CENTER,
            z_index: ZIndex::Global(3),
            ..default()
        })
        .insert(Cursor)
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load("sprites/rectangle.png"),
                    ..default()
                },
                background_color: BackgroundColor(color(BLUE)),
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(15.), Val::Px(120.)),
                    position: UiRect {
                        left: Val::Px(232.5),
                        top: Val::Px(-60.0),
                        ..UiRect::DEFAULT
                    },
                    ..default()
                },
                ..default()
            });
        });
}

pub fn get_handle_rotation(
    mut mouse_wheel_event_reader: EventReader<MouseWheel>,
    mut device_info: ResMut<DeviceInfo>,
) {
    for mouse_wheel in mouse_wheel_event_reader.iter() {
        let degrees = device_info.rotation_degrees + mouse_wheel.y as i16 * 15;
        device_info.set_rotation_degrees(degrees);
    }
}

pub fn rotate_cursor(mut query: Query<(&Cursor, &mut Transform)>, device_info: Res<DeviceInfo>) {
    let (_cursor, mut transform) = query.single_mut();
    transform.rotation = transform.rotation.lerp(
        Quat::from_rotation_z(device_info.rotation_degrees as f32 / 180. * PI),
        0.1,
    );
}

pub fn show_splash(
    mut commands: Commands,
    mut main_text_query: Query<Entity, With<MainText>>,
    mut sub_text_query: Query<Entity, With<SubText>>,
) {
    let main_text = main_text_query.single_mut();
    commands.entity(main_text).insert(Animator::new(
        Delay::new(Duration::from_secs(2))
            .then(create_text_color_tween(color(WHITE), 0))
            .then(Delay::new(Duration::from_secs(2)))
            .then(create_text_color_tween(color(BLACK), 0)),
    ));
    let sub_text = sub_text_query.single_mut();
    commands.entity(sub_text).insert(Animator::new(
        Delay::new(Duration::from_secs(2))
            .then(create_text_color_tween(color(LIGHT_GREY), 0))
            .then(Delay::new(Duration::from_secs(2)))
            .then(create_text_color_tween(color(BLACK), 0)),
    ));
    commands
        .promise(|| ())
        .then(asyn!(state => {
            state.asyn().timeout(4.0)
        }))
        .then(asyn!(
            app_state: Res<State<AppState>>,
            next_state: ResMut<NextState<AppState>>,
        => {
            change_state_to(Ok(AppState::Home), app_state, next_state);}
        ));
}

pub fn transition_to_home(mut commands: Commands, asset_server: Res<AssetServer>) {}

pub fn transition_to_step_sequencer(mut commands: Commands, asset_server: Res<AssetServer>) {}

pub fn transition_to_generator(mut commands: Commands, asset_server: Res<AssetServer>) {}

pub fn transition_to_edit_track(mut commands: Commands, asset_server: Res<AssetServer>) {}

pub fn transition_to_edit_pitch(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn change_state_to(
    state: Result<AppState, TryFromPrimitiveError<AppState>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match state {
        Ok(state) if app_state.0 != state => {
            next_state.set(state);
            info!("Switching from {:?} to {:?}", app_state.0, state);
        }
        Ok(_) => info!("Already in state: {:?}", app_state),
        Err(_) => info!("No such state: {:?}", app_state),
    }
}
