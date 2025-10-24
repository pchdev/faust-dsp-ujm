
use ratatui::{
    buffer::Buffer, 
    layout::{Flex, Rect}, 
    style::Stylize, 
    text::Text, 
    widgets::{
        Widget
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


impl Screen for Splash {
    fn title(&self) -> &'static str {
        FAUST
    }
    fn description(&self) -> &'static str {
        "Splash"
    }
    fn build() -> (Box<dyn Screen>, Option<Box<dyn Layout>>) where Self: Sized {
        (Box::new(Splash::default()), None)
    }
    fn render(&self, 
           _: &Option<Box<dyn Layout>>, 
        area: Rect, 
         buf: &mut Buffer
    ) {
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