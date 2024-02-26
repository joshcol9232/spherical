use macroquad::prelude::*;
use macroquad::math::Vec3;

use crate::consts;
use crate::tools;

pub struct DataPoint {
    location: Vec3,
    value: f32,
}

impl DataPoint {
    pub fn new(lat: f32, lon: f32, r: f32, value: f32) -> Self {
        Self::from_vec3(tools::vec_from_lat_lon_r(lat, lon, r), value)
    }

    pub fn from_vec3(location: Vec3, value: f32) -> Self {
        Self {
            location,
            value,
        }
    }

    pub fn value(&self) -> f32 { self.value }

    fn calc_colour(&self, max_value: f32) -> Color {
        tools::value_to_colour(self.value, max_value, &consts::COLOUR_MAP)
    }

    pub fn render_point(&self, max_value: f32, marker_size: usize) {
        draw_sphere(self.location, consts::MARKER_SIZE_SCALE * marker_size as f32, None, self.calc_colour(max_value));
    }
}


