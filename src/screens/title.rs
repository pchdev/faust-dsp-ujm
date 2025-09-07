
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer, layout::{
        Constraint, Flex, Layout, Rect
    }, style::Stylize, text::{
        Line, Text
    }, widgets::{Widget, WidgetRef}
};

use crate::screens::Screen;

/// Got from: https://patorjk.com/software/taag/
/// Font is 'ANSI Shadow'
const FAUST: &'static str = r"
███████╗ █████╗ ██╗   ██╗███████╗████████╗
██╔════╝██╔══██╗██║   ██║██╔════╝╚══██╔══╝
█████╗  ███████║██║   ██║███████╗   ██║   
██╔══╝  ██╔══██║██║   ██║╚════██║   ██║   
██║     ██║  ██║╚██████╔╝███████║   ██║   
╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   
                                          
";

#[derive(Debug, Default)]
pub struct Title;

impl WidgetRef for Title {
    fn render_ref(&self, area: Rect, buf: &mut Buffer)
    {
        let mut txt = Text::from(FAUST);
        txt.push_line("");
        txt.push_line(Line::from(
            "Digital Audio Processing and Synthesis")
            .bold()
        );
        let [lv] = Layout::vertical([
                Constraint::Length(txt.height() as u16),
            ])
            .flex(Flex::Center)
            .areas(area)
        ;
        let [lh] = Layout::horizontal([
                Constraint::Length(txt.width() as u16)
            ])
            .flex(Flex::Center)
            .areas(lv)
        ;
        txt.render(lh, buf);
    }
}

impl Screen for Title {
    fn on_key_event(&mut self, k: crossterm::event::KeyEvent) {
        match k.code {
            KeyCode::Down => {

            }
            KeyCode::Enter => {
                
            } 
            _ => ()
        }
    }
}