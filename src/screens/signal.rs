use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};

use crate::screens::{ContentArea, Screen};


pub struct Signal<'a> {
    ca: ContentArea<'a>,

}


impl<'a> WidgetRef for Signal<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        
    }
}

impl<'a> Screen for Signal<'a> {
    fn on_key_event(&mut self, k: KeyEvent) {
        
    }

    fn on_tick(&mut self, t: usize) {
        
    }
}