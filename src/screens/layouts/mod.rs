use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};

use crate::{widgets::InteractiveWidget};


pub mod content;
pub mod plainfull;
pub mod sidebyside;


pub trait Layout : InteractiveWidget {
    fn add_title(mut self, title: &'static str) 
    -> Self where Self: Sized {
        self
    }
    fn add_paragraph(mut self, txt: &'static str) 
    -> Self where Self: Sized {
        self
    }
    fn add_list(mut self, list: Vec<&'static str>) 
    -> Self where Self: Sized {
        self
    }
    fn add_widget(mut self, index: usize, w: Box<dyn InteractiveWidget>) 
    -> Self where Self: Sized {
        self
    }
}