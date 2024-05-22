//! # Keybindings
//! the keybindings intends to handle the most important keys
//! and mode shifting

use crossterm::event;

/// Action enum for keybindings
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
/// modes switcher enum
#[derive(Debug)]
pub enum Modes {
    Normal,
    Insert,
}

/// the cursor impl handle the most important critical events
/// such as cursor movment, key pressed, and mode shifting, and
/// char adding
pub struct Cursor;

/// Cursor struct represents the cursor in the application.
impl Cursor {
    /// Handles mode shifting events.
    ///
    /// This function takes an `event::Event` as input and returns a `Result` containing an
    /// `Option<Actions>`. It is currently unimplemented and will always return an error.
    ///
    /// # Arguments
    ///
    /// * `ev` - The event to handle.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Actions>>` - The result containing an optional action.
    pub fn mode_switcher(ev: event::Event) -> anyhow::Result<Option<Actions>> {
        // match Modes {
        //     Modes::Normal => Cursor::normal_md(ev),
        //     Modes::Insert => Cursor::insert_md(ev),
        // }
        unimplemented!("cool")
    }
    /// Handles events in normal mode.
    ///
    /// This function takes an `event::Event` as input and returns a `Result` containing an
    /// `Option<Actions>`. It is currently unimplemented and will always return an error.
    ///
    /// # Arguments
    ///
    /// * `ev` - The event to handle.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Actions>>` - The result containing an optional action.
    pub fn normal_md(ev: event::Event) -> anyhow::Result<Option<Actions>> {
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
        // unimplemented!("not yet")
    }

    /// Handles events in insert mode.
    ///
    /// This function takes an `event::Event` as input and returns a `Result` containing an
    /// `Option<Actions>`. It is currently unimplemented and will always return an error.
    ///
    /// # Arguments
    ///
    /// * `ev` - The event to handle.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Actions>>` - The result containing an optional action.
    pub fn insert_md(ev: event::Event) -> anyhow::Result<Option<Actions>> {
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
