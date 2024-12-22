use crossterm::event::{self, Event as CEvent, KeyCode};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crate::tui::TuiApp;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    rx: mpsc::Receiver<Event<KeyCode>>,
    _tx: mpsc::Sender<Event<KeyCode>>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).unwrap() {
                    if let CEvent::Key(key) = event::read().unwrap() {
                        event_tx.send(Event::Input(key.code)).unwrap();
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    event_tx.send(Event::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event<KeyCode>, mpsc::RecvError> {
        self.rx.recv()
    }
}

pub fn handle_events(app: &mut TuiApp, event: Event<KeyCode>) {
    match event {
        Event::Input(key) => match key {
            KeyCode::Char('q') => {
                // Handle quit
            }
            KeyCode::Char('s') => {
                // Handle save
                app.save_changes();
            }
            KeyCode::Char('a') => {
                // Handle add category
            }
            KeyCode::Char('d') => {
                // Handle delete category
            }
            KeyCode::Char('r') => {
                // Handle rename category
            }
            KeyCode::Char('m') => {
                // Handle move feed
            }
            KeyCode::Char('g') => {
                // Handle merge categories
            }
            KeyCode::Char('f') => {
                // Handle search/filter
            }
            KeyCode::Char('h') => {
                // Handle help
            }
            KeyCode::Char('k') => {
                // Handle keep duplicate
            }
            KeyCode::Char('x') => {
                // Handle remove duplicate
            }
            _ => {}
        },
        Event::Tick => {
            // Handle tick event
        }
    }
}
