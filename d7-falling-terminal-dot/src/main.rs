use std::{cmp::min, io::Write, time::Duration};

use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read},
    execute, queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

#[derive(Parser)]
#[command(version)]
struct Config {
    /// Falling object symbol.
    #[arg(short, long, default_value_t = '▪')]
    cursor: char,

    /// Empty space symbol.
    #[arg(short, long, default_value_t = ' ')]
    empty: char,

    /// Gravitational constant.
    #[arg(short, long, default_value_t = 9.86)]
    gravity: f64,
}

fn main() -> std::io::Result<()> {
    let config = Config::parse();

    let mut out = std::io::stdout();
    enable_raw_mode()?;
    let (mut cols, mut rows) = size()?;
    let (mut x, mut y) = (0, 0.0);
    let mut momentum_y = 0.0;

    queue!(
        out,
        EnterAlternateScreen,
        Hide,
        Clear(ClearType::All),
        MoveTo(x, y as u16),
        Print(config.cursor)
    )?;
    out.flush()?;

    let mut timestamp = std::time::Instant::now();
    loop {
        use crossterm::event::{
            Event::{Key, Resize},
            KeyCode::{Char, Esc},
            KeyEvent, KeyModifiers,
        };

        if poll(Duration::from_millis(10))? {
            match read()? {
                Key(
                    KeyEvent {
                        code: Char('d') | Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }
                    | KeyEvent { code: Esc, .. },
                ) => break,
                Resize(new_cols, new_rows) => (cols, rows) = (new_cols, new_rows),
                _ => (),
            };
        }

        // Clear out current spot
        queue!(out, Print(config.empty))?;

        let dt = timestamp.elapsed().as_secs_f64();
        timestamp = std::time::Instant::now();

        momentum_y = f64::mul_add(config.gravity, dt, momentum_y);
        y = (y + momentum_y).min(rows as f64);

        queue!(
            out,
            Clear(ClearType::All),
            MoveTo(x, y as u16),
            Print(config.cursor)
        )?;
        out.flush()?;
    }

    queue!(out, Clear(ClearType::All), Show, LeaveAlternateScreen)?;
    out.flush()?;
    disable_raw_mode()
}
