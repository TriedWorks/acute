use winit::event::VirtualKeyCode as vkc;
use winit_input_helper::WinitInputHelper;

use crate::utils;
use crate::utils::rotor_from_angles;
use rusty_timer::Timer;
use ultraviolet::{projection::rh_ydown::perspective_vk, Isometry3, Mat4, Rotor3, Vec3};

pub struct Camera {
    pub perspective: Mat4,
    pub transformation: Isometry3,
    pub aspect: f32,
    pub fov: f32,
}

impl Camera {
    pub fn new(sc_desc: &wgpu::SwapChainDescriptor) -> Self {
        let vertical_fov: f32 = utils::rad(60.0);
        let aspect = sc_desc.width as f32 / sc_desc.height as f32;
        let z_near: f32 = 0.1;
        let z_far: f32 = 1024.0;

        let perspective: Mat4 = perspective_vk(vertical_fov, aspect, z_near, z_far);
        let mut transformation: Isometry3 = Isometry3::identity();
        transformation.translation.z -= 3.0;
        transformation.translation.y += 2.0;

        Self {
            perspective,
            transformation,
            aspect,
            fov: vertical_fov,
        }
    }

    pub fn update_transformation(&mut self, transformation: Isometry3) {
        self.transformation = transformation;
    }

    pub fn update_translation(&mut self, translation: &Vec3) {
        self.transformation.translation = translation.clone()
    }

    fn transformation_matrix(&self) -> Mat4 {
        self.transformation.into_homogeneous_matrix()
    }

    pub fn to_matrix(&self) -> Mat4 {
        self.perspective * self.transformation_matrix()
    }
}

pub struct CameraController {
    speed: f32,
    is_w: bool,
    is_s: bool,
    is_a: bool,
    is_d: bool,
    is_con: bool,
    is_space: bool,
    is_shift: bool,
    is_q: bool,
    is_e: bool,
    zoom: f32,

    old_mouse_coords: (f32, f32),
    mouse_coords: (f32, f32),
}

impl CameraController {
    pub fn input(&mut self, input: &WinitInputHelper) {
        self.is_w = input.key_held(vkc::W);
        self.is_s = input.key_held(vkc::S);
        self.is_a = input.key_held(vkc::A);
        self.is_d = input.key_held(vkc::D);
        self.is_q = input.key_held(vkc::Q);
        self.is_e = input.key_held(vkc::E);
        self.is_space = input.key_held(vkc::Space);
        self.is_shift = input.key_held(vkc::LShift);
        self.is_con = input.key_held(vkc::LControl);

        self.mouse_coords = match input.mouse() {
            Some(input) => input,
            _ => (0.0, 0.0),
        };

        self.zoom += input.scroll_diff();
        if self.zoom >= 6.0 {
            self.zoom -= 1.0;
        }
        if self.zoom <= -10.0 {
            self.zoom += 1.0
        }
    }

    pub fn update(&mut self, camera: &mut Camera, timer: &Timer) {
        self.update_rotation(camera, timer);

        let forward: Vec3 = Vec3::new(0.0, 0.0, 1.0) * timer.delta_time().as_secs_f32();
        let left: Vec3 = Vec3::new(1.0, 0.0, 0.0) * timer.delta_time().as_secs_f32();
        let up: Vec3 = Vec3::new(0.0, -1.0, 0.0) * timer.delta_time().as_secs_f32();

        match self.is_con {
            true => {}
            false => {}
        }

        match self.is_w {
            true => {
                camera.transformation.translation += forward * self.speed;
            }
            false => {}
        }
        match self.is_s {
            true => {
                camera.transformation.translation -= forward * self.speed;
            }
            false => {}
        }
        match self.is_a {
            true => {
                camera.transformation.translation += left * self.speed;
            }
            false => {}
        }
        match self.is_d {
            true => {
                camera.transformation.translation -= left * self.speed;
            }
            false => {}
        }
        match self.is_space {
            true => {
                camera.transformation.translation += up * self.speed;
            }
            false => {}
        }
        match self.is_shift {
            true => {
                camera.transformation.translation -= up * self.speed;
            }
            false => {}
        }
        match self.is_q {
            true => {
                let mut rotation_rl = rotor_from_angles(0.0, 0.0, -self.speed / 20.0)
                    * timer.delta_time().as_secs_f32();
                rotation_rl.normalize();
                camera.transformation.append_rotation(rotation_rl);
            }
            false => {}
        }
        match self.is_e {
            true => {
                let mut rotation_lr = rotor_from_angles(0.0, 0.0, self.speed / 20.0)
                    * timer.delta_time().as_secs_f32();
                rotation_lr.normalize();
                camera.transformation.append_rotation(rotation_lr);
            }
            false => {}
        }

        self.change_fov(camera);
    }

    /// >:[
    fn update_rotation(&mut self, camera: &mut Camera, timer: &Timer) {
        if self.mouse_coords.0 == self.old_mouse_coords.0
            && self.mouse_coords.1 == self.old_mouse_coords.1
        {
            return;
        }

        let dx: f32 = -self.mouse_coords.0 + self.old_mouse_coords.0;
        let dy: f32 = self.mouse_coords.1 - self.old_mouse_coords.1;

        self.old_mouse_coords = self.mouse_coords;

        let dt = timer.delta_time().as_secs_f32() * self.speed * 400.0;

        let offset_rotor: Rotor3 = utils::rotor_from_angles(dy * dt, dx * dt, 0.0);

        camera.transformation.append_rotation(offset_rotor);
    }

    /// I can feel the suffer on it's way back to us
    fn change_fov(&mut self, camera: &mut Camera) {
        let fov_angle = camera.fov;

        let mut fov: f32 = fov_angle - self.zoom / 5.0;

        // You want some flippy dippy or some zoomers?
        if fov >= 180.0 || fov < 0.0 {
            fov -= fov_angle;
        }

        // Don't even complain about the Variable names, I didn't
        // come up with them >:c
        let t: f32 = (fov / 2.0).tan();
        let sy: f32 = 1.0 / t;
        let sx: f32 = sy / camera.aspect;

        camera.perspective.cols[0].x = sx;
        camera.perspective.cols[1].y = sy;
    }
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 10.0,
            is_w: false,
            is_s: false,
            is_a: false,
            is_d: false,
            is_con: false,
            is_space: false,
            is_shift: false,
            is_q: false,
            is_e: false,
            zoom: 0.0,
            old_mouse_coords: (0.0, 0.0),
            mouse_coords: (0.0, 0.0),
        }
    }
}
