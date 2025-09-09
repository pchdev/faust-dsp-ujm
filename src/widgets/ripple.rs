
use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer, prelude::Rect, style::{Color, Style}, symbols, widgets::{
        canvas::{
            Canvas, 
            Circle
        }, StatefulWidgetRef, WidgetRef
    }
};

#[derive(Debug, Default)]
pub struct Ripple {
    pub frame: usize
}

impl StatefulWidgetRef for Ripple {
    type State = usize;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Canvas::default()
            .marker(symbols::Marker::Braille)
            .background_color(Color::White)
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 250.0,
                    y: 250.0,
                    radius: 4.0,
                    color: Color::Black
                });
                ctx.draw(&Circle {
                    x: 250.0,
                    y: 250.0,
                    radius: 50.0,
                    color: Color::Black
                });
            })            
            .x_bounds([00.0, 500.0 as f64])
            .y_bounds([00.0, 500.0 as f64])
            .render_ref(area, buf)
        ;        
    }
}