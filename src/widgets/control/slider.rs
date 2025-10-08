
use std::ops::Range;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, 
    layout::Rect, 
    style::{Style, Stylize}, 
    widgets::{Block, BorderType, Gauge, WidgetRef}
};

use crate::widgets::{control::ControlWidget, InteractiveWidget};

#[derive(Debug, Default)]
pub struct Slider {
    pub value: f32,
    pub range: Range<f32>,
    pub label: String,
}

impl Slider {
    pub fn new(label: &str, value: f32, range: Range<f32>) -> Self {
        Slider {
            value, range, label: String::from(label)
        }
    }
}

impl ControlWidget for Slider {
    fn label(&self) -> String {
        self.label.clone()
    }
    fn get_value(&self) -> f32 {
        self.value
    }
}

impl WidgetRef for Slider {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Gauge::default()
            .use_unicode(true)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(format!(" {} ", self.label))
            )
            .gauge_style(
                Style::default().black().on_white()
            )
            .percent(self.value as u16)
            .render_ref(area, buf);
    }
}

impl InteractiveWidget for Slider {
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Up => {
                self.value += 1.0;
                if self.value > 100.0 {
                   self.value = 100.0
                }
            }
            KeyCode::Down => {
                self.value -= 1.0;
                if self.value < 0.0 {
                   self.value = 0.0;
                }
            }
            _ => ()
        }
    }
}






