use std::io::{stdout, Write};

use anyhow::Context;
use crossterm::{cursor, event::read, terminal, ExecutableCommand, QueueableCommand};

use crate::cursor::{Actions, Cursor, Modes};

pub struct Core;
impl Core {
    pub fn run(self) -> anyhow::Result<()> {
        // Cursor position (cx, cy)
        let mut cx = 0;
        let mut cy = 0;
        let mut mode = Modes::Normal;
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
                Cursor::handle_modes(&mut stdout, &mode, read().context("Failed to read event")?)?
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
