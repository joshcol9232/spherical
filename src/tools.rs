use macroquad::math::Vec3;

use colorous::Gradient;

pub fn vec_from_spherical(phi: f32, theta: f32, r: f32) -> Vec3 {
    let phi_s = phi.sin();
    Vec3 {
        x: phi_s * theta.cos() * r,
        y: phi_s * theta.sin() * r,
        z: phi.cos() * r,
    }
}

pub fn vec_from_lat_lon_r(lat: f32, lon: f32, r: f32) -> Vec3 {
    vec_from_spherical(((lon - 90.0)/180.0) * std::f32::consts::PI,
                        (lat/360.0) * std::f32::consts::PI * 2.0,
                        r)
}

pub fn value_to_colour(value: f32, max_value: f32, gradient: &Gradient) -> macroquad::color::Color {
    let colorous_colour = gradient.eval_continuous(value as f64 / max_value as f64);
    macroquad::color::Color {
        r: colorous_colour.r as f32 / 255.0,
        g: colorous_colour.g as f32 / 255.0,
        b: colorous_colour.b as f32 / 255.0,
        a: 1.0
    }
}

