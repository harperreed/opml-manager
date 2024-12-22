use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::tui::TuiApp;

pub struct FeedList {
    pub feeds: Vec<Feed>,
    pub selected_feed: Option<usize>,
}

impl FeedList {
    pub fn new(feeds: Vec<Feed>) -> Self {
        FeedList {
            feeds,
            selected_feed: None,
        }
    }

    pub fn select_feed(&mut self, index: Option<usize>) {
        self.selected_feed = index;
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: ratatui::layout::Rect) {
        let feeds: Vec<ListItem> = self
            .feeds
            .iter()
            .map(|f| {
                let mut item = ListItem::new(f.title.clone());
                if f.is_duplicate {
                    item = item.style(Style::default().fg(Color::Red));
                }
                item
            })
            .collect();

        let feeds_list = List::new(feeds)
            .block(Block::default().borders(Borders::ALL).title("Feeds"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        f.render_stateful_widget(feeds_list, area, &mut self.selected_feed);
    }
}
