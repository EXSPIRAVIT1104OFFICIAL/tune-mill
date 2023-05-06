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
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use std::{convert::TryFrom, time::Duration};
use styles::*;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash, Default, States, TryFromPrimitive)]
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
        .add_system(check_state_on_tween_complete_event)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(ImageBundle {
            image: UiImage {
                texture: asset_server.load("background.png"),
                ..default()
            },
            z_index: ZIndex::Global(0),
            style: CENTER,
            ..default()
        })
        .insert(Background);

    commands
        .spawn(ImageBundle {
            image: UiImage {
                texture: asset_server.load("grid.png"),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            z_index: ZIndex::Global(1),
            style: CENTER,
            ..default()
        })
        .insert(Grid);
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

    let start: Tween<BackgroundColor> = Tween::new(
        EaseFunction::CubicIn,
        Duration::from_secs_f32(0.25),
        BackgoundColorLens {
            start: color(PURE_WHITE),
            end: color(NULL_WHITE),
        },
    )
    .with_repeat_count(RepeatCount::Finite(1));
    let end = Tween::new(
        EaseFunction::CubicOut,
        Duration::from_secs_f32(0.25),
        BackgoundColorLens {
            start: color(NULL_WHITE),
            end: color(PURE_WHITE),
        },
    )
    .with_repeat_count(RepeatCount::Finite(1))
    .with_completed_event(AppState::Home as u64);
    let seq = Delay::new(Duration::from_secs_f32(0.5))
        .then(start)
        .then(Delay::new(Duration::from_secs(2)))
        .then(end);

    commands
        .spawn((
            ImageBundle {
                image: UiImage {
                    texture: asset_server.load("background.png"),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                z_index: ZIndex::Global(3),
                style: CENTER,
                ..default()
            },
            Animator::<BackgroundColor>::new(seq),
        ))
        .insert(TextLayerHider);
}

fn setup_home() {}

fn check_state_on_tween_complete_event(
    mut reader: EventReader<TweenCompleted>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in reader.iter() {
        change_state_to(
            AppState::try_from(event.user_data as usize),
            app_state.0,
            &mut next_state,
        );
    }
}

fn change_state_to(
    state: Result<AppState, TryFromPrimitiveError<AppState>>,
    app_state: AppState,
    next_state: &mut ResMut<NextState<AppState>>,
) {
    match state {
        Ok(state) if app_state != state => {
            next_state.set(state);
            println!("switching from {:?} to {:?}", app_state, state);
        }
        Ok(_) => println!("already in state: {:?}", app_state),
        Err(_) => println!("no such state: {:?}", app_state),
    }
}
