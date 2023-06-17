use bevy::prelude::*;
use bevy_tweening::*;
use std::time::Duration;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BackgoundColorLens {
    pub color: Color,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextColorLens {
    pub color: Color,
    pub section: usize,
}

impl Lens<BackgroundColor> for BackgoundColorLens {
    fn lerp(&mut self, target: &mut BackgroundColor, ratio: f32) {
        let start: Vec4 = target.0.into();
        let end: Vec4 = self.color.into();
        let value = start.lerp(end, ratio);
        target.0 = value.into();
    }
}

impl Lens<Text> for TextColorLens {
    fn lerp(&mut self, target: &mut Text, ratio: f32) {
        // Note: Add<f32> for Color affects alpha, but not Mul<f32>. So use Vec4 for
        // consistency.
        let start: Vec4 = target.sections[self.section].style.color.into();
        let end: Vec4 = self.color.into();
        let value = start.lerp(end, ratio);
        target.sections[self.section].style.color = value.into();
    }
}

pub fn create_text_color_tween(color: Color, section: usize) -> Tween<Text> {
    Tween::new(
        EaseFunction::CubicInOut,
        Duration::from_secs(2),
        TextColorLens { color, section },
    )
    .with_repeat_count(RepeatCount::Finite(1))
    .with_completed(|_entity, _tween| {
        info!("Text color tween completed");
    })
}

pub fn create_background_color_tween(color: Color) -> Tween<BackgroundColor> {
    Tween::new(
        EaseFunction::CubicInOut,
        Duration::from_secs(2),
        BackgoundColorLens { color },
    )
    .with_repeat_count(RepeatCount::Finite(1))
    .with_completed(|_entity, _tween| {
        info!("Background color tween completed");
    })
}
