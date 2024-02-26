use crate::data_point::DataPoint;
use crate::opts;
use crate::consts;

use netcdf;

pub fn load_from_file_opts(opt: &opts::Opt) -> netcdf::Result<Vec<DataPoint>> {
    let file = netcdf::open(&opt.file)?;
    let var = &file.variable(&opt.variable)
        .expect(&format!("Could not load variable '{}'", opt.variable));

    let lon = &file.variable(&opt.lon_name)
        .expect(&format!("Could not load variable '{}'", opt.lon_name));
    let lat = &file.variable(&opt.lat_name)
        .expect(&format!("Could not load variable '{}'", opt.lat_name));

    let lons = lon.get::<f32, _>(..)?;
    let lats = lat.get::<f32, _>(..)?;
    let data = var.get::<f32, _>((opt.timestep, opt.level, ..))?;

    // Load into DataPoints
    let mut points: Vec<DataPoint> = vec![];

    for (i, d) in data.into_iter().enumerate() {
        println!("i {}", i);
        println!("Loc: {} {}", lats[i], lons[i]);
        points.push(DataPoint::new(lats[i], lons[i], consts::RADIUS, d));
    }
    Ok(points)
}

