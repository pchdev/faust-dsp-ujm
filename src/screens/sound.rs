


use ratatui::{buffer::Buffer, prelude::Rect, symbols, widgets::{canvas::{Canvas, Circle}, WidgetRef}};

use crate::screens::Screen;

#[derive(Debug, Default)]
pub struct Sound;

/* 
◌
◍
●
◯
*/ 


impl WidgetRef for Sound {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let circle = Circle {
            x: 250.0,
            y: 250.0,
            radius: 100.0,
            color: ratatui::style::Color::Red
        };
        Canvas::default()
            .marker(symbols::Marker::Dot)
            .paint(|ctx| {
                ctx.draw(&circle)
            })
            .x_bounds([00.0, 500.0 as f64])
            .y_bounds([00.0, 500.0 as f64])
            .render_ref(area, buf)
        ;
    }
}

impl Screen for Sound {
    fn on_key_event(&mut self, k: crossterm::event::KeyEvent) {
        
    }
}