use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct GameConfig {
    /// The gravity constant that affects the player.
    #[arg(short, long, default_value_t = 9.86)]
    pub gravity: f32,

    /// The displacement upwards when jumping.
    #[arg(short, long, default_value_t = 1.0)]
    pub jump: f32,

    /// Symbol to use when drawing the player.
    #[arg(short, long, default_value_t = b'A')]
    pub player_symbol: u8,

    // /// Symbol to use when clearing pixels/cells
    // #[arg(short, long, default_value_t = b' ')]
    // pub empty: u8,
    /// Enable/Disable debugging
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}

pub fn gravity((dx, dy): (f32, f32), dt: f32, config: &GameConfig) -> (f32, f32) {
    (dx, f32::mul_add(-config.gravity, dt, dy))
}

pub fn jump((dx, dy): (f32, f32), config: &GameConfig) -> (f32, f32) {
    (dx, dy + config.jump)
}

pub fn left((_, dy): (f32, f32)) -> (f32, f32) {
    (-1.0, dy)
}

pub fn right((_, dy): (f32, f32)) -> (f32, f32) {
    (1.0, dy)
}

pub fn in_air((_, fy): (f32, f32)) -> bool {
    fy.floor() > 0.0
}

pub fn to_term_cords((x, y): (f32, f32), rows: u16) -> (u16, u16) {
    let (x, y) = (x.floor() as u16, y.floor() as u16);

    (x, rows - y)
}
