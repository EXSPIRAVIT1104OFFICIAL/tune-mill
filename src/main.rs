mod styles;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowPlugin},
    winit::WinitSettings,
};
use styles::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
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

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::hex("#3A3A3C").unwrap()))
        .insert_resource(WinitSettings::desktop_app())
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
        .add_state::<AppState>()
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: MAIN_TEXT_STYLE,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "TUNE MILL",
                    TextStyle {
                        font: asset_server.load("fonts/PPMori-Regular.otf"),
                        font_size: 40.,
                        color: color(WHITE),
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position: UiRect {
                        bottom: Val::Px(2.5),
                        ..default()
                    },
                    ..default()
                }),
            );

            parent.spawn(
                TextBundle::from_section(
                    "TM",
                    TextStyle {
                        font: asset_server.load("fonts/PPMori-Extralight.otf"),
                        font_size: 20.,
                        color: color(LIGHT_GREY),
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
        })
        .insert(MainText);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                // Alpha channel of the color controls transparency.
                color: Color::rgba(1., 1., 1., 1.),
                ..default()
            },
            texture: asset_server.load("grid.png"),
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        })
        .insert(Grid);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                // Alpha channel of the color controls transparency.
                color: Color::rgba(1., 1., 1., 1.),
                ..default()
            },
            texture: asset_server.load("background.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(Background);
}
