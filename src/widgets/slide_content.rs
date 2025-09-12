use ratatui::{buffer::Buffer, layout::Rect, text::Text, widgets::{Paragraph, Widget}};

pub struct SlideContent<'a> {
    text: Paragraph<'a>
}

impl<'a> SlideContent<'a> {

    fn parse(str: String) {
        // Parse each word, 
    }
    pub fn new(str: String) -> SlideContent<'a> {
        SlideContent { 
            text: Paragraph::new("")
        }
    }
}


impl<'a> Widget for SlideContent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        
    }
}