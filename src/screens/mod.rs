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

pub trait Screen {
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

// impl InteractiveWidget for dyn Screen {
//     fn on_key_event(&mut self, k: KeyEvent) {}
//     fn on_tick(&mut self, t: usize) {}
// }

impl<T> WidgetRef for T 
where 
    T: Screen 
{
    fn render_ref(&self,area:Rect,buf: &mut Buffer) {
        if let Some(l) = self.layout() {
            l.render_ref(area, buf);
        } 
    }
}

// impl WidgetRef for dyn Screen {
//     fn render_ref(&self, area: Rect, buf: &mut Buffer) {
//         if let Some(l) = self.layout() {
//             l.render_ref(area, buf);
//         } 
//     }
// }

