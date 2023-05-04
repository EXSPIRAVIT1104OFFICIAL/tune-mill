use bevy::prelude::*;
use bevy_tweening::{lens::*, *};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BackgoundColorLens {
    pub start: Color,
    pub end: Color,
}

impl Lens<BackgroundColor> for BackgoundColorLens {
    fn lerp(&mut self, target: &mut BackgroundColor, ratio: f32) {
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();
        let value = start.lerp(end, ratio);
        target.0 = value.into();
    }
}
