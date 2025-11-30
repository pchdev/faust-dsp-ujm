use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};
use crate::{layouts::Layout};

pub mod layouts;

#[macro_export]
macro_rules! leafy {
    ($str:expr) => {
        concat!("**[↲]**  ", indoc!($str))
    };
}

#[derive(Default)]
pub struct ScreenParagraph;

#[derive(Default)]
pub struct ScreenList;

pub type ScreenHandle = (
    Box<dyn Screen>, 
    Option<Box<dyn Layout>>
);

pub trait Screen {
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn build() -> ScreenHandle where Self: Sized;

    fn render(&self, 
        layout: &Option<Box<dyn Layout>>, 
          area: Rect, 
           buf: &mut Buffer
    ) {
        if let Some(l) = layout {
            l.render_ref(area, buf);
        }
    }
    fn on_key_event(&mut self, 
        layout: &mut Option<Box<dyn Layout>>, 
             k: KeyEvent
    ) {
        if let Some(l) = layout {
            l.on_key_event(k);
        }
    }
    fn on_tick(&mut self, 
        layout: &mut Option<Box<dyn Layout>>, 
             t: usize
    ) {
        if let Some(l) = layout {
            l.on_tick(t);
        }
    }
}

pub trait InteractiveWidget : WidgetRef {
    fn on_key_event(&mut self, k: KeyEvent) {}
    fn on_tick(&mut self, t: usize) {}
}

