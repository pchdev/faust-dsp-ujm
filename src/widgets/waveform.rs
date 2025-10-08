
use std::{
    f64::consts::PI, 
};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, layout::Flex, prelude::Rect, style::Color, symbols, widgets::{
        canvas::{
            Canvas, Line, Points
        }, Widget, WidgetRef
    }
};
use ratatui_macros::vertical;

use crate::widgets::{control::block::ControlBlock, InteractiveWidget};

const RESOLUTION: usize = 400usize;

impl Waveform {
    pub(crate) fn new() -> Self {
        Waveform {
            tick: 0,
            coords: [(0.0, 0.0); RESOLUTION],
            cblock: ControlBlock::default()
                .add_slider("amplitude", 25.0, 0.0..100.0)
                .add_slider("frequency", 50.0, 1.0..100.0)

        }
    }
}

impl InteractiveWidget for Waveform {
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::F(5) => {
                self.cblock.display = !self.cblock.display;  
            }
            KeyCode::Esc => {
                self.cblock.display = false;
            }
            _ => {
                if self.cblock.display {
                    self.cblock.on_key_event(k);
                } 
            }
        }    
    }
    fn on_tick(&mut self, tick: usize) {
        self.tick += 1;
        let amplitude = self.cblock.read_control(0).unwrap() as f64;
        let frequency = self.cblock.read_control(1).unwrap() as f64;
        // Update coordinates:
        for n in 0..self.coords.len() {
            let x = (n as f64 + tick as f64) / frequency as f64;
            let y = (x * PI * 2.0).sin() * amplitude as f64 + 200.0;
            self.coords[n] = (n as f64 + 5.5, y);
        }   
    }
}

#[derive(Debug)]
pub struct Waveform {
    pub tick: usize,
    pub coords: [(f64, f64); RESOLUTION],
    cblock: ControlBlock,
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
        let lv = vertical![==33%, ==67%]
            .flex(Flex::Center)
            .split(area)
        ;
        self.cblock.render(lv[0], buf);
    }
}