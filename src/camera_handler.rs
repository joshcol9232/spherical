use macroquad::prelude::*;
use macroquad::math::{Quat, Mat3, Vec3, Vec2};

use crate::tools::vec_from_spherical;
use crate::consts;


pub struct CameraHandler {
    // Target is always (0, 0, 0)
    rot: Quat,
    r: f32,
    cam: Camera3D,
}

impl CameraHandler {
    pub fn new(x_rotation: f32, y_rotation: f32, r: f32) -> Self {
        let rot = Quat::from_rotation_x(x_rotation) + Quat::from_rotation_y(y_rotation);

        let cam = Camera3D {
            position: rot * vec3(1., 0., 0.) * r,
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        };
        Self { rot, r, cam }
    }

    pub fn get_camera(&self) -> &Camera3D { &self.cam }

    fn update_camera(&mut self) {
        self.rot = self.rot.normalize();
        self.cam.position = self.rot * self.cam.position;
    }

    pub fn process_mouse(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let delta: Vec2 = mouse_delta_position() * -1.0 * std::f32::consts::PI;
            println!("Mouse vec: {:?}", delta);

            let current_pos_norm = self.cam.position.normalize();

            // Get the 3D vector in the plane of the camera.
            // y = camera.up, x = norm(camera.loc) cross camera.up
            let y_unit_vec: Vec3 = self.cam.up.normalize();
            let x_unit_vec: Vec3 = current_pos_norm.cross(y_unit_vec);
            let delta_3d = x_unit_vec * delta.x + self.cam.up * delta.y;

            let new_desired_pos = (current_pos_norm + delta_3d).normalize();
            println!("New desired: {:?}", new_desired_pos);

            self.rot = self.rot + Quat::from_rotation_arc(current_pos_norm, new_desired_pos);

            self.update_camera();
        }
    }
}

impl Default for CameraHandler {
    fn default() -> Self {
        Self::new(0.0, 0.0, consts::START_CAMERA_DISTANCE)
    }
}

