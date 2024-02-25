mod consts;
mod camera_handler;
mod tools;
mod data_point;

use crate::data_point::DataPoint;
use crate::tools::vec_from_spherical;
use crate::camera_handler::CameraHandler;

use macroquad::prelude::*;

fn generate_test_sphere(segments: usize) -> Vec<DataPoint> {
    let mut data_points = vec![];
    
    let dp = std::f32::consts::PI * 2.0 / segments as f32;
    let dt = std::f32::consts::PI / segments as f32;

    for p in 0..(segments * 2) {
        for t in 0..segments {
            data_points.push(DataPoint::from_raw(vec_from_spherical(p as f32 * dp, t as f32 * dt, 1.0), p as f32));
        }
    }
    data_points
}



fn load_data_points() -> Vec<DataPoint> {
    vec![
        DataPoint::new(0.0, 0.0, 1.0, 10.0),
    ]
}



#[macroquad::main("3D")]
async fn main() {
    
    let mut cam_handler = CameraHandler::default();
    let data_points = generate_test_sphere(10);

    loop {
        // Update


        // Render
        clear_background(WHITE);
        cam_handler.incremental_update(0.01, 0.01, 0.005);
        set_camera(cam_handler.get_camera());

        for data_p in data_points.iter() {
            data_p.render();
        }

        // Back to screen space
        set_default_camera();
        draw_text("bob", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}

