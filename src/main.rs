
fn viewshed_of_slice(elevation_angle_slice: &[f32]) -> Vec<i8> {
    
    let mut max_angle = f32::NEG_INFINITY;
    let mut viewshed = vec![0; elevation_angle_slice.len()];
    for (idx, angle) in elevation_angle_slice.iter().enumerate() {
        if angle >= &max_angle {
            viewshed[idx] = 1;
            max_angle = *angle;
        }
    }
    viewshed
}

fn main() {
    println!("Calculating the viewshed!");

    // terrain and viewpoint will eventually be user suplied
    let terrain = vec![1., 2., 3., 5., 7., 6., 5., 5., 8., 20., 32., 7., 5.];
    let viewpoint = (6,);

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

    viewshed[viewpoint.0..].copy_from_slice(&viewshed_of_slice(&elevation_angle[viewpoint.0..]));
    
    let mut elevation_angle_slice: Vec<f32> = Vec::new();
    elevation_angle_slice.extend_from_slice(&elevation_angle[..viewpoint.0]);
    elevation_angle_slice.reverse();
    let mut slice_viewshed = viewshed_of_slice(&elevation_angle_slice);
    slice_viewshed.reverse();
    viewshed[..viewpoint.0].copy_from_slice(&slice_viewshed);

    println!("Terrain: {:?}", terrain);
    println!("Viewshed: {:?}", viewshed);
}