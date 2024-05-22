use anyhow::Context;
use crossterm::{
    cursor,
    event::read,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Write};

use crate::{
    buffer::Buffer,
    cursor::{Actions, Cursor, Modes},
};

pub struct Editor {
    buffer: Buffer,
    stdout: std::io::Stdout,
    pub size: (u16, u16),
    cx: u16,
    cy: u16,
    pub mode: Modes,
}

impl Drop for Editor {
    fn drop(&mut self) {
        // Restore the terminal state
        _ = self.stdout.execute(terminal::LeaveAlternateScreen);
        _ = terminal::disable_raw_mode();
        _ = self.stdout.flush();
    }
}

impl Editor {
    /// function that return the editor instance
    pub fn new(buffer: Buffer) -> anyhow::Result<Self> {
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
            buffer,
            stdout,
            cx: 0,
            cy: 0,
            mode: Modes::Normal,
            size: terminal::size()?,
        })
    }

    /// draw the buffer content and cursor
    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.draw_buffer()?;
        self.draw_statusline()?;
        self.stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
        self.stdout.queue(cursor::Show)?;
        self.stdout.flush()?;

        Ok(())
    }
    /// draw the buffer content
    pub fn draw_buffer(&mut self) -> anyhow::Result<()> {
        self.stdout.queue(cursor::Hide)?;
        self.stdout.queue(cursor::MoveTo(0, 0))?;

        for (i, line) in self.buffer.lines.iter().enumerate() {
            if i as u16 >= self.size.1 - 1 {
                break;
            }
            self.stdout.queue(cursor::MoveTo(0, i as u16))?;
            self.stdout.queue(style::Print(line))?;
        }
        Ok(())
    }
    /// status line
    pub fn draw_statusline(&mut self) -> anyhow::Result<()> {
        let file = self.buffer.file.as_deref().unwrap_or(" [No Name] ");
        let pos = format!(" {}:{} ", self.cy + 1, self.cx + 1);
        let status_line = format!(" {:?} ", self.mode).to_uppercase();
        let file_width = self.size.0 - status_line.len() as u16 - pos.len() as u16 - 2;

        self.stdout.queue(cursor::MoveTo(0, self.size.1 - 2))?;
        // style the mode
        let styled_mode = status_line
            .with(style::Color::Rgb { r: 0, g: 0, b: 0 })
            .on(style::Color::Rgb {
                r: 38,
                g: 139,
                b: 210,
            })
            .bold()
            .white();
        // queue the mode in the status bar
        self.stdout.queue(style::PrintStyledContent(styled_mode))?;
        // the file edited
        self.stdout.queue(style::PrintStyledContent(
            format!(" {:<width$}", file, width = file_width as usize)
                .with(style::Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                })
                .on(style::Color::Rgb {
                    r: 203,
                    g: 75,
                    b: 22,
                })
                .white()
                .bold(),
        ))?;
        // position
        _ = self.stdout.queue(style::PrintStyledContent(
            pos.with(style::Color::Rgb { r: 0, g: 0, b: 0 })
                .bold()
                .white()
                .on(style::Color::Rgb {
                    r: 133,
                    g: 87,
                    b: 35,
                }),
        ));
        Ok(())
    }

    /// fn that run the editor
    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.draw()?;

            if let Some(action) = Cursor::handle_modes(self, read()?)? {
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

        Ok(())
    }
}
