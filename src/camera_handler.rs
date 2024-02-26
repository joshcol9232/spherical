use macroquad::prelude::*;
use macroquad::math::{Quat, Mat3, Vec3, Vec2};

use crate::tools::vec_from_spherical;
use crate::consts;


pub struct CameraHandler {
    // Target is always (0, 0, 0)
    rot: Quat,
    scale: f32,
    cam: Camera3D,
}

impl CameraHandler {
    pub fn new(x_rotation: f32, y_rotation: f32, scale: f32) -> Self {
        let rot = Quat::from_rotation_x(x_rotation) + Quat::from_rotation_y(y_rotation);

        let cam = Camera3D {
            position: rot * vec3(1., 0., 0.) * scale,
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        };
        Self { rot, scale, cam }
    }

    pub fn get_camera(&self) -> &Camera3D { &self.cam }

    fn update_rotation(&mut self) {
        self.rot = self.rot.normalize();
        self.cam.position = self.rot * self.cam.position;
    }

    fn zoom(&mut self, factor: f32) {
        self.cam.position *= factor;
    }

    fn get_mouse_delta() -> Option<Vec2> {
        let delta: Vec2 = mouse_delta_position();
        // Hack to stop large change when releasing the mouse button and
        // clicking again.
        if delta.x.abs() > 0.2 || delta.y.abs() > 0.2 {
            None
        } else {
            Some(delta)
        }
    }

    // Convert a mouse delta position, to a rotation of the camera.
    fn mouse_delta_to_rotation(&self, delta: &Vec2) -> Quat {
        let current_pos_norm = self.cam.position.normalize();

        // Get the 3D vector in the plane of the camera.
        // y = camera.up, x = norm(camera.loc) cross camera.up
        let y_unit_vec: Vec3 = self.cam.up.normalize();
        let x_unit_vec: Vec3 = current_pos_norm.cross(y_unit_vec);
        let delta_3d = x_unit_vec * delta.x + self.cam.up * delta.y;
        let new_desired_pos = (current_pos_norm + delta_3d).normalize();
        Quat::from_rotation_arc(current_pos_norm, new_desired_pos)
    }

    pub fn process_mouse(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let delta_opt = Self::get_mouse_delta();
            if let Some(mut delta) = delta_opt {
                delta *= -1.0 * std::f32::consts::PI;
                self.rot = self.rot + self.mouse_delta_to_rotation(&delta);
                self.update_rotation();
            }
        }
        if is_mouse_button_down(MouseButton::Right) {
            let delta_opt = Self::get_mouse_delta();
            if let Some(delta) = delta_opt {
                // Zoom by `y` amount
                self.zoom(-delta.y + 1.0);
            }

        }
    }
}

impl Default for CameraHandler {
    fn default() -> Self {
        Self::new(0.0, 0.0, consts::START_SCALE)
    }
}

