mod lens;
mod styles;

use bevy::{
    prelude::*,
    time::Stopwatch,
    window::{PresentMode, WindowPlugin},
    winit::WinitSettings,
};
use bevy_tweening::{lens::*, *};
use lens::*;
use num_enum::TryFromPrimitive;
use std::{convert::TryFrom, time::Duration};
use styles::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States, TryFromPrimitive)]
#[repr(usize)]
pub enum AppState {
    #[default]
    Splash,
    Home,
    StepSequencer,
    RandomSequencer,
    Edit,
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Grid;

#[derive(Component)]
struct MainText;

#[derive(Component)]
struct SubText;
#[derive(Component)]
struct TextLayerHider;

#[derive(Component, Deref, DerefMut)]
pub struct CustomTimer(Timer);

// 0: background
// 1: grid
// 2: text layer
// 3: text layer hider

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(color(GREY)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "TUNE MILL™".into(),
                        resolution: (480., 480.).into(),
                        fit_canvas_to_parent: true,
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(TweeningPlugin)
        .add_state::<AppState>()
        .add_startup_system(setup)
        .add_system(component_animator_system::<BackgroundColor>)
        .add_system(setup_splash.in_schedule(OnEnter(AppState::Splash)))
        .add_system(setup_home.in_schedule(OnEnter(AppState::Home)))
        .add_system(check_state_tween_complete)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: CENTER,
            z_index: ZIndex::Global(0),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: UiImage {
                        texture: asset_server.load("background.png"),
                        ..default()
                    },
                    ..default()
                })
                .insert(Background);
        });

    commands
        .spawn(NodeBundle {
            style: CENTER,
            z_index: ZIndex::Global(1),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: UiImage {
                        texture: asset_server.load("grid.png"),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..default()
                })
                .insert(Grid);
        });
}

fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: CENTER_FLEX,
            z_index: ZIndex::Global(2),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "TUNE MILL",
                        TextStyle {
                            font: asset_server.load("fonts/PPMori-Regular.otf"),
                            font_size: 40.,
                            color: color(WHITE),
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(MAIN_TEXT_OFFSET),
                )
                .insert(MainText);

            parent
                .spawn(
                    TextBundle::from_section(
                        "TM",
                        TextStyle {
                            font: asset_server.load("fonts/PPMori-Extralight.otf"),
                            font_size: 20.,
                            color: color(LIGHT_GREY),
                        },
                    )
                    .with_text_alignment(TextAlignment::Center),
                )
                .insert(SubText);
        });

    let start = Tween::new(
        EaseFunction::QuinticInOut,
        std::time::Duration::from_secs(2),
        BackgoundColorLens {
            start: Color::WHITE,
            end: Color::NONE,
        },
    )
    .with_repeat_count(RepeatCount::Finite(1));
    let end = Tween::new(
        EaseFunction::QuinticInOut,
        std::time::Duration::from_secs(2),
        BackgoundColorLens {
            start: Color::NONE,
            end: Color::WHITE,
        },
    )
    .with_repeat_count(RepeatCount::Finite(1))
    .with_completed_event(AppState::Home as u64);
    let seq = start.then(end);

    commands
        .spawn(NodeBundle {
            style: CENTER,
            z_index: ZIndex::Global(3),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ImageBundle {
                        image: UiImage {
                            texture: asset_server.load("background.png"),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::NONE),
                        ..default()
                    },
                    Animator::<BackgroundColor>::new(seq),
                ))
                .insert(TextLayerHider);
        });
    println!("start");
}

fn setup_home() {
    println!("home");
}

fn check_state_tween_complete(
    mut reader: EventReader<TweenCompleted>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in reader.iter() {
        match AppState::try_from(event.user_data as usize) {
            Ok(state) if app_state.0 != state => {
                next_state.set(state.clone());
                println!("switching from {:?} to {:?}", app_state.0, state.clone());
            }
            Ok(_) => println!("already in state: {:?}", app_state.0),
            Err(_) => println!("no such state: {:?}", app_state.0),
        }
    }
}
