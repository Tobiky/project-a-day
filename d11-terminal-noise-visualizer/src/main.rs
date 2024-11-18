use std::io::{self, Write};

use clap::Parser;
use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    QueueableCommand,
};
use terminal_noise_visualizer::draw_noise;

#[derive(Parser)]
struct CLI {
    /// Log2 value for scaling the perlin noise coordinate stepping. Higher values zoom in. Default 1, min 1.
    #[arg(long, short, default_value_t = 1)]
    pub scale_log2: u8,
}

fn main() -> io::Result<()> {
    let cli = CLI::parse();
    let mut out = io::stdout();
    let screen = size()?;

    enable_raw_mode()?;
    queue!(out, Hide, EnterAlternateScreen)?;

    draw_noise(&mut out, screen, cli.scale_log2)?;
    out.flush()?;

    loop {
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break,
            Event::Resize(cols, rows) => {
                let screen = (cols, rows);
                out.queue(Clear(ClearType::All))?;
                draw_noise(&mut out, screen, cli.scale_log2)?;
                out.flush()?;
            }
            _ => (),
        };
    }

    execute!(out, LeaveAlternateScreen, Show)?;
    disable_raw_mode()
}
