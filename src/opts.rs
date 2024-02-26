use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Spherical")]
pub struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: PathBuf,
    #[structopt(name = "VARIABLE")]
    pub variable: String,

    #[structopt(short, long, default_value = "0")]
    pub timestep: usize,

    #[structopt(short, long, default_value = "0")]
    pub level: usize,
    #[structopt(short, long)]
    pub centered_colour_map: bool,
    #[structopt(short, long, default_value = "5")]
    pub markersize: usize,
    
    #[structopt(short, long, default_value = "Mesh2d_face_x")]
    pub lon_name: String,
    #[structopt(short, long, default_value = "Mesh2d_face_y")]
    pub lat_name: String,
    #[structopt(short, long, default_value = "time_instant")]
    pub time_name: String,
}

