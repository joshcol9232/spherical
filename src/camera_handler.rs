use macroquad::prelude::*;

use crate::tools::vec_from_spherical;
use crate::consts;

pub struct CameraHandler {
    // Current location in spherical coordinates.
    // Target is always (0, 0, 0)
    phi: f32,
    theta: f32,
    r: f32,

    cam: Camera3D,
}

impl CameraHandler {
    pub fn new(phi: f32, theta: f32, r: f32) -> Self {
        let cam = Camera3D {
            position: vec_from_spherical(phi, theta, r),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        };
        Self { phi, theta, r, cam }
    }

    pub fn get_camera(&self) -> &Camera3D { &self.cam }

    pub fn incremental_update(&mut self, dphi: f32, dtheta: f32, dr: f32) {
        self.phi += dphi;
        self.theta += dtheta;
        self.r += dr;
        self.cam.position = vec_from_spherical(self.phi, self.theta, self.r);
    }
}

impl Default for CameraHandler {
    fn default() -> Self {
        Self::new(0.0, 0.0, consts::START_CAMERA_DISTANCE)
    }
}

