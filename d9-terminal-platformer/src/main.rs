use std::{io::Write, time::Duration};

use clap::Parser;

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear,
        ClearType::{self, All},
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use terminal_platformer::{
    draw_platform, generate_platforms, gravity, in_air, jump, left, on_bottom, on_platform, right,
    to_term_cords,
};

fn main() -> std::io::Result<()> {
    let config = terminal_platformer::GameConfig::parse();

    let mut out = std::io::stdout();
    let (prev_width, prev_height) = size()?;
    let (mut cols, mut rows) = (prev_width, prev_height);

    let platforms: Vec<_> = generate_platforms(&config, (cols, rows));

    let mut x = 0f32;
    let mut y = rows as f32;

    let mut velocity_y = 0f32;
    let mut velocity_x = 0f32;

    let mut timestamp = std::time::Instant::now();

    let mut pause = false;

    enable_raw_mode()?;
    queue!(out, EnterAlternateScreen, Hide, Clear(All),)?;
    out.flush()?;

    loop {
        use crossterm::event::{
            poll, read,
            Event::{Key, Resize},
            KeyCode::{Char, Esc, Left, Right, Up},
            KeyEvent, KeyModifiers,
        };

        if poll(Duration::from_millis(10))? {
            let event = read()?;
            if let Key(keys) = event {
                match keys {
                    KeyEvent {
                        code: Char('d') | Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }
                    | KeyEvent { code: Esc, .. } => break,
                    KeyEvent {
                        code: Char(' '), ..
                    } => pause = !pause,
                    KeyEvent {
                        code: Up | Char('w') | Char('k'),
                        ..
                    } if !in_air((x, y), (cols, rows), &platforms) => {
                        (velocity_x, velocity_y) = jump((velocity_x, velocity_y), &config)
                    }
                    KeyEvent {
                        code: Left | Char('a') | Char('h'),
                        ..
                    } => (velocity_x, velocity_y) = left((velocity_x, velocity_y)),
                    KeyEvent {
                        code: Right | Char('d') | Char('l'),
                        ..
                    } => (velocity_x, velocity_y) = right((velocity_x, velocity_y)),
                    _ => (),
                };
            } else if let Resize(new_cols, new_rows) = event {
                (cols, rows) = (new_cols, new_rows)
            }
        }

        if pause {
            continue;
        }

        queue!(out, Clear(ClearType::All))?;

        for platform in platforms.iter().copied() {
            draw_platform(&mut out, &config, (cols, rows), platform)?;
        }

        let dt = timestamp.elapsed().as_secs_f32();
        timestamp = std::time::Instant::now();

        if config.debug {
            queue!(
                out,
                MoveTo(0, 0),
                Print("dt: "),
                Print(dt),
                MoveToNextLine(1),
                Print("vGravity: ")
            )?;
        }
        if in_air((x, y), (cols, rows), &platforms) {
            let prev = (velocity_x, velocity_y);
            (velocity_x, velocity_y) = gravity((velocity_x, velocity_y), dt, &config);
            if config.debug {
                queue!(
                    out,
                    Print('('),
                    Print(prev.0 - velocity_x),
                    Print(','),
                    Print(prev.1 - velocity_y),
                    Print(')')
                )?;
            }
        } else if velocity_y < 0. {
            velocity_y = 0.;
            if config.debug {
                queue!(out, Print("reset"))?;
            }
        } else if config.debug {
            queue!(out, Print(false))?;
        }

        if config.debug {
            let (tx, ty) = to_term_cords((x, y), (cols, rows));
            queue!(
                out,
                MoveToNextLine(1),
                Print("pX: "),
                Print(x),
                MoveToNextLine(1),
                Print(" -> "),
                Print(x + velocity_x),
                MoveToNextLine(1),
                Print("pY: "),
                Print(y),
                MoveToNextLine(1),
                Print(" -> "),
                Print(y + velocity_y),
                MoveToNextLine(1),
                Print("vX: "),
                Print(velocity_x),
                MoveToNextLine(1),
                Print("vY: "),
                Print(velocity_y),
                MoveToNextLine(1),
                Print("air: "),
                Print(in_air((x, y), (cols, rows), &platforms)),
                Print(" ("),
                Print("bottom: "),
                Print(on_bottom((x, y), (cols, rows), &platforms)),
                Print(", platform: "),
                Print(on_platform((x, y), (cols, rows), &platforms)),
                Print(")"),
                MoveToNextLine(1),
                Print("tX: "),
                Print(tx),
                MoveToNextLine(1),
                Print("tY: "),
                Print(ty),
                MoveToNextLine(1),
                Print("platforms: "),
                Print(platforms.len()),
            )?;
        }

        y = (y + velocity_y).max(0.).min(rows as f32);
        x = (x + velocity_x).max(0.).min(cols as f32);

        let (x, y) = to_term_cords((x, y), (cols, rows));

        queue!(out, MoveTo(x, y), Print(config.player_symbol))?;

        out.flush()?;

        velocity_x = 0.;
    }
    queue!(out, Clear(All), Show, LeaveAlternateScreen,)?;
    disable_raw_mode()?;
    out.flush()
}
