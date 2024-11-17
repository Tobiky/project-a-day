fn lerp(a: f32, b: f32, weight: f32) -> f32 {
    weight.clamp(0., 1.).mul_add(b - a, a)
}

fn blerp(tl: f32, tr: f32, bl: f32, br: f32, x_weight: f32, y_weight: f32) -> f32 {
    let lower = lerp(bl, br, x_weight);
    let upper = lerp(tl, tr, x_weight);
    let middle = lerp(lower, upper, y_weight);

    middle
}

fn random_gradient(#[allow(unused_variables)] grid_point @ [x, y]: [i32; 2]) -> [f32; 2] {
    fastrand::seed((x as u64) << 32 | y as u64);
    let random = fastrand::f32() * 2. * std::f32::consts::PI;
    [random.cos(), random.sin()]
}

fn dot(
    #[allow(unused_variables)] a @ [x1, y1]: [f32; 2],
    #[allow(unused_variables)] b @ [x2, y2]: [f32; 2],
) -> f32 {
    x1.mul_add(x2, y1 * y2)
}

fn grid_point_gradient(
    #[allow(unused_variables)] grid_point @ [gx, gy]: [i32; 2],
    #[allow(unused_variables)] point @ [px, py]: [f32; 2],
) -> f32 {
    let grid_point_gradient = random_gradient(grid_point);
    let grid_point_distance = [px - gx as f32, py - gy as f32];
    dot(grid_point_distance, grid_point_gradient)
}

pub fn perlin(point @ [px, py]: [f32; 2]) -> f32 {
    let (gx1, gy1) = (px.floor() as i32, py.floor() as i32);
    let (gx2, gy2) = (gx1 + 1, gy1 + 1);

    let x_weight = px - gx1 as f32;
    let y_weight = py - gy1 as f32;

    // grid bottom left, bottom right, top left, top right
    let gbl = grid_point_gradient([gx1, gy1], point);
    let gbr = grid_point_gradient([gx2, gy1], point);
    let gtl = grid_point_gradient([gx1, gy2], point);
    let gtr = grid_point_gradient([gx2, gy2], point);

    blerp(gtl, gtr, gbl, gbr, x_weight, y_weight)
}
