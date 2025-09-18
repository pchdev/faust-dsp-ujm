
use std::{
    f64::consts::PI, 
    time::Duration
};

use rand::Rng;
use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    style::{Color}, 
    symbols, 
    widgets::{
        canvas::{
            Canvas, Line, Points
        }, 
        WidgetRef
    }
};

const RESOLUTION: usize = 400usize;

impl Waveform {
    pub(crate) fn new(frequency: usize, amplitude: usize) -> Self {
        Waveform {
            tick: 0,
            frequency,
            amplitude,
            coords: [(0.0, 0.0); RESOLUTION]

        }
    }
    pub(crate) fn on_tick(&mut self, tick: usize) {
        self.tick += 1;
        // Update coordinates:
        for n in 0..self.coords.len() {
            let x = (n as f64 + tick as f64) / self.frequency as f64;
            let y = (x * PI * 2.0).sin() * self.amplitude as f64 + 200.0;
            self.coords[n] = (n as f64 + 5.5, y);
        }
    }
}

#[derive(Debug)]
pub struct Waveform {
    pub tick: usize,
    pub frequency: usize,
    pub amplitude: usize, 
    pub coords: [(f64, f64); RESOLUTION],
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