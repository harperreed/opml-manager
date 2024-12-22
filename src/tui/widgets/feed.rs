use crate::Feed;
use ratatui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub struct FeedList {
    pub feeds: Vec<Feed>,
    pub selected_feed: ListState,
}

impl FeedList {
    pub fn new(feeds: Vec<Feed>) -> Self {
        let mut selected_feed = ListState::default();
        selected_feed.select(Some(0));
        FeedList {
            feeds,
            selected_feed,
        }
    }

    pub fn select_next(&mut self) {
        let i = match self.selected_feed.selected() {
            Some(i) => {
                if i >= self.feeds.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected_feed.select(Some(i));
    }

    pub fn select_previous(&mut self) {
        let i = match self.selected_feed.selected() {
            Some(i) => {
                if i == 0 {
                    self.feeds.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected_feed.select(Some(i));
    }

    pub fn draw<B: Backend>(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let feeds: Vec<ListItem> = self
            .feeds
            .iter()
            .map(|f| {
                let mut item = ListItem::new(f.title.clone());
                if !f.category.is_empty() {
                    item = item.style(Style::default().fg(Color::Blue));
                }
                item
            })
            .collect();

        let feeds_list = List::new(feeds)
            .block(Block::default().borders(Borders::ALL).title("Feeds"))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(feeds_list, area, &mut self.selected_feed.clone());
    }
}
