use anyhow::Ok;
use crossterm::{event, terminal};

use crate::editor::Editor;

/// Enum for action cursor
pub enum Actions {
    AddChar(char),
    NewLine,
    Quit,
    Up,
    Down,
    Left,
    Right,
    EnterMode(Modes),
}

/// enum for mode
#[derive(Debug)]
pub enum Modes {
    Normal,
    Insert,
}

pub struct Cursor;

impl Cursor {
    pub fn handle_modes(edt: &mut Editor, ev: event::Event) -> anyhow::Result<Option<Actions>> {
        if matches!(ev, event::Event::Resize(_, _)) {
            edt.size = terminal::size()?;
            if let event::Event::Resize(width, height) = ev {
                edt.size = (width, height);
                return Ok(None);
            }
        }
        match edt.mode {
            Modes::Normal => Cursor::normal_mode(ev),
            Modes::Insert => Cursor::insert_mode(ev),
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
    pub fn insert_mode(ev: event::Event) -> anyhow::Result<Option<Actions>> {
        let action = match ev {
            event::Event::Key(event) => match event.code {
                event::KeyCode::Esc => Some(Actions::EnterMode(Modes::Normal)),
                event::KeyCode::Enter => Some(Actions::NewLine),
                event::KeyCode::Char(c) => Some(Actions::AddChar(c)),
                _ => None,
            },
            _ => None,
        };

        Ok(action)
    }
}
