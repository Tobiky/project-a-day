use std::io;

use clap::Parser;
use crossterm::{cursor::MoveTo, queue, style::Print};

#[derive(Parser)]
#[command(version)]
pub struct GameConfig {
    /// The gravity constant that affects the player.
    #[arg(short, long, default_value_t = 3.0)]
    pub gravity: f32,

    /// The displacement upwards when jumping.
    #[arg(short, long, default_value_t = 0.40)]
    pub jump: f32,

    /// Symbol to use when drawing the player.
    #[arg(short, long, default_value_t = 'A')]
    pub player_symbol: char,

    /// Enable/Disable debugging
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// Symbol to use when drawing platforms.
    #[arg(short = 'l', long, default_value_t = '▀')]
    pub platform_symbol: char,

    /// Seed to use when generating platforms.
    #[arg(short = 'e', long)]
    pub seed: Option<u64>,

    /// Max amount of platforms to generate. Defaults to 10.
    #[arg(short = 's', long, default_value_t = 7)]
    pub platforms: u8,

    /// Max length of platforms. Defaults to 7.
    #[arg(short, long, default_value_t = 7)]
    pub max_platform_length: u8,
}

pub fn gravity(
    #[allow(unused_variables)] velocity @ (dx, dy): (f32, f32),
    dt: f32,
    config: &GameConfig,
) -> (f32, f32) {
    (dx, f32::mul_add(-config.gravity, dt, dy))
}

pub fn jump(
    #[allow(unused_variables)] velocity @ (dx, dy): (f32, f32),
    config: &GameConfig,
) -> (f32, f32) {
    (dx, dy + config.jump)
}

pub fn left(#[allow(unused_variables)] velocity @ (_, dy): (f32, f32)) -> (f32, f32) {
    (-1.0, dy)
}

pub fn right(#[allow(unused_variables)] velocity @ (_, dy): (f32, f32)) -> (f32, f32) {
    (1.0, dy)
}

pub fn on_bottom(
    #[allow(unused_variables)] position @ (fx, fy): (f32, f32),
    #[allow(unused_variables)] screen @ (cols, rows): (u16, u16),
    #[allow(unused_variables)] platforms: &[(u16, u16, u16, u16)],
) -> bool {
    let (_, y) = (
        (fx.floor() as u16).min(cols).max(0),
        (fy.floor() as u16).min(rows).max(0),
    );

    y == 0
}

pub fn on_platform(
    #[allow(unused_variables)] position @ (fx, fy): (f32, f32),
    #[allow(unused_variables)] screen @ (cols, rows): (u16, u16),
    platforms: &[(u16, u16, u16, u16)],
) -> bool {
    let (x, y) = (
        (fx.floor() as u16).min(cols).max(0),
        (fy.floor() as u16).min(rows).max(0),
    );

    platforms
        .iter()
        .copied()
        .any(|(px, py, dx, _)| y == py + 1 && px < x && x < px + dx)
}

pub fn in_air(
    position: (f32, f32),
    screen: (u16, u16),
    platforms: &[(u16, u16, u16, u16)],
) -> bool {
    !on_bottom(position, screen, platforms) && !on_platform(position, screen, platforms)
}

pub fn draw_platform(
    out: &mut impl io::Write,
    config: &GameConfig,
    #[allow(unused_variables)] window @ (cols, rows): (u16, u16),
    #[allow(unused_variables)] platform @ (x, y, dx, _): (u16, u16, u16, u16),
) -> io::Result<()> {
    let mut buffer = [0; 4];
    let platform_string = config
        .platform_symbol
        .encode_utf8(&mut buffer)
        .repeat(dx.min(cols) as usize);

    queue!(out, MoveTo(x, rows - y), Print(platform_string))
}

pub fn to_term_cords(
    #[allow(unused_variables)] position @ (x, y): (f32, f32),
    #[allow(unused_variables)] window @ (cols, rows): (u16, u16),
) -> (u16, u16) {
    let (x, y) = (x.floor() as u16, y.floor() as u16);
    let (x, y) = (x, rows - y);
    let (x, y) = (x.min(cols), y.min(rows));

    (x, y)
}

pub fn generate_platforms(
    config: &GameConfig,
    #[allow(unused_variables)] window @ (cols, rows): (u16, u16),
) -> Vec<(u16, u16, u16, u16)> {
    config.seed.map(fastrand::seed);

    let mut platforms = vec![];
    let mut line = 3;
    let mut walk = 0;

    while platforms.len() < config.platforms as usize {
        let dx = fastrand::u16(1..config.max_platform_length as u16);
        let end = cols.min(walk + dx);

        if fastrand::bool() {
            platforms.push((walk, line, end - walk, 0))
        }

        walk = end + 1;

        if walk >= cols {
            line += 2;
            walk = 0;
        }
    }

    platforms.push((0, 1, cols, 0));

    platforms
}
