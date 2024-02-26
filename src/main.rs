mod consts;
mod camera_handler;
mod tools;
mod data_point;
mod opts;
mod loader;

use crate::data_point::DataPoint;
use crate::tools::vec_from_spherical;
use crate::camera_handler::CameraHandler;

use macroquad::prelude::*;
use structopt::StructOpt;

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

fn render_axis() {
    const X: Vec3 = vec3(1.0, 0.0, 0.0);
    const Y: Vec3 = vec3(0.0, 1.0, 0.0);
    const Z: Vec3 = vec3(0.0, 0.0, 1.0);

    const SCALE: f32 = 3.0;
    const ORIGIN_POINT: Vec3 = vec3(-SCALE / 2.0, -SCALE / 2.0, -SCALE / 2.0);

    draw_line_3d(ORIGIN_POINT, ORIGIN_POINT + X * SCALE, GRAY);
    draw_line_3d(ORIGIN_POINT, ORIGIN_POINT + Y * SCALE, GRAY);
    draw_line_3d(ORIGIN_POINT, ORIGIN_POINT + Z * SCALE, GRAY);
}


enum RenderingMode {
    PointCloud,
    CubedSphere
}


#[macroquad::main("Spherical")]
async fn main() {
    let opt = opts::Opt::from_args();

    let mut cam_handler = CameraHandler::default();

    /*
    let data_points = generate_test_sphere(10);
    */
    let data_points = loader::load_from_file_opts(&opt).expect("Couldn't load points from file...");

    let max_value = max_value_in_data_points(&data_points).unwrap();

    const RENDERING_MODE: RenderingMode = RenderingMode::PointCloud;
    //const RENDERING_MODE: RenderingMode = RenderingMode::CubedSphere;

    loop {
        // Update
        cam_handler.process_mouse();

        // Render
        clear_background(WHITE);
        set_camera(cam_handler.get_camera());
        render_axis();

        match RENDERING_MODE {
            RenderingMode::PointCloud => {
                for data_p in data_points.iter() {
                    data_p.render_point(max_value, opt.markersize);
                }
            },
            RenderingMode::CubedSphere => {
                let bytes: Vec<u8> = vec![255, 0, 0, 255,
                                          0, 255, 0, 255,
                                          0, 0, 255, 255,
                                          255, 255, 255, 255];
                let texture = Texture2D::from_rgba8(2, 2, &bytes);
                draw_affine_parallelogram(vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(1.0, 1.0, 1.0), Some(&texture), WHITE);
            },
        }

        // Back to screen space
        set_default_camera();
        draw_text("bob", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}

