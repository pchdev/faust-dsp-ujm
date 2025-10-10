
use std::{
    f64::consts::PI, time::Duration, 
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

#[derive(Debug)]
pub struct Waveform {
    pub phase: f32,
    pub coords: [(f64, f64); RESOLUTION],
    cblock: ControlBlock,
}

impl Waveform {
    pub(crate) fn new() -> Self {
        Waveform {
            phase: 0f32,
            coords: [(0.0, 0.0); RESOLUTION],
            cblock: ControlBlock::default()
                .add_slider("amplitude", 25.0, 0.0..100.0)
                .add_button("sine")
                .add_button("triangle")
                .add_button("square")
                .add_button("noise")

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
    fn on_tick(&mut self, _: usize) {
        // One tick is 5 millis,
        // meaning we have 200 ticks per second:
        let period = Duration::from_millis(5).as_secs_f32();

        let incr = 0.005025;
        let amplitude = self.cblock.read_control(0).unwrap() as f64;
        let sine = self.cblock.read_control(1).unwrap();
        let tri = self.cblock.read_control(2).unwrap();
        let x_offset = 5.5 as f64;

        if sine != 0.0 {
            // Update coordinates:
            let mut phase = self.phase;
            for n in 0..self.coords.len() {
                let y = (phase as f64 * PI * 2.0).sin() * amplitude as f64 + 200.0;
                self.coords[n] = (n as f64 + x_offset, y);
                phase += incr;
                if phase >= 1.0 {
                   phase -= 1.0;
                }
            }   
            self.phase = phase;
        } else if tri != 0.0 {
            let mut phase = self.phase;
            for n in 0..self.coords.len() {
                let t = if phase <= 0.5 {
                    phase
                } else {
                    1.0 - phase
                };
                let y = ((t as f64 - 0.25) * 4.0) * amplitude as f64 + 200.0;
                self.coords[n] = (n as f64 + x_offset, y);
                phase += incr;
                if phase >= 1.0 {
                   phase -= 1.0;
                }
            }
            self.phase = phase;
        }

    }
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
                for n in (0..self.coords.len()).step_by(2) {
                    ctx.draw(&Line {
                        x1: self.coords[n].0,
                        y1: self.coords[n].1,
                        x2: self.coords[n+1].0,
                        y2: self.coords[n+1].1,
                        color: Color::Black
                    });
                }
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