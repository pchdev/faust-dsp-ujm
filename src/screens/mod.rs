use crossterm::event::KeyEvent;
use ratatui::widgets::WidgetRef;


pub mod calendar;
pub mod signal;
pub mod sound;
pub mod title;
pub mod toc;

pub(crate) trait Screen : WidgetRef {
    fn on_key_event(&mut self, k: KeyEvent);
}