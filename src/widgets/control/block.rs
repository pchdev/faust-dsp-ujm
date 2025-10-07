use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, layout::{Constraint, Flex, Layout, Rect}, style::{Style, Stylize}, symbols::border, text::Line, widgets::{Block, BorderType, Clear, Widget, WidgetRef}
};

use crate::widgets::control::button::Button;

#[derive(Default)]
pub(crate) struct ControlBlock {
    controls: Vec<Box<dyn WidgetRef>>,
    pub display: bool,
    sel: usize,
}

impl ControlBlock {
    pub fn add_button(mut self, label: &'static str) -> Self {
        let mut bx = Box::new(Button::default());
        bx.label = String::from(label);
        self.controls.push(bx);
        self
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
            KeyCode::Enter => {
                // Select sub widget
                let ctl = &self.controls[self.sel];
                // ctl.on_key_event()
            }
            _ => {
                
            }
        }
    }
}

impl std::fmt::Debug for ControlBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            .border_type(BorderType::Double)
            .border_style(Style::default().blue())
        ;
        block.render_ref(ly[self.sel], buf);
    }
}

