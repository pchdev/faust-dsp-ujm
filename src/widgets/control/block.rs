use std::ops::Range;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, 
    layout::{Constraint, Flex, Layout, Rect}, 
    style::{Style, Stylize}, 
    symbols::border, 
    text::Line, 
    widgets::{Block, BorderType, Clear, Padding, Widget, WidgetRef}
};

use crate::widgets::{control::{button::Button, slider::Slider, ControlWidget}};

#[derive(Default)]
pub(crate) struct ControlBlock {
    controls: Vec<Box<dyn ControlWidget>>,
    pub display: bool,
    sel: usize,
}

impl ControlBlock {
    pub fn add_button(mut self, label: &'static str) -> Self {
        let mut bx = Box::new(Button::default());
        bx.label = String::from(label);
        self.controls.push(bx);
        return self;
    }
    pub fn add_slider(mut self, 
        label: &'static str, 
        value: f32, 
        range: Range<f32>
    ) -> Self {
        self.controls.push(Box::new(
            Slider::new(label, value, range)
        ));
        return self;
    }

    pub fn read_control(&self, index: usize) -> Option<f32> {
        match self.controls.get(index) {
            Some(ctl) => {
                Some(ctl.get_value())
            }
            None => None
        }
    }

    pub fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Right => {
                self.sel += 1;
                if self.sel >= self.controls.len() {
                   self.sel = self.controls.len()-1;
                }
            }
            KeyCode::Left => {
                if self.sel > 0 {
                   self.sel -= 1;
                }
            }
            _ => {
                // Select sub widget
                let ctl = &mut self.controls[self.sel];
                ctl.on_key_event(k);
            }
        }
    }
}

impl std::fmt::Debug for ControlBlock {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl WidgetRef for ControlBlock {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        if !self.display {
            return;
        }
        let block = Block::bordered()
            .title(Line::from(" controls ").right_aligned().bold())
            .border_set(border::ROUNDED)
            .style(Style::default().black().on_white())
        ;   
        // Compute the constraints:
        let div = 100.0 / self.controls.len() as f64;
        let constraints: Vec<Constraint> = (0..self.controls.len())
            .map(|_| Constraint::Percentage(div as u16))
            .collect()
        ;
        // Build the layout:
        let ly = Layout::horizontal(constraints)
            .flex(Flex::Center)
            .split(block.inner(area))
        ;
        Clear::default().render(area, buf);
        block.render_ref(area, buf);
        // Render every widget in their respective layout split:
        for (n, w) in self.controls.iter().enumerate() {
            w.render_ref(ly[n], buf);
        }
        // Add border to selected widget:
        let block = Block::bordered()
            .padding(Padding::vertical(10))
            .border_type(BorderType::Plain)
            .border_style(Style::default().blue())
        ;
        block.render_ref(ly[self.sel], buf);
    }
}

