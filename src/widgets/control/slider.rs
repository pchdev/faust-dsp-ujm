
use std::ops::Range;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, BorderType, Gauge, WidgetRef}};

use crate::widgets::InteractiveWidget;


#[derive(Debug, Default)]
pub struct Slider {
    value: usize,
    range: Range<usize>,
}


impl WidgetRef for Slider {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Gauge::default()
            .use_unicode(true)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(" amplitude ")
            )
            .percent(self.value as u16)
            .render_ref(area, buf);
    }
}

impl InteractiveWidget for Slider {
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Up => {
                self.value += 1;
            }
            KeyCode::Down => {
                self.value -= 1;
            }
            _ => ()
        }
    }
}






