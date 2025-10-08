use crate::widgets::{InteractiveWidget};

pub mod block;
pub mod button;
pub mod knob;
pub mod meter;
pub mod slider;


pub trait ControlWidget : InteractiveWidget {
    fn label(&self) -> String;
    fn get_value(&self) -> f32;
}

