use perlin_noise::perlin;

const SAMPLES: usize = 8;
const DISTANCE: usize = 4;
fn main() {
    let mut noise = [[0f32; SAMPLES]; SAMPLES];

    let x_offset = 0.;
    let y_offset = 0.;

    let delta = DISTANCE as f32 / SAMPLES as f32;
    for xs in 0..SAMPLES {
        let x = delta.mul_add(xs as f32, x_offset);
        for ys in 0..SAMPLES {
            let y = delta.mul_add(ys as f32, y_offset);
            noise[ys][xs] = perlin([x, y]);
        }
    }

    for row in noise {
        println!("{row:+.4?}");
    }
}
