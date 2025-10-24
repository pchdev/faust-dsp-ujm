
use std::{f64::consts::PI};

use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    style::{Color}, 
    symbols, 
    widgets::{
        canvas::{
            Canvas, 
            Points
        }, 
        WidgetRef
    }
};

use crate::widgets::{control::block::ControlBlock, InteractiveWidget};

#[derive(Debug)]
pub struct Particles {
    pub tick: usize,
    pub frequency: usize,
    pub amplitude: usize,
    controls: ControlBlock
}

impl Default for Particles {
    fn default() -> Self {
        Particles { 
            tick: 0,
            frequency: 1,
            amplitude: 400,
            controls: ControlBlock::default()
                .add_button("test")
        }        
    }
}

impl Particles {
    fn position(&self, pos: (usize, usize)) -> (f64, f64) {
        // Spacing between each particle:
        const SPACING: f64 = 25.0;
        const PI_2: f64 = PI * 2.0;
        let (mut x, mut y) = (pos.0 as f64, pos.1 as f64);
        // Compute the phase:
        let ph = self.tick as f64 - x * SPACING;
        let ph = if ph < 0.0 { 0.0 } else { ph };
        let ph = ph / self.amplitude as f64;
        // When phase reaches 0.5
        x *= SPACING;
        x += 100.0;
        y *= SPACING;
        y += 100.0;
        let ph = (ph * PI_2).sin() * SPACING;
        (
            x + ph,
            y
        )
    }
}

impl InteractiveWidget for Particles {
    fn on_key_event(&mut self, k: KeyEvent) {

    }

    fn on_tick(&mut self, _: usize) {
        // TODO: frequency
        self.tick += 1;
        if self.tick >= self.amplitude {
           self.tick -= self.amplitude;
        }
    }
}

impl WidgetRef for Particles {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        const NROWS: usize = 8;
        const NCOLS: usize = 10;
        let mut coords = [[(0.0, 0.0); NCOLS]; NROWS];
        for (y, row) in &mut coords.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
               *col = self.position((x,y));
            }
        }
        Canvas::default()
            .marker(symbols::Marker::Braille)
            .background_color(Color::White)
            .paint(|ctx| {
                for row in coords {
                    ctx.draw(&Points {
                        coords: &row,
                        color: Color::Black                    
                    });
                }
            })            
            .x_bounds([00.0, 400.0 as f64])
            .y_bounds([00.0, 400.0 as f64])
            .render_ref(area, buf)
        ;        
    }
}