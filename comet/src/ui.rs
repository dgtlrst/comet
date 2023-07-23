/* 
    ui.rs

    terminal user interface
*/

use crossterm::{
    execute,
    terminal::{self, ClearType, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand, event::{EnableMouseCapture, DisableMouseCapture, read, KeyCode, Event, KeyEvent, KeyModifiers, self}
};
use std::{io::{self, Write, Read}};
use tui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders, Paragraph}, text::Text, style::{Style, Color}, layout::{Direction, Constraint, Layout}};

pub fn run_tui() -> Result<(), io::Error> {

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a text widget for "Hello, TUI!"
    let text = Text::raw("Press Ctrl+q to exit.");

    // Clear the terminal and draw the TUI
    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("TUI Example")
                .border_style(Style::default().fg(Color::Yellow));

            f.render_widget(block, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let paragraph = Paragraph::new(text.clone())
                .style(Style::default().fg(Color::White))
                .block(Block::default());
            f.render_widget(paragraph, chunks[0]);
        })?;

        // Handle user input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::CONTROL,
                        kind: _,
                        state: _,
                    } => break, // Exit when the user presses Ctrl + q
                    _ => continue,
                }
            }
        }
    }

    // Restore terminal settings and exit
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
