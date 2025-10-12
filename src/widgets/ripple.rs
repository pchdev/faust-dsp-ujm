use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    buffer::Buffer, 
    layout::Flex, 
    prelude::Rect, 
    style::{Color}, 
    symbols, 
    widgets::{
        canvas::{
            Canvas, 
            Circle
        }, 
        WidgetRef
    }
};

use ratatui_macros::vertical;
use crate::widgets::{
    control::block::ControlBlock, 
    InteractiveWidget
};
// DFDDC8
#[derive(Debug, Default)]
pub struct Ripple {
    amplitude: f32,
    distance: f32,
    phase: Vec<f32>,
    cblock: ControlBlock
}

impl Ripple {
    pub(crate) fn new() -> Self {
        let mut r = Ripple {
            amplitude: 1.0,
            distance: 0.25,
            phase: vec![0f32; 4],
            cblock: 
                ControlBlock::default()
                    .add_slider("amplitude", 100.0, 0.0..100.0)
                    // .add_slider("frequency", 25.0, 1.0..100.0)
        };
        for (n, ph) in r.phase.iter_mut().enumerate() {
            // add the negative (delayed) phase offsets:
            *ph -= n as f32 * 0.25;
        }
        return r;
    }
}

impl InteractiveWidget for Ripple {
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
                   // Update values immediately:
                   let reg = self.amplitude;
                   self.amplitude = self.cblock
                       .read_control(0)
                       .unwrap()
                       / 100.0;
                    // if amplitude changes, we need to re-compute
                    // the number of simultaneous waves:
                    if reg != self.amplitude {
                        self.phase.resize(
                            (self.amplitude/self.distance) as usize, 
                            0.0
                        );
                    }
                   for (n, ph) in self.phase.iter_mut().enumerate() {
                        // add the negative (delayed) phase offsets:
                        *ph -= n as f32 * 0.25;
                    }
                } 
            }
        }        
    }
    fn on_tick(&mut self, _: usize) {
        let len = Duration::from_millis(5).as_secs_f32() * 0.9;
        let incr = len/(self.amplitude*2.0);
        for ph in &mut self.phase {
            *ph += incr;
            if *ph >= 1.0 {
               *ph -= 1.0;
            }
        }
    }
}

impl WidgetRef for Ripple {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        // Maximum number of circles at the same time:
        let max_amplitude = self.amplitude * 180.0;

        Canvas::default()
            .marker(symbols::Marker::Braille)
            .background_color(Color::White)
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 200.0,
                    y: 200.0,
                    radius: 4.0,
                    color: Color::Black
                });
                for ph in &self.phase {
                    let c = (ph*240.0) as u8;
                    let clr = Color::Rgb(c, c, c);
                    let r = ph * max_amplitude;
                    if r > 0.0 {
                        ctx.draw(&Circle {
                            x: 200.0,
                            y: 200.0,
                            radius: r as f64,
                            color: clr
                        });
                    }
                }
            })            
            .x_bounds([00.0, 400.0 as f64])
            .y_bounds([00.0, 400.0 as f64])
            .render_ref(area, buf)
        ;
        // Render controls if any:
        let lv = vertical![==33%, ==67%]
            .flex(Flex::Center)
            .split(area)
        ;
        // Render control block if visible:
        self.cblock.render_ref(lv[0], buf);
    }
}