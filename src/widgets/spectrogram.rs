use ratatui::prelude::*;
use ratatui::widgets::canvas::{Canvas, Line};
use ratatui::widgets::WidgetRef;

use crate::widgets::InteractiveWidget;

#[derive(Default, Debug)]
pub struct SpectrumCanvas;

impl InteractiveWidget for SpectrumCanvas {}

impl WidgetRef for SpectrumCanvas {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Canvas::default()
            .marker(symbols::Marker::Braille)
            .background_color(Color::White)
            .x_bounds([0.0, 10.0])  // frequency bins (1 to 10 harmonics)
            .y_bounds([0.0, 1.2])   // normalized amplitude
            .paint(|ctx| {
                let harmonics = 10;
                let base_amp = 1.0;

                // Draw axes
                ctx.draw(&Line {
                    x1: 0.25,
                    y1: 0.25,
                    x2: 9.75,
                    y2: 0.25,
                    color: Color::Gray,
                });
                ctx.draw(&Line {
                    x1: 0.25,
                    y1: 0.25,
                    x2: 0.25,
                    y2: 1.0,
                    color: Color::Gray,
                });

                // Draw harmonic bars (sawtooth: amplitude ~ 1/n)
                for n in 1..=harmonics {
                    let amp = base_amp / n as f64;
                    ctx.draw(&Line {
                        x1: n as f64 - 0.75,
                        y1: 0.25,
                        x2: n as f64 - 0.75,
                        y2: amp + 0.25,
                        color: Color::Black,
                    });
                }

                // Labels
                ctx.print(0.2, 1.05, "Amplitude");
                ctx.print(8.0, 0.15, "Frequency →");
            })
            .render(area, buf);
    }
}
