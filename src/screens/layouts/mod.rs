use crate::{screens::layouts::{plainfull::PlainFull, sidebyside::SideBySide}, widgets::InteractiveWidget};

pub mod content;
pub mod plainfull;
pub mod sidebyside;


pub enum LayoutEnum {
    None,
    Plainfull,
    SideBySide
}
fn test() {
    match LayoutEnum::None {
        LayoutEnum::None => (),
        _ => ()
    }
}

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
    fn add_widget(mut self, w: Box<dyn InteractiveWidget>) 
    -> Self where Self: Sized {
        self
    }
}