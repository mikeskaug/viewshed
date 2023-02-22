extern crate ndarray;

use ndarray::{array, Array};

use crate::algorithms::viewshed_2d;

fn main() {
    println!("Calculating the viewshed!");

    // terrain and viewpoint will eventually be user suplied
    // let terrain = vec![1., 2., 3., 5., 7., 6., 5., 5., 8., 20., 32., 7., 5.];
    // let viewpoint = (6 as usize,);

    // let viewshed = viewshed_1d(&terrain, viewpoint);

    let terrain: Array<f64, _> = array![[1., 2., 3., 5., 7.], [6., 5., 5., 8., 20.], [32., 7., 5., 3., 10.]];
    let viewpoint = (2 as usize, 3 as usize);

    let viewshed = viewshed_2d(&terrain, viewpoint);

    println!("Terrain: {:?}", terrain);
    println!("Viewshed: {:?}", viewshed);
}

mod algorithms {
    use ndarray::{Array, Array2, Zip};

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
            let del_d = ((idx as f64) - (viewpoint.0 as f64)).abs();
            let theta = (del_h / del_d).atan();
            elevation_angle[[idx, idy]] = theta;
        }

        // traverse down the left edge of the terrain
        // for each edge cell, find the ray from the viewpoint to the edge cell
        // traverse the ray and determine visibility based on max angle
        for idy in 0..elevation_angle.shape()[0] {
            let idx = 0;
            let del_y = (idy as f64) - (viewpoint.1 as f64);
            let del_x = (idx as f64) - (viewpoint.0 as f64);
            let viewpoint_to_edge_angle = del_y.atan2(del_x);
            let ray_idxs = Array::range(idx as f64, (viewpoint.0 as f64) + 1.0, 1.0);
            let ray_idys = &ray_idxs * f64::tan(viewpoint_to_edge_angle);
            
            println!("idx: {}, idy: {}", idx, idy);
            println!("viewpoint_to_edge_angle: {}", viewpoint_to_edge_angle);
            println!("ray_idxs: {:?}", ray_idxs);
            println!("ray_idys: {:?}", ray_idys);
            
            let mut max_angle = f64::NEG_INFINITY;
            Zip::from(&ray_idxs)
                .and(&ray_idys)
                .for_each(|&ray_idx, &ray_idy| {
                    let ray_idx = ray_idx as usize;
                    let ray_idy = ray_idy as usize;
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