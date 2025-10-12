use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect, style::Color, symbols, widgets::{canvas::{Canvas, Circle, Line, Points}, Block, Borders, Widget, WidgetRef}
};

use crate::widgets::InteractiveWidget;

pub struct Quantization {
    pub signal_freq: f64,   // cycles across the canvas width
    pub sample_rate: usize, // number of samples
    pub amplitude: f64,
    index: u8,
}

impl Default for Quantization {
    fn default() -> Self {
        Self {
            signal_freq: 1.0,
            sample_rate: 8,
            amplitude: 1.0,
            index: 0
        }
    }
}

impl InteractiveWidget for Quantization {
    fn on_key_event(&mut self, k: crossterm::event::KeyEvent) {
        match k.code {
            KeyCode::Enter => {
                self.index += 1;
                if self.index > 2 {
                    self.index = 0;
                }
            }
            _ => ()
        }
    }
}

impl WidgetRef for Quantization {
    fn render_ref(&self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // High-resolution "continuous" waveform
        let continuous_points = 400;
        let continuous: Vec<(f64, f64)> = (0..=continuous_points)
            .map(|i| {
                let x = i as f64 / continuous_points as f64;
                let y = (2.0 * std::f64::consts::PI * self.signal_freq * x).sin() * self.amplitude;
                (x, y)
            })
            .collect();

        // Sampled signal
        let samples: Vec<(f64, f64)> = (0..self.sample_rate)
            .map(|i| {
                let x = i as f64 / (self.sample_rate - 1).max(1) as f64;
                let y = (2.0 * std::f64::consts::PI * self.signal_freq * x).sin() * self.amplitude;
                (x, y)
            })
            .collect();

        let canvas = Canvas::default()
            // .block(Block::default().borders(Borders::ALL).title("Sampling Illustration"))
            .marker(symbols::Marker::Braille)
            .background_color(Color::White)
            .x_bounds([0.0, 1.0])
            .y_bounds([-1.1, 1.1])
            .paint(|ctx| {
                // Zero line
                ctx.draw(&Line {
                    x1: 0.0,
                    y1: 0.0,
                    x2: 1.0,
                    y2: 0.0,
                    color: Color::Gray,
                });

                // Continuous waveform (piecewise lines)
                for pair in continuous.windows(2) {
                    let (x1, y1) = pair[0];
                    let (x2, y2) = pair[1];
                    ctx.draw(&Line {
                        x1,
                        y1,
                        x2,
                        y2,
                        color: Color::Black,
                    });
                }

                // Sample points + stems
                for (x, y) in &samples {
                    // stem
                    ctx.draw(&Line {
                        x1: *x,
                        y1: 0.0,
                        x2: *x,
                        y2: *y,
                        color: Color::Red,
                    });
                    match self.index {
                        0 => {
                            let yq = (*y * i16::MAX as f64) as i16;
                            ctx.print(*x, *y, ratatui::text::Line::from(format!("{}", yq)));
                        }
                        1 => {
                            let yq = (*y * 8_388_607 as f64) as i32;
                            ctx.print(*x, *y, ratatui::text::Line::from(format!("{}", yq)));
                        }
                        2 => {
                            ctx.print(*x, *y, ratatui::text::Line::from(format!("{}", *y)));
                        }
                        _ => ()
                    };

                    // small cross marker for the sample
                    ctx.draw(&Circle {
                        x: *x,
                        y: *y,
                        radius: 0.0001,
                        color: Color::Red
                    });

                }
            });

        canvas.render(area, buf);
    }
}
