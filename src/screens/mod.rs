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

pub trait Screen : WidgetRef {
    type Layout;
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn layout(&mut self) -> &mut Self::Layout;
    fn on_key_event(&mut self, _: KeyEvent) {}
    fn on_tick(&mut self, _: usize) {}
}