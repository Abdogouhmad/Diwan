use crossterm::{event, style, QueueableCommand};
use std::io;

/// Enum for action cursor
pub enum Actions {
    Quit,
    Up,
    Down,
    Left,
    Right,
    EnterMode(Modes),
}

/// enum for mode
pub enum Modes {
    Normal,
    Insert,
}

pub struct Cursor;

impl Cursor {
    pub fn handle_modes(
        stdout: &mut io::Stdout,
        mode: &Modes,
        ev: event::Event,
    ) -> anyhow::Result<Option<Actions>> {
        match mode {
            Modes::Normal => Cursor::normal_mode(ev),
            Modes::Insert => Cursor::insert_mode(stdout, ev),
        }
    }
    pub fn normal_mode(ev: event::Event) -> anyhow::Result<Option<Actions>> {
        match ev {
            event::Event::Key(ev) => match ev.code {
                event::KeyCode::Char('q') => Ok(Some(Actions::Quit)),
                event::KeyCode::Char('h') | event::KeyCode::Left => Ok(Some(Actions::Left)),
                event::KeyCode::Char('j') | event::KeyCode::Down => Ok(Some(Actions::Down)),
                event::KeyCode::Char('k') | event::KeyCode::Up => Ok(Some(Actions::Up)),
                event::KeyCode::Char('l') | event::KeyCode::Right => Ok(Some(Actions::Right)),
                event::KeyCode::Char('i') => Ok(Some(Actions::EnterMode(Modes::Insert))),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
    pub fn insert_mode(
        stdout: &mut io::Stdout,
        ev: event::Event,
    ) -> anyhow::Result<Option<Actions>> {
        match ev {
            event::Event::Key(ev) => match ev.code {
                event::KeyCode::Esc => Ok(Some(Actions::EnterMode(Modes::Normal))),
                event::KeyCode::Char(c) => {
                    stdout.queue(style::Print(c))?;
                    Ok(None)
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}
