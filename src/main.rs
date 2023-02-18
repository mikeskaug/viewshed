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
    let viewpoint = (2 as usize, 2 as usize);

    let viewshed = viewshed_2d(&terrain, viewpoint);

    println!("Terrain: {:?}", terrain);
    println!("Viewshed: {:?}", viewshed);
}

mod algorithms {
    use ndarray::{Array, Array2};

    pub fn viewshed_2d(terrain: &Array2<f64>, viewpoint: (usize,usize)) -> Array2<u8> {
        let viewpoint_h = &terrain[[viewpoint.0, viewpoint.1]];

        let mut elevation_angle = Array::<f64, _>::zeros(terrain.dim());
        let mut viewshed = Array::<u8, _>::zeros(terrain.dim());

        // calculate the elevation angle for each point in the terrain
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

        // Iterate and determine visibility
        
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