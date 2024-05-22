use anyhow::Context;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    style::Print,
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();

    // Enable raw mode to capture input directly without buffering.
    terminal::enable_raw_mode().context("can't enable raw mode")?;

    // Enter the alternate screen.
    stdout
        .queue(terminal::EnterAlternateScreen)
        .context("can't enter the alternate screen")?
        .queue(terminal::Clear(terminal::ClearType::All))
        .context("can't clear the screen")?
        .flush()
        .context("can't flush stdout")?;

    // Run the terminal event loop.
    loop {
        // Read the next event.
        match read().context("failed to read event")? {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(c) => {
                    if c == 'q' {
                        break;
                    }
                    // Handle other character key presses here
                    stdout.queue(Print(c))?;
                    stdout.flush()?;
                }
                _ => {}
            },
            _ => {}
        }
    }

    // Leave the alternate screen and disable raw mode.
    stdout
        .execute(terminal::LeaveAlternateScreen)
        .context("can't leave the alternate screen")?;
    terminal::disable_raw_mode().context("can't disable raw mode")?;
    stdout.flush().context("can't flush stdout")?;
    Ok(())
}
