use macroquad::prelude::*;

use kdtree::KdTree;

pub struct CubedSphereMeshBuilder {
    // List of indices of DataPoints that form parallelograms
    cells: Vec<[usize; 4]>,
    textures: Vec<Texture2D>,
}

