use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};
use crate::{screens::layouts::Layout};

pub mod layouts;
// pub mod myself;
// pub mod agenda;
// pub mod signal;
pub mod sound;
// pub mod splash;
// pub mod digital;
// pub mod faust;

#[macro_export]
macro_rules! leafy {
    ($str:expr) => {
        concat!("**[↲]**  ", indoc!($str))
    };
}


#[derive(Default)]
struct ScreenParagraph;

#[derive(Default)]
struct ScreenList;

pub trait Screen {
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn build() -> (Box<dyn Screen>, Option<Box<dyn Layout>>) where Self: Sized;

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

