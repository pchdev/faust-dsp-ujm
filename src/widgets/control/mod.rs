pub mod block;
pub mod button;
pub mod knob;
pub mod meter;
pub mod slider;

use tui_screens::InteractiveWidget;

pub trait ControlWidget : InteractiveWidget {
    fn label(&self) -> String;
    fn get_value(&self) -> f32;
}

