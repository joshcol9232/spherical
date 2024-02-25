use macroquad::prelude::*;
use macroquad::math::Vec3;

use crate::consts;
use crate::tools::vec_from_lat_lon_r;

pub struct DataPoint {
    location: Vec3,
    value: f32,
}

impl DataPoint {
    pub fn new(lat: f32, lon: f32, r: f32, value: f32) -> Self {
        Self {
            location: vec_from_lat_lon_r(lat, lon, r),
            value,
        }
    }

    pub fn from_raw(location: Vec3, value: f32) -> Self {
        Self { location, value }
    }

    fn get_colour(&self) -> Color {
        BLUE
    }

    pub fn render(&self) {
        draw_sphere(self.location, consts::MARKER_SIZE, None, self.get_colour());
    }
}


