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

#[derive(Debug, Default)]
pub struct Ripple {
    pub tick: usize,
    cblock: ControlBlock
}

impl Ripple {
    pub(crate) fn new() -> Self {
        Ripple {
            tick: 0,
            cblock: 
                ControlBlock::default()
                    .add_slider("amplitude", 25.0, 0.0..200.0)
                    .add_slider("frequency", 25.0, 1.0..100.0)
        }
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
                } 
            }
        }        
    }
    fn on_tick(&mut self, tick: usize) {
        // TODO: frequency
        let amplitude = self.cblock.read_control(0).unwrap() as usize * 4; 
        if tick % 2 == 0 {
            self.tick += 1;
        }
        if self.tick >= amplitude {
           self.tick -= amplitude;
        }
    }
}

impl WidgetRef for Ripple {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let ncircles = 3;
        let amplitude = self.cblock.read_control(0).unwrap() as f64;
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
                for n in 0..ncircles {
                    let pos = n*25;
                    let delayed = if self.tick < pos { 0 } else { 
                        self.tick - pos
                    };
                    let c = delayed as u8;
                    let color = Color::Rgb(c, c, c);
                    ctx.draw(&Circle {
                        x: 200.0,
                        y: 200.0,
                        radius: delayed as f64,
                        color: color
                    });
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