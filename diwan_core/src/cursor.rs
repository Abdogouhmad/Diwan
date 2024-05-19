use anyhow::Context;
use crossterm::{
    cursor,
    event::{self, read},
    style, terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{self, stdout, Write};

/// Enum for action cursor
enum Actions {
    Quit,
    Up,
    Down,
    Left,
    Right,
    EnterMode(Modes),
}

/// enum for mode
enum Modes {
    NORMAL,
    INSERT,
}

pub struct Editor;

impl Editor {
    fn handle_modes(
        stdout: &mut io::Stdout,
        mode: &Modes,
        ev: event::Event,
    ) -> anyhow::Result<Option<Actions>> {
        match mode {
            Modes::NORMAL => Editor::normal_mode(ev),
            Modes::INSERT => Editor::insert_mode(stdout, ev),
        }
    }
    fn normal_mode(ev: event::Event) -> anyhow::Result<Option<Actions>> {
        match ev {
            event::Event::Key(ev) => match ev.code {
                event::KeyCode::Char('q') => Ok(Some(Actions::Quit)),
                event::KeyCode::Char('h') | event::KeyCode::Left => Ok(Some(Actions::Left)),
                event::KeyCode::Char('j') | event::KeyCode::Down => Ok(Some(Actions::Down)),
                event::KeyCode::Char('k') | event::KeyCode::Up => Ok(Some(Actions::Up)),
                event::KeyCode::Char('l') | event::KeyCode::Right => Ok(Some(Actions::Right)),
                event::KeyCode::Char('i') => Ok(Some(Actions::EnterMode(Modes::INSERT))),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
    fn insert_mode(stdout: &mut io::Stdout, ev: event::Event) -> anyhow::Result<Option<Actions>> {
        match ev {
            event::Event::Key(ev) => match ev.code {
                event::KeyCode::Esc => Ok(Some(Actions::EnterMode(Modes::NORMAL))),
                event::KeyCode::Char(c) => {
                    stdout.queue(style::Print(c))?;
                    Ok(None)
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
    pub fn run(self) -> anyhow::Result<()> {
        // Cursor position (cx, cy)
        let mut cx = 0;
        let mut cy = 0;
        let mut mode = Modes::NORMAL;
        // Create a mutable stdout handle to manage terminal output
        let mut stdout = stdout();
        // Enable raw mode
        terminal::enable_raw_mode().context("Failed to enable raw mode")?;
        // Enter the alternate screen
        stdout
            .execute(terminal::EnterAlternateScreen)
            .context("Failed to enter alternate screen")?;
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .context("Failed to clear screen")?;

        // Main loop for handling input
        loop {
            // Move the cursor to the current position (cx, cy) and flush the output
            stdout.queue(cursor::MoveTo(cx, cy))?;
            stdout.flush().context("Failed to flush stdout")?;

            // Read an event from the terminal
            if let Some(action) =
                Editor::handle_modes(&mut stdout, &mode, read().context("Failed to read event")?)?
            {
                match action {
                    Actions::Quit => break,
                    Actions::Up => {
                        cy = cy.saturating_sub(1);
                    }
                    Actions::Down => {
                        cy += 1u16;
                    }
                    Actions::Left => {
                        cx = cx.saturating_sub(1);
                    }
                    Actions::Right => {
                        cx += 1u16;
                    }
                    Actions::EnterMode(new_mode) => mode = new_mode,
                }
            }
        }

        // Leave the alternate screen and disable raw mode
        stdout
            .execute(terminal::LeaveAlternateScreen)
            .context("Failed to leave alternate screen")?;
        terminal::disable_raw_mode().context("Failed to disable raw mode")?;
        Ok(())
    }
}
