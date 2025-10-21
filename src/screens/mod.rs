use crossterm::event::KeyEvent;
use ratatui::{
    widgets::WidgetRef
};

pub mod layouts;
pub mod myself;
pub mod agenda;
pub mod signal;
pub mod sound;
pub mod splash;
pub mod digital;
pub mod faust;

#[macro_export]
macro_rules! leafy {
    ($str:expr) => {
        concat!("**[↲]**  ", indoc!($str))
    };
}

// pub(crate) use leafy;

pub(crate) trait Screen : WidgetRef {
    fn title(&self) -> &'static str;
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