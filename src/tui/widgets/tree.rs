use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};
use std::collections::HashMap;

pub struct CategoryTree {
    pub categories: HashMap<String, Vec<String>>,
    pub state: ListState,
}

impl CategoryTree {
    pub fn new(categories: HashMap<String, Vec<String>>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        CategoryTree { categories, state }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.categories.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.categories.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        if self.categories.is_empty() {
            let empty_message = ListItem::new("No categories available")
                .style(Style::default().fg(Color::Gray));
            let list = List::new(vec![empty_message])
                .block(Block::default().borders(Borders::ALL).title("Categories"));
            frame.render_widget(list, area);
            return;
        }
        let items: Vec<ListItem> = self
            .categories
            .keys()
            .map(|c| {
                ListItem::new(c.as_str())
                    .style(Style::default().fg(Color::White))
            })
            .collect();
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Categories"))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, area, &mut self.state);
    }
}
