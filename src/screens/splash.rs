
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, layout::{
        Constraint, 
        Flex, 
        Layout, 
        Rect
    }, style::Stylize, text::{
        Line, Text
    }, 
    widgets::{
        StatefulWidgetRef, Widget, WidgetRef
    }
};
use ratatui_macros::{horizontal, vertical, line};

use indoc::indoc;

use crate::screens::Screen;

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

enum State {
    Start,
    Dissolve(usize)
}

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
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Down => {

            }
            KeyCode::Enter => {
                
            } 
            _ => ()
        }
    }
}