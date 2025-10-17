use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::{Block, Row, Table, WidgetRef}};
use ratatui_macros::constraints;

use crate::widgets::InteractiveWidget;

#[derive(Debug, Default)]
pub struct Decibels;


// https://en.wikipedia.org/wiki/Decibel
// dBFS
// dB SPL
// dBu or dBv
// dBV
// LUFS (television)

impl InteractiveWidget for Decibels {}

impl WidgetRef for Decibels {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let rows = [
            Row::new(vec![
                "Scale", "1", "2", "4", "5", "10"
                ]
            ),
            Row::new(vec![
                "dB", "0", "1", "3", "6", "7", "10"
            ])
        ];
        let w = constraints![
            ==50%, ==10%, ==10%, ==10%, ==10%, ==10% 
        ];
        let table = Table::new(rows, w)
                .column_spacing(1)
                .style(Style::default())
                .block(Block::new().title("Table"))
        ;
        table.render_ref(area, buf);
    }
}