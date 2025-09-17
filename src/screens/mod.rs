use crossterm::event::KeyEvent;
use ratatui::widgets::{StatefulWidgetRef, Widget, WidgetRef};

pub mod myself;
pub mod agenda;
pub mod signal;
pub mod sound;
pub mod splash;

pub(crate) trait Screen : WidgetRef {
    fn on_key_event(&mut self, k: KeyEvent) {}
    fn on_tick(&mut self, t: usize) {}
}

// The ideal would be:
// 

// #[derive(Screen)]
// #[screen(layout = Layout::SideBySide)]

// struct MyScreen {
//     // ---------------------------
//     /// Get **markdown comments**
//     ph1: Paragraph<'_>,
//     // ---------------------------
//     /// - First item
//     /// - Second item
//     /// - etc.
//     ph2: List<'_>
//     // ---------------------------
// }