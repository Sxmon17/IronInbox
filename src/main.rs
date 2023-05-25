use std::io::{self, stdout, Write};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Modifier, Style};
use tui::text::{Spans};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let stdout = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(30),
                    Constraint::Percentage(50),
                ].as_ref()
            )
            .split(f.size());

        let mailboxes = vec!["Inbox", "Sent", "Drafts"];
        let mailbox_items: Vec<ListItem> = mailboxes
            .iter()
            .map(|m| ListItem::new(Spans::from(*m)))
            .collect();
        let mailbox_list = List::new(mailbox_items)
            .block(Block::default().borders(Borders::ALL).title("Mailboxes"))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
        f.render_widget(mailbox_list, chunks[0]);

        let emails = vec!["Email 1", "Email 2", "Email 3"];
        let email_items: Vec<ListItem> = emails
            .iter()
            .map(|e| ListItem::new(Spans::from(*e)))
            .collect();
        let email_list = List::new(email_items)
            .block(Block::default().borders(Borders::ALL).title("Email List"))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
        f.render_widget(email_list, chunks[1]);

        let email_content = Paragraph::new("This is the content of the selected email.")
            .block(Block::default().borders(Borders::ALL).title("Email Content"));
        f.render_widget(email_content, chunks[2]);
    })?;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => {}
        }
    }

    Ok(())
}
