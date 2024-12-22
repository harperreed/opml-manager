use crate::tui::TuiApp;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_ui(frame: &mut Frame, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let header = Paragraph::new("OPML Manager")
        .style(Style::default().fg(Color::White).bg(Color::Blue))
        .block(Block::default().borders(Borders::ALL).title("Header"));

    let status = Paragraph::new(format!(
        "Status: {}",
        if app.modified {
            "Modified"
        } else {
            "Unmodified"
        }
    ))
    .style(Style::default().fg(Color::White).bg(Color::Blue))
    .block(Block::default().borders(Borders::ALL).title("Status"));

    let categories: Vec<ListItem> = app
        .categories
        .keys()
        .map(|c| ListItem::new(c.clone()))
        .collect();
    let categories_list = List::new(categories)
        .block(Block::default().borders(Borders::ALL).title("Categories"))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let feeds: Vec<ListItem> = app
        .feeds
        .iter()
        .map(|f| ListItem::new(f.title.clone()))
        .collect();
    let feeds_list = List::new(feeds)
        .block(Block::default().borders(Borders::ALL).title("Feeds"))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_widget(header, chunks[0]);
    frame.render_widget(status, chunks[2]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[1]);

    frame.render_stateful_widget(
        categories_list,
        main_chunks[0],
        &mut app.categories_state.clone(),
    );
    frame.render_stateful_widget(feeds_list, main_chunks[1], &mut app.feeds_state.clone());
}
