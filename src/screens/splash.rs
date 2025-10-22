
use ratatui::{
    buffer::Buffer, 
    layout::{Flex, Rect}, 
    style::Stylize, 
    text::Text, 
    widgets::{
        Widget, WidgetRef
    }
};
use ratatui_macros::{horizontal, vertical, line};

use indoc::indoc;

use crate::screens::{Layout, Screen};

/// Got from: https://patorjk.com/software/taag/
/// Font is 'ANSI Shadow'
const FAUST: &'static str = indoc!{"
    ███████╗ █████╗ ██╗   ██╗███████╗████████╗
    ██╔════╝██╔══██╗██║   ██║██╔════╝╚══██╔══╝
    █████╗  ███████║██║   ██║███████╗   ██║   
    ██╔══╝  ██╔══██║██║   ██║╚════██║   ██║   
    ██║     ██║  ██║╚██████╔╝███████║   ██║   
    ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   

"};

#[derive(Debug, Default)]
pub struct Splash;


impl WidgetRef for Splash {   
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut txt = Text::from(FAUST);
        txt.push_line("");
        txt.push_line(
            line![" Digital Audio Processing and Synthesis"]
            .bold()
        );
        let [lv] = vertical![==txt.height() as u16]
            .flex(Flex::Center)
            .areas(area)
        ;
        let [lh] = horizontal![==txt.width() as u16]
            .flex(Flex::Center)
            .areas(lv)
        ;
        txt.render(lh, buf);
    }
}

impl Screen for Splash {
    fn title(&self) -> &'static str {
        FAUST
    }
    fn description(&self) -> &'static str {
        "Splash"
    }
    fn layout(&self) -> Option<&dyn Layout> {
        None
    }
    fn layout_mut(&mut self) -> Option<&mut dyn Layout> {
        None
    }
}