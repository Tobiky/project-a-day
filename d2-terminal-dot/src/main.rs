use std::io::Write;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

const CURSOR: char = '▪';

fn main() -> std::io::Result<()> {
    let mut out = std::io::stdout();
    enable_raw_mode()?;
    let (mut cols, mut rows) = size()?;
    let (mut x, mut y) = (0, 0);

    queue!(
        out,
        EnterAlternateScreen,
        Hide,
        Clear(ClearType::All),
        MoveTo(x, y),
        Print(CURSOR)
    )?;
    out.flush()?;
    loop {
        use crossterm::event::{
            Event::{Key, Resize},
            KeyCode::{Char, Down, Esc, Left, Right, Up},
            KeyEvent, KeyModifiers,
        };

        match crossterm::event::read()? {
            Key(
                KeyEvent {
                    code: Char('d') | Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }
                | KeyEvent { code: Esc, .. },
            ) => break,
            Key(KeyEvent {
                code: Up | Char('w') | Char('k'),
                ..
            }) => y = y.wrapping_sub(1).rem_euclid(rows),
            Key(KeyEvent {
                code: Down | Char('s') | Char('j'),
                ..
            }) => y = y.wrapping_add(1).rem_euclid(rows),
            Key(KeyEvent {
                code: Left | Char('a') | Char('h'),
                ..
            }) => x = x.wrapping_sub(1).rem_euclid(cols),
            Key(KeyEvent {
                code: Right | Char('d') | Char('l'),
                ..
            }) => x = x.wrapping_add(1).rem_euclid(cols),
            Resize(new_cols, new_rows) => (cols, rows) = (new_cols, new_rows),
            _ => (),
        };

        execute!(out, Print('A'))?;

        queue!(out, Clear(ClearType::All), MoveTo(x, y), Print(CURSOR))?;
        out.flush()?;
    }
    queue!(out, Clear(ClearType::All), Show, LeaveAlternateScreen)?;
    out.flush()?;
    disable_raw_mode()
}
