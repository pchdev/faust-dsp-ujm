use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, style::Stylize, symbols::border, widgets::{Block, WidgetRef}};




pub(crate) struct ControlBlock {
    controls: Vec<Box<dyn WidgetRef>>,
}

impl ControlBlock {
    fn add_widget<T>(&mut self, w: T) 
        where T: WidgetRef
    {
        // self.controls.push(Box::new(w));
    }
}

impl WidgetRef for ControlBlock {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(" controls ")
            .border_set(border::ROUNDED)
        ;   
        // Compute the constraints:
        let div = 100.0 / self.controls.len() as f64;
        let constraints: Vec<Constraint> = (0..self.controls.len())
            .map(|_| Constraint::Percentage(div as u16))
            .collect()
        ;
    }
}

