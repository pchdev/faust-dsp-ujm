
use crossterm::event::{KeyCode, KeyEvent};
use num_derive::FromPrimitive;
use ratatui::{
    buffer::Buffer, layout::Flex, prelude::Rect, style::{Style, Stylize}, text::Text, widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, StatefulWidgetRef, Widget, WidgetRef, Wrap
    }
};
use ratatui_macros::{horizontal, text, line, vertical};

use crate::{
    screens::Screen, 
    widgets::ripple::{Ripple}
};

use indoc::indoc;

const SOUND: &'static str = indoc!{"
┏━┓┏━┓╻ ╻┏┓╻╺┳┓
┗━┓┃ ┃┃ ┃┃┗┫ ┃┃
┗━┛┗━┛┗━┛╹ ╹╺┻┛
"};

#[derive(Debug, Default)]
enum State {
    #[default]
    Start,
    Ripple(Ripple),
    Displacement,
    Waveform
}

impl TryFrom<usize> for State {
    type Error = ();
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Start),
            1 => Ok(Self::Ripple(Ripple::default())),
            2 => Ok(Self::Displacement),
            3 => Ok(Self::Waveform),
            _ => Err(())
        }
    }
}

#[derive(Debug, Default)]
pub struct Sound {
     state: State,
    lstate: ListState
}

impl WidgetRef for Sound {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        let [lhl, lhr] = horizontal![==50%, ==50%]
            .flex(Flex::Center)
            .areas(area);     
        let lhlv = vertical![==5%, ==20%, ==75%]
            .flex(Flex::Center)
            .horizontal_margin(5)
            .split(lhl)
        ;
        let lhrb = Block::bordered()
            .borders(Borders::LEFT)
            .border_type(BorderType::Plain)
            .render(lhr, buf)
        ;
        let title = Paragraph::new(SOUND)
            .centered();
        title.render(lhlv[1], buf);
        let mut state = self.lstate.clone();
        let txt = indoc! {
            "- Sound is a **pressure wave** that propagates through a **medium** (*gas*, *liquid* or *solid*)"
        };
        let t = Paragraph::new(
                tui_markdown::from_str(txt)
            )
            .wrap(Wrap {trim: true })
        ;
        t.render(lhlv[2], buf);

        match &self.state {
            State::Ripple(r) => {
                let mut running = true;
                r.render_ref(lhr, buf, &mut running);
            }
            _ => ()
        }
    }
}

impl Screen for Sound {
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Down => {
                self.lstate.select_next();
            }
            KeyCode::Up => {
                self.lstate.select_previous();
            }
            KeyCode::Enter => {
                match self.lstate.selected() {
                    Some(0) => {
                        self.state = State::Ripple(
                            Ripple { 
                                tick: 0,
                                frequency: 1,
                                amplitude: 250
                            }
                        );
                    }
                    _ => ()
                }

            }
            _ => ()
        }
    }
    fn on_tick(&mut self, t: usize) {
        match &mut self.state {
            State::Ripple(r) => {
                r.on_tick(t);
            }
            _ => ()
        }
    }
}