use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer, layout::Rect, widgets::WidgetRef
};

use crate::{screens::layouts::Layout, widgets::InteractiveWidget};

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
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn layout(&self) -> Option<&dyn Layout>;
    fn layout_mut(&mut self) -> Option<&mut dyn Layout>;

    fn on_key_event(&mut self, k: KeyEvent) {
        if let Some(l) = self.layout_mut() {
            l.on_key_event(k);
        }
    }
    fn on_tick(&mut self, t: usize) {
        if let Some(l) = self.layout_mut() {
            l.on_tick(t);
        }
    }
}

