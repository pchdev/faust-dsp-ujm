
use crossterm::event::{KeyCode, KeyEvent};
use num_derive::FromPrimitive;
use ratatui::{
    buffer::Buffer, layout::{Constraint, Flex, Layout}, prelude::Rect, style::{Style, Stylize}, text::Text, widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, StatefulWidgetRef, Widget, WidgetRef, Wrap
    }
};
use ratatui_macros::{horizontal, text, line, vertical};

use crate::{
    screens::Screen, 
    widgets::{particles::Particles, ripple::Ripple}
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
    Particles(Particles),
    Waveform
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
        Block::bordered()
            .borders(Borders::LEFT)
            .border_type(BorderType::Plain)
            .render(lhr, buf)
        ;
        Paragraph::new(SOUND)
            .centered()
            .render(lhlv[1], buf)
        ;
        let txt1 = tui_markdown::from_str(indoc! {
            "• Sound is a ***pressure wave*** that propagates \
            through a **medium** (*gas*, *liquid* or *solid*)."
        });
        let txt2 = tui_markdown::from_str(indoc! {
            "• The propagation is carried by the **oscillation** of the medium's particles around \
            their point of origin."
        });
        let mut t1 = Paragraph::new(txt1).wrap(Wrap {trim: true });
        let mut t2 = Paragraph::new(txt2).wrap(Wrap {trim: true });
        match self.lstate.selected() {
            Some(0) => {
                t1 = t1.black().on_gray()
            }
            Some(1) => {
                t2 = t2.black().on_gray()
            }
            _ => ()
        }
        let t1c = t1.line_count(lhlv[2].width) + 1;
        let t2c = t2.line_count(lhlv[2].width) + 1;
        let lp = Layout::vertical([
                Constraint::Length(t1c.try_into().unwrap()),
                Constraint::Length(t2c.try_into().unwrap()),
            ])
            .split(lhlv[2])
        ;

        match &self.state {
            State::Ripple(r) => {
                r.render_ref(lhr, buf);
            }
            State::Particles(p) => {
                p.render_ref(lhr, buf);
            }
            _ => ()
        }
        t1.render(lp[0], buf);
        t2.render(lp[1], buf);
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
                                amplitude: 200
                            }
                        );
                    }
                    Some(1) => {
                        self.state = State::Particles(
                            Particles {
                                tick: 0,
                                frequency: 1,
                                amplitude: 300
                            }
                        )
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
            State::Particles(p) => {
                p.on_tick(t);
            }
            _ => ()
        }
    }
}