

use crate::algorithms::viewshed_1d;

fn main() {
    println!("Calculating the viewshed!");

    // terrain and viewpoint will eventually be user suplied
    let terrain = vec![1., 2., 3., 5., 7., 6., 5., 5., 8., 20., 32., 7., 5.];
    let viewpoint = (6 as usize,);

    let viewshed = viewshed_1d(&terrain, viewpoint);

    println!("Terrain: {:?}", terrain);
    println!("Viewshed: {:?}", viewshed);
}

mod algorithms {

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