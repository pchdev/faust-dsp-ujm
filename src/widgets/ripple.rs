
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
        let ncircles = 2;
        Canvas::default()
            .marker(symbols::Marker::Braille)
            .background_color(Color::White)
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: self.amplitude as f64,
                    y: self.amplitude as f64,
                    radius: 4.0,
                    color: Color::Black
                });
                for n in 0..ncircles {
                    let pos = n*25;
                    let delayed = if self.tick < pos { 0 } else { 
                        self.tick - pos
                    };
                    let c = delayed as u8;
                    let color = Color::Rgb(c, c, c);
                    ctx.draw(&Circle {
                        x: self.amplitude as f64,
                        y: self.amplitude as f64,
                        radius: delayed as f64,
                        color: color
                    });
                }
                // TODO: other ripple circles
                // + sound?
            })            
            .x_bounds([00.0, 400.0 as f64])
            .y_bounds([00.0, 400.0 as f64])
            .render_ref(area, buf)
        ;        
    }
}