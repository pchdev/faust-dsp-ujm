use crossterm::event::KeyEvent;
use ratatui::widgets::WidgetRef;


pub mod agenda;
pub mod signal;
pub mod sound;
pub mod splash;
pub mod toc;

pub(crate) trait Screen : WidgetRef {
    fn on_key_event(&mut self, k: KeyEvent);
}