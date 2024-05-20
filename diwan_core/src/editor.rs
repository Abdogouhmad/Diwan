use anyhow::Context;
use crossterm::{
    cursor,
    event::read,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Write};

use crate::cursor::{Actions, Cursor, Modes};
#[derive(Debug)]
pub struct Editor {
    stdout: std::io::Stdout,
    pub size: (u16, u16),
    cx: u16,
    cy: u16,
    pub mode: Modes,
}

impl Drop for Editor {
    fn drop(&mut self) {
        _ = self.stdout.flush();
        _ = self.stdout.execute(terminal::LeaveAlternateScreen);
        _ = terminal::disable_raw_mode();
    }
}

impl Editor {
    /// function that return the editor instance
    pub fn new() -> anyhow::Result<Self> {
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

        Ok(Editor {
            stdout,
            cx: 0,
            cy: 0,
            mode: Modes::Normal,
            size: terminal::size()?,
        })
    }

    /// draw the cursor
    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.draw_statusline()?;
        self.stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
        self.stdout.flush()?;

        Ok(())
    }

    /// status line
    pub fn draw_statusline(&mut self) -> anyhow::Result<()> {
        let file = " src/main.rs";
        let pos = format!(" {}:{} ", self.cx, self.cy);
        let status_line = format!(" {:?} ", self.mode).to_uppercase();
        let file_width = self.size.0 - status_line.len() as u16 - pos.len() as u16 - 2;

        self.stdout.queue(cursor::MoveTo(0, self.size.1 - 2))?;
        let on_normal = status_line
            .clone()
            .with(style::Color::Rgb { r: 0, g: 0, b: 0 })
            .on(style::Color::Rgb {
                r: 251,
                g: 187,
                b: 118,
            });
        let on_insert = status_line
            .clone()
            .with(style::Color::Rgb { r: 0, g: 0, b: 0 })
            .on(style::Color::Rgb {
                r: 248,
                g: 148,
                b: 63,
            });
        // the mode
        match self.mode {
            Modes::Normal => {
                self.stdout.queue(style::PrintStyledContent(on_normal))?;
            }
            Modes::Insert => {
                self.stdout.queue(style::PrintStyledContent(on_insert))?;
            }
        }
        // the file edited
        self.stdout.queue(style::PrintStyledContent(
            format!("{:<width$}", file, width = file_width as usize)
                .with(style::Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                })
                .bold()
                .on(style::Color::Rgb {
                    r: 67,
                    g: 70,
                    b: 89,
                }),
        ))?;
        // position
        _ = self.stdout.queue(style::PrintStyledContent(
            pos.with(style::Color::Rgb { r: 0, g: 0, b: 0 })
                .bold()
                .on(style::Color::Rgb {
                    r: 184,
                    g: 144,
                    b: 243,
                }),
        ));
        Ok(())
    }

    /// fn that run the editor
    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.draw()?;

            if let Some(action) = Cursor::handle_modes(self, read()?)? {
                //println!("Action received: {:?}", action);
                match action {
                    Actions::Quit => break,
                    Actions::Up => {
                        self.cy = self.cy.saturating_sub(1);
                    }
                    Actions::Down => {
                        self.cy += 1;
                    }
                    Actions::Left => {
                        self.cx = self.cx.saturating_sub(1);
                    }
                    Actions::Right => {
                        self.cx += 1;
                    }
                    Actions::EnterMode(new_mode) => {
                        self.mode = new_mode;
                    }
                    Actions::AddChar(c) => {
                        self.stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
                        self.stdout.queue(style::Print(c))?;
                        self.cx += 1;
                    }
                    Actions::NewLine => {
                        self.cx = 0;
                        self.cy += 1;
                    }
                }
            }
        }
        // Leave the alternate screen and disable raw mode
        Ok(())
    }
}
