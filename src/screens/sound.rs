
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
    widgets::{particles::Particles, ripple::Ripple, waveform::Waveform}
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
    Waveform(Waveform)
}

#[derive(Debug)]
pub struct Sound<'a> {
     state: State,
    lstate: ListState,
    paragraphs: Vec<Paragraph<'a>>
}


macro_rules!  add_paragraph {
    ($obj:expr, $str:literal) => {
        $obj.paragraphs.push(
                Paragraph::new(tui_markdown::from_str( indoc!{$str}
            )).wrap(Wrap {trim: true })
        )
    };
}

impl<'a> Default for Sound<'a> {
    
    fn default() -> Self {
        let mut s = Sound {
            state: State::default(),
            lstate: ListState::default(),
            paragraphs: vec![]
        };
        add_paragraph!(s, 
            "• Sound is a ***pressure wave*** that propagates \
            through a **medium** (*gas*, *liquid* or *solid*)."
        );
        add_paragraph!(s, 
            "• The propagation is carried by the **periodic oscillation** (*vibration*) of \
            the medium's particles around their point of origin."
        );
        add_paragraph!(s,
            "• Once the wave reaches our ears, our ear-drum starts to vibrate, transmitting a signal \
            to the brain."
        );
        add_paragraph!(s,
            "• **Properties and measurement**:
            - **Amplitude**: in *Pascals* (*Pa*) or *Decibels* (*dB*)
            - **Frequency**: in *Hertz* (Hz, kHz, MHz)
            - **Spectrum**: or *Timbre*
            "
        );
        add_paragraph!(s,
            "• **Speed**:
                - **Air**: ~340 m/s
                - **Water**: ~1,480 m/s
                - **Steel**: ~5,960 m/s
                - **Solid atomic hydrogen**: ~36,000 m/s
                - **Speed of light**: 299,792,458 m/s
            "
        );
        return s;
        
    }
}

impl<'a> WidgetRef for Sound<'a> {
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
        let mut constraints: Vec<Constraint> = vec![];
        let mut paragraphs: Vec<Paragraph> = vec![];
        for (n, paragraph) in self.paragraphs.iter().enumerate() {
            let mut ph = paragraph.clone();
            if self.lstate.selected().is_some_and(|s| n == s) {
                ph = ph.black().on_gray();
            }
            let lc = ph.line_count(lhlv[2].width) + 1;
            constraints.push(
                Constraint::Length(lc.try_into().unwrap())
            );
            paragraphs.push(ph);
        }
        let lp = Layout::vertical(constraints).split(lhlv[2]);
        for (n, paragraph) in paragraphs.iter().enumerate() {
            paragraph.render(lp[n], buf);
        }
        match &self.state {
            State::Ripple(r) => {
                r.render_ref(lhr, buf);
            }
            State::Particles(p) => {
                p.render_ref(lhr, buf);
            }
            State::Waveform(p) => {
                p.render_ref(lhr, buf);
            }
            _ => ()
        }
    }
}

impl<'a> Screen for Sound<'a> {
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
                                amplitude: 400
                            }
                        )
                    }
                    Some(2) => {
                        self.state = State::Waveform(
                            Waveform { 
                                tick: 0, 
                                frequency: 1, 
                                amplitude: 400 
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
            State::Waveform(w) => {
                w.on_tick(t);
            }
            _ => ()
        }
    }
}