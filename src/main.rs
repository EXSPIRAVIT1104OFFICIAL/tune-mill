mod components;
mod resources;
mod styles;
mod systems;
mod tween;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowPlugin},
};
use bevy_tweening::*;
use num_enum::TryFromPrimitive;
use pecs::prelude::*;
use resources::*;
use styles::*;
use systems::*;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash, Default, States, TryFromPrimitive)]
#[repr(usize)]
pub enum AppState {
    #[default]
    Splash,
    Home,
    StepSequencer,
    EditTrack,
    EditPitch,
    ModeExecution,
    Generator,
}

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
        .add_plugin(PecsPlugin)
        .add_state::<AppState>()
        .init_resource::<DeviceInfo>()
        .add_startup_system(setup)
        .add_system(get_handle_rotation)
        .add_system(rotate_cursor.after(get_handle_rotation))
        .add_system(component_animator_system::<Text>)
        .add_system(component_animator_system::<BackgroundColor>)
        .add_system(show_splash.in_schedule(OnEnter(AppState::Splash)))
        .add_system(transition_to_home.in_schedule(OnEnter(AppState::Home)))
        .add_system(transition_to_step_sequencer.in_schedule(OnEnter(AppState::StepSequencer)))
        .add_system(transition_to_generator.in_schedule(OnEnter(AppState::Generator)))
        .add_system(transition_to_edit_track.in_schedule(OnEnter(AppState::EditTrack)))
        .add_system(transition_to_edit_pitch.in_schedule(OnEnter(AppState::EditPitch)))
        .run();
}
