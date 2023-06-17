use bevy::prelude::*;

#[derive(Resource)]
pub struct DeviceInfo {
    pub rotation_degrees: i16,
    pub input_voltage: f32,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            rotation_degrees: 0,
            input_voltage: 0.,
        }
    }
}

impl DeviceInfo {
    pub fn set_rotation_degrees(&mut self, degrees: i16) {
        self.rotation_degrees = if degrees > 360 {
            degrees % 360
        } else if degrees < 0 {
            360 + degrees
        } else {
            degrees
        };
        info!(self.rotation_degrees)
    }
}
