use macroquad::math::Vec3;

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
