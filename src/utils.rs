#![allow(dead_code, unused_imports)]

use ultraviolet::Rotor3;

use std::f32::consts::PI;

/// Radians?!?! More like Degrees.
pub fn rad(angle: f32) -> f32 {
    angle / 180.0 * PI
}

/// zxy?!? where is the hidden camera
pub fn rotor_from_angles(x: f32, y: f32, z: f32) -> Rotor3 {
    Rotor3::from_euler_angles(rad(z), rad(x), rad(y))
}