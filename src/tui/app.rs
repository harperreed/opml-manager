use crate::Feed;
use ratatui::widgets::ListState;
use std::collections::HashMap;

pub struct TuiApp {
    pub feeds: Vec<Feed>,
    pub categories: HashMap<String, Vec<Feed>>,
    pub categories_state: ListState,
    pub feeds_state: ListState,
    pub selected_category: Option<String>,
    pub selected_feed: Option<usize>,
    pub mode: AppMode,
    pub modified: bool,
}

pub enum AppMode {
    Normal,
    Category,
    Feed,
    Dedup,
}

impl TuiApp {
    pub fn new() -> Self {
        let mut categories_state = ListState::default();
        categories_state.select(Some(0));
        let mut feeds_state = ListState::default();
        feeds_state.select(Some(0));

        TuiApp {
            feeds: Vec::new(),
            categories: HashMap::new(),
            categories_state,
            feeds_state,
            selected_category: None,
            selected_feed: None,
            mode: AppMode::Normal,
            modified: false,
        }
    }

    pub fn load_feeds(&mut self, feeds: Vec<Feed>) {
        self.feeds = feeds;
        self.categorize_feeds();
    }

    fn categorize_feeds(&mut self) {
        self.categories.clear();
        for feed in &self.feeds {
            for category in &feed.category {
                self.categories
                    .entry(category.clone())
                    .or_default()
                    .push(feed.clone());
            }
        }
    }

    pub fn select_category(&mut self, category: Option<String>) {
        self.selected_category = category;
        self.selected_feed = None;
    }

    pub fn select_feed(&mut self, index: Option<usize>) {
        self.selected_feed = index;
    }

    pub fn set_mode(&mut self, mode: AppMode) {
        self.mode = mode;
    }

    pub fn mark_modified(&mut self) {
        self.modified = true;
    }

    pub fn save_changes(&mut self) {
        self.modified = false;
    }
}
