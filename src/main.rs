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
    
    let dp = std::f32::consts::PI / segments as f32;
    let dt = std::f32::consts::PI / segments as f32;

    for p in 0..segments * 2 {
        for t in 0..segments {
            data_points.push(DataPoint::from_vec3(vec_from_spherical(p as f32 * dp, t as f32 * dt, 1.0), p as f32));
        }
    }
    data_points
}

fn max_value_in_data_points(data_points: &[DataPoint]) -> Option<f32> {
    data_points.iter()
               .max_by(|p1, p2| p1.value().partial_cmp(&p2.value()).unwrap())
               .and_then(|p| Some(p.value()))
}


#[macroquad::main("3D")]
async fn main() {
    let mut cam_handler = CameraHandler::default();
    let data_points = generate_test_sphere(10);
    let max_value = max_value_in_data_points(&data_points).unwrap();

    loop {
        // Update
        cam_handler.process_mouse();

        // Render
        clear_background(WHITE);
        set_camera(cam_handler.get_camera());

        for data_p in data_points.iter() {
            data_p.render(max_value);
        }

        // Back to screen space
        set_default_camera();
        draw_text("bob", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}

