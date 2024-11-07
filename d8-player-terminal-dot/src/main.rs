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
use terminal_platformer::{gravity, in_air, jump, left, right, to_term_cords};

fn main() -> std::io::Result<()> {
    let config = terminal_platformer::GameConfig::parse();

    let mut out = std::io::stdout();
    let (prev_width, prev_height) = size()?;
    let (mut cols, mut rows) = (prev_width, prev_height);

    let mut x = 0f32;
    let mut y = 0 as f32;

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
                    } if !in_air((x, y)) => {
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

        let dt = timestamp.elapsed().as_secs_f32();
        timestamp = std::time::Instant::now();

        if config.debug {
            queue!(out, MoveTo(0, 0), Print("gravity: "))?;
        }
        if in_air((x, y)) {
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
                queue!(out, Print("reset            "))?;
            }
        } else if config.debug {
            queue!(out, Print(false), Print("            "))?;
        }

        y = (y + velocity_y).max(0.).min(rows as f32);
        x = (x + velocity_x).max(0.).min(cols as f32);

        if config.debug {
            queue!(
                out,
                MoveToNextLine(1),
                Print(" X: "),
                Print(x),
                MoveToNextLine(1),
                Print(" -> X: "),
                Print(x + velocity_x),
                MoveToNextLine(1),
                Print(" Y: "),
                Print(y),
                MoveToNextLine(1),
                Print(" -> Y: "),
                Print(y + velocity_y),
                MoveToNextLine(1),
                Print("vX: "),
                Print(velocity_x),
                MoveToNextLine(1),
                Print("vY: "),
                Print(velocity_y),
                MoveToNextLine(1),
                Print("air: "),
                Print(in_air((x, y))),
            )?;
        }

        let (x, y) = to_term_cords((x, y), rows);

        if config.debug {
            queue!(
                out,
                MoveToNextLine(1),
                Print("tX: "),
                Print(x),
                MoveToNextLine(1),
                Print("tY: "),
                Print(y)
            )?;
        }

        queue!(out, MoveTo(x, y), Print(config.player_symbol as char))?;

        out.flush()?;

        velocity_x = 0.;
    }
    queue!(out, Clear(All), Show, LeaveAlternateScreen,)?;
    disable_raw_mode()?;
    out.flush()
}
