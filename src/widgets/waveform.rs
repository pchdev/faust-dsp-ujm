
use std::{
    f64::consts::PI, 
    thread, 
    time::Duration
};

use crossterm::event::KeyEvent;

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    style::{
        Color, 
        Style
    }, 
    symbols, 
    widgets::{
        canvas::{
            Canvas, Circle, Line, Points
        }, 
        StatefulWidgetRef, 
        WidgetRef
    }
};

impl Waveform {
    pub(crate) fn on_tick(&mut self, tick: usize) {
        self.tick += 1;
        // Update coordinates:
        for n in 0..self.coords.len() {
            let x = (n as f64 + tick as f64) / 100.0;
            let y = (x * PI * 2.0).sin() * 25.0 + 200.0;
            self.coords[n] = (n as f64 + 5.5, y);
        }
    }
}

#[derive(Debug)]
pub struct Waveform {
    pub tick: usize,
    pub frequency: usize,
    pub amplitude: usize, 
    pub coords: [(f64, f64); 400],
}

impl WidgetRef for Waveform {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Canvas::default()
            .marker(symbols::Marker::Braille)
            .background_color(Color::White)
            .paint(|ctx| {
                ctx.draw(&Line {
                    x1: 5.5, y1: 200.0,
                    x2: 400.0, y2: 200.0,
                    color: Color::Gray
                });
                ctx.draw(&Points {
                    coords: &self.coords,
                    color: Color::Black                    
                });
            })            
            .x_bounds([00.0, 400.0 as f64])
            .y_bounds([00.0, 400.0 as f64])
            .render_ref(area, buf)
        ;        
    }
}