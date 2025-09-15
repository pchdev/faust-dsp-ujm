
use std::{f64::consts::PI, thread, time::Duration};

use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer, prelude::Rect, style::{Color, Style}, symbols, widgets::{
        canvas::{
            Canvas, 
            Circle, Points
        }, StatefulWidgetRef, WidgetRef
    }
};


impl Waveform {
    pub(crate) fn on_tick(&mut self, tick: usize) {
        // TODO: frequency
        if tick % 2 == 0 {
            self.tick += 1;
        }
        if self.tick >= self.amplitude {
           self.tick -= self.amplitude;
        }
    }
}


#[derive(Debug, Default)]
pub struct Waveform {
    pub tick: usize,
    pub frequency: usize,
    pub amplitude: usize
}

impl WidgetRef for Waveform {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let coords = [(200f64, 200f64); 500];
        Canvas::default()
            .marker(symbols::Marker::Braille)
            .background_color(Color::White)
            .paint(|ctx| {
                ctx.draw(&Points {
                    coords: &coords,
                    color: Color::Black                    
                });
            })            
            .x_bounds([00.0, 400.0 as f64])
            .y_bounds([00.0, 400.0 as f64])
            .render_ref(area, buf)
        ;        
    }
}