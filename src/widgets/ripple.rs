
use std::{thread, time::Duration};

use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer, prelude::Rect, style::{Color, Style}, symbols, widgets::{
        canvas::{
            Canvas, 
            Circle
        }, StatefulWidgetRef, WidgetRef
    }
};

#[derive(Debug, Default)]
pub struct Ripple {
    pub tick: usize,
    pub frequency: usize,
    pub amplitude: usize
}

impl Ripple {
    pub(crate) fn on_tick(&mut self, tick: usize) {
        // TODO: frequency
        self.tick += 1;
        if self.tick >= self.amplitude {
           self.tick -= self.amplitude;
        }
    }
}

impl StatefulWidgetRef for Ripple {
    type State = bool;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, running: &mut bool) {
        Canvas::default()
            .marker(symbols::Marker::Braille)
            .background_color(Color::White)
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 250.0,
                    y: 250.0,
                    radius: 4.0,
                    color: Color::Black
                });
                ctx.draw(&Circle {
                    x: 250.0,
                    y: 250.0,
                    radius: self.tick as f64,
                    color: Color::Black
                });
                // TODO: other ripple circles
                // + fade out as reaching max amplitude
                // + sound?
            })            
            .x_bounds([00.0, 500.0 as f64])
            .y_bounds([00.0, 500.0 as f64])
            .render_ref(area, buf)
        ;        
    }
}