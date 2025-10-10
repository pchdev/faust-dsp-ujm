
use std::{
    f64::consts::PI, time::Duration, 
};

use crossterm::event::{KeyCode, KeyEvent};
use rand::Rng;
use ratatui::{
    buffer::Buffer, layout::Flex, prelude::Rect, style::Color, symbols, text::Text, widgets::{
        canvas::{
            Canvas, Label, Line, Points
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

        let incr = 0.0050125;
        let amplitude = self.cblock.read_control(0).unwrap() as f64;
        let sine = self.cblock.read_control(1).unwrap();
        let tri = self.cblock.read_control(2).unwrap();
        let sqr = self.cblock.read_control(3).unwrap();
        let noise = self.cblock.read_control(4).unwrap();
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
        } else if sqr != 0.0 {
            let mut phase = self.phase;
            for n in 0..self.coords.len() {
                let mut y = if phase <= 0.5 {-1.0} else {1.0};
                y = y * amplitude + 200.0;
                self.coords[n] = (n as f64 + x_offset, y);
                phase += incr;
                if phase >= 1.0 {
                   phase -= 1.0;
                }
            }
            self.phase = phase;
        } else if noise != 0.0 {
            // shift all values left:
            for n in 0..self.coords.len()-1 {
                self.coords[n] = (n as f64, self.coords[n+1].1);
            }
            let mut rng = rand::rng();
            let y = rng.random_range(-1.0..1.0);
            let n = self.coords.len()-1;
            let last = self.coords.last_mut().unwrap();
            *last = (n as f64 + x_offset, y * amplitude + 200.0);
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
                    x1: 10.0, y1: 200.0,
                    x2: 390.0, y2: 200.0,
                    color: Color::Gray
                });
                ctx.draw(&Line {
                    x1: 10.0, y1: 300.0,
                    x2: 390.0, y2: 300.0,
                    color: Color::Gray
                });
                ctx.draw(&Line {
                    x1: 10.0, y1: 100.0,
                    x2: 390.0, y2: 100.0,
                    color: Color::Gray
                });
                ctx.draw(&Points {
                    coords: &self.coords,
                    color: Color::Black                    
                });
                ctx.print(375.0, 305.0, ratatui::text::Line::from("+1V"));
                ctx.print(375.0, 95.0, ratatui::text::Line::from("-1V"));
                for n in (0..self.coords.len()).step_by(2) {
                    // If we have too much of a gap between two values in one sample, 
                    // draw the line on the same vertical axis:
                    let x2 = if (self.coords[n].1 - self.coords[n+1].1).abs() >= 0.5 {
                        self.coords[n].0
                    } else {
                        self.coords[n+1].0
                    };
                    ctx.draw(&Line {
                        x1: self.coords[n].0,
                        y1: self.coords[n].1,
                        x2: x2,
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