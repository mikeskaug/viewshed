use std::env;

extern crate ndarray;

use crate::algorithms::viewshed_2d;
use crate::io::load_terrain;

fn main() {
    println!("Calculating the viewshed!");

    let args: Vec<String> = env::args().collect();
    let terrain_geotiff_filepath = &args[1];

    let load_result = load_terrain(&terrain_geotiff_filepath);
    let (terrain, geo_transform) = match load_result {
        Ok((terrain, geo_transform)) => (terrain, geo_transform),
        Err(e) => panic!("There was a problem loading terrain from file: {:?}", e),
    };

    let viewpoint = (100 as usize, 100 as usize);

    let viewshed = viewshed_2d(&terrain, viewpoint);

    println!("Terrain: \n{:?}", terrain);
    println!("Viewpoint: {:?}", viewpoint);
    println!("Viewshed: \n{:?}", viewshed);
}

mod io {    
    use gdal::{Dataset, GeoTransform};
    use gdal::errors::GdalError;
    use gdal::raster::ResampleAlg::NearestNeighbour;
    use ndarray::Array2;

    pub fn load_terrain(filename: &str) -> Result<(Array2<f64>, GeoTransform), GdalError> {
        println!("Loading terrain file: {:?}", filename );
        
        let ds = Dataset::open(filename)?;
        let geo_transform = ds.geo_transform()?;
        let terrain_raster = ds.rasterband(1)?;

        let (x_size, y_size) = terrain_raster.size();
        let terrain_array = terrain_raster.read_as_array((0,0), (x_size,y_size), (x_size,y_size), Some(NearestNeighbour))?;

        let terrain_array_f64 = terrain_array.mapv(|x: i16| f64::from(x));

        Ok((terrain_array_f64, geo_transform))
    }
}

mod transforms {
    pub fn to_viewpoint(coordinate: (usize, usize), viewpoint: (usize, usize)) -> (isize, isize) {
        let transformed = (coordinate.0 as isize - viewpoint.0 as isize, coordinate.1 as isize - viewpoint.1 as isize);
        transformed
    }

    pub fn to_origin(coordinate: (isize, isize), viewpoint: (usize, usize)) -> (usize, usize) {
        let transformed = (coordinate.0 + viewpoint.0 as isize, coordinate.1 + viewpoint.1 as isize);
        let transformed = (transformed.0 as usize, transformed.1 as usize);
        transformed
    }
}

mod algorithms {
    use std::env;
    use ndarray::{Array, Array2, Zip};
    use crate::transforms::{to_origin, to_viewpoint};

    pub fn viewshed_2d(terrain: &Array2<f64>, viewpoint: (usize,usize)) -> Array2<u8> {
        let viewpoint_h = &terrain[[viewpoint.0, viewpoint.1]];

        let mut elevation_angle = Array::<f64, _>::zeros(terrain.dim());
        let mut viewshed = Array::<u8, _>::zeros(terrain.dim());

        // calculate the elevation angle from the viewpoint to each point in the terrain
        for ((idx, idy), terrain_height) in terrain.indexed_iter() {
            if (idx, idy) == viewpoint {
                elevation_angle[[idx, idy]] = 0.0;
                continue;
            }
            let del_h = terrain_height - viewpoint_h;
            let del_d = (((idx as f64) - (viewpoint.0 as f64)).powf(2.0) + ((idy as f64) - (viewpoint.1 as f64)).powf(2.0)).sqrt();
            let theta = (del_h / del_d).atan();
            elevation_angle[[idx, idy]] = theta;
        }

        // traverse down the RIGHT EDGE of the terrain
        // for each edge cell, find the ray from the viewpoint to the edge cell
        // traverse the ray and determine visibility based on max angle
        for idy in 0..elevation_angle.shape()[1] {
            let (idx, idy) = to_viewpoint((elevation_angle.shape()[0]-1, idy), viewpoint);
            let del_y = idy as f64; //del_y = idy because we shifted origin to the viewpoint
            let del_x = idx as f64;
            let viewpoint_to_edge_angle = del_y.atan2(del_x);
            let ray_idxs = Array::range(1.0, (idx as f64) + 1.0, 1.0);
            let ray_idys = &ray_idxs * f64::tan(viewpoint_to_edge_angle);
            
            let debug = env::var("DEBUG").is_ok();
            if debug {
                println!("idx: {}, idy: {}", idx, idy);
                println!("viewpoint_to_edge_angle: {}", viewpoint_to_edge_angle);
                println!("ray_idxs: {:?}", ray_idxs);
                println!("ray_idys: {:?}", ray_idys);
            }
            
            let mut max_angle = f64::NEG_INFINITY;
            Zip::from(&ray_idxs)
                .and(&ray_idys)
                .for_each(|&ray_idx, &ray_idy| {
                    let (ray_idx, ray_idy) = to_origin(((ray_idx.round() as isize).try_into().unwrap(), (ray_idy.round() as isize).try_into().unwrap()), viewpoint);
                    let angle = elevation_angle[[ray_idx, ray_idy]];
                    if angle >= max_angle {
                        viewshed[[ray_idx, ray_idy]] = 1;
                        max_angle = angle;
                    }
                });
        }

        // traverse across the TOP of the terrain
        // for each edge cell, find the ray from the viewpoint to the edge cell
        // traverse the ray and determine visibility based on max angle
        for idx in 0..elevation_angle.shape()[0] {
            let (idx, idy) = to_viewpoint((idx, elevation_angle.shape()[1]-1), viewpoint);
            let del_y = idy as f64; //del_y = idy because we shifted origin to the viewpoint
            let del_x = idx as f64;
            let viewpoint_to_edge_angle = del_y.atan2(del_x);
            let ray_idys = Array::range(1.0, (idy as f64) + 1.0, 1.0);
            let ray_idxs = &ray_idys / f64::tan(viewpoint_to_edge_angle);
            
            let debug = env::var("DEBUG").is_ok();
            if debug {
                println!("idx: {}, idy: {}", idx, idy);
                println!("viewpoint_to_edge_angle: {}", viewpoint_to_edge_angle);
                println!("ray_idxs: {:?}", ray_idxs);
                println!("ray_idys: {:?}", ray_idys);
            }
            
            let mut max_angle = f64::NEG_INFINITY;
            Zip::from(&ray_idxs)
                .and(&ray_idys)
                .for_each(|&ray_idx, &ray_idy| {
                    let (ray_idx, ray_idy) = to_origin(((ray_idx.round() as isize).try_into().unwrap(), (ray_idy.round() as isize).try_into().unwrap()), viewpoint);
                    let angle = elevation_angle[[ray_idx, ray_idy]];
                    if angle >= max_angle {
                        viewshed[[ray_idx, ray_idy]] = 1;
                        max_angle = angle;
                    }
                });
        }

        // traverse down the LEFT EDGE of the terrain
        // for each edge cell, find the ray from the viewpoint to the edge cell
        // traverse the ray and determine visibility based on max angle
        for idy in 0..elevation_angle.shape()[1] {
            let (idx, idy) = to_viewpoint((0, idy), viewpoint);
            let del_y = idy as f64; //del_y = idy because we shifted origin to the viewpoint
            let del_x = idx as f64;
            let viewpoint_to_edge_angle = del_y.atan2(del_x);
            let ray_idxs = Array::range(-1.0, (idx as f64) - 1.0, -1.0);
            let ray_idys = &ray_idxs * f64::tan(viewpoint_to_edge_angle);
            
            let debug = env::var("DEBUG").is_ok();
            if debug {
                println!("idx: {}, idy: {}", idx, idy);
                println!("viewpoint_to_edge_angle: {}", viewpoint_to_edge_angle);
                println!("ray_idxs: {:?}", ray_idxs);
                println!("ray_idys: {:?}", ray_idys);
            }
            
            let mut max_angle = f64::NEG_INFINITY;
            Zip::from(&ray_idxs)
                .and(&ray_idys)
                .for_each(|&ray_idx, &ray_idy| {
                    let (ray_idx, ray_idy) = to_origin(((ray_idx.round() as isize).try_into().unwrap(), (ray_idy.round() as isize).try_into().unwrap()), viewpoint);
                    let angle = elevation_angle[[ray_idx, ray_idy]];
                    if angle >= max_angle {
                        viewshed[[ray_idx, ray_idy]] = 1;
                        max_angle = angle;
                    }
                });
        }

        // traverse across the BOTTOM of the terrain
        // for each edge cell, find the ray from the viewpoint to the edge cell
        // traverse the ray and determine visibility based on max angle
        for idx in 0..elevation_angle.shape()[0] {
            let (idx, idy) = to_viewpoint((idx, 0), viewpoint);
            let del_y = idy as f64; //del_y = idy because we shifted origin to the viewpoint
            let del_x = idx as f64;
            let viewpoint_to_edge_angle = del_y.atan2(del_x);
            let ray_idys = Array::range(-1.0, (idy as f64) - 1.0, -1.0);
            let ray_idxs = &ray_idys / f64::tan(viewpoint_to_edge_angle);
            
            let debug = env::var("DEBUG").is_ok();
            if debug {
                println!("idx: {}, idy: {}", idx, idy);
                println!("viewpoint_to_edge_angle: {}", viewpoint_to_edge_angle);
                println!("ray_idxs: {:?}", ray_idxs);
                println!("ray_idys: {:?}", ray_idys);
            }
            
            let mut max_angle = f64::NEG_INFINITY;
            Zip::from(&ray_idxs)
                .and(&ray_idys)
                .for_each(|&ray_idx, &ray_idy| {
                    let (ray_idx, ray_idy) = to_origin(((ray_idx.round() as isize).try_into().unwrap(), (ray_idy.round() as isize).try_into().unwrap()), viewpoint);
                    let angle = elevation_angle[[ray_idx, ray_idy]];
                    if angle >= max_angle {
                        viewshed[[ray_idx, ray_idy]] = 1;
                        max_angle = angle;
                    }
                });
        }

        viewshed
    }

    pub fn viewshed_1d(terrain: &Vec<f32>, viewpoint: (usize,)) -> Vec<i32> {
        let viewpoint_h = &terrain[viewpoint.0];
        let mut elevation_angle = vec![0.0; terrain.len()];
        let mut viewshed = vec![0; terrain.len()];

        for (idx, terrain_height) in terrain.iter().enumerate() {
            if idx == viewpoint.0 {
                elevation_angle[idx] = 0.0;
                continue;
            }
            let del_h = terrain_height - viewpoint_h;
            let del_d = ((idx as f32) - (viewpoint.0 as f32)).abs();
            let theta = (del_h / del_d).atan();
            elevation_angle[idx] = theta;
        }

        // Determine visibility to the right from viewpoint
        let mut max_angle = f32::NEG_INFINITY;
        for idx in viewpoint.0..elevation_angle.len() {
            let angle = elevation_angle[idx];
            if angle >= max_angle {
                viewshed[idx] = 1;
                max_angle = angle;
            }
        }

        // Determine visibility to the left from viewpoint
        let mut max_angle = f32::NEG_INFINITY;
        for idx in (0..viewpoint.0).rev() {
            let angle = elevation_angle[idx];
            if angle >= max_angle {
                viewshed[idx] = 1;
                max_angle = angle;
            }
        }
        viewshed
    }
}