
use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    buffer::Buffer, 
    layout::{
        Flex, 
    }, 
    prelude::Rect, 
    widgets::{
        Block, 
        BorderType, 
        Borders, 
        ListState, 
        Paragraph, 
        Widget, WidgetRef, 
    }
};

use indoc::indoc;

use ratatui_macros::{
    horizontal, 
};

use crate::{
    screens::{ContentArea, Screen, leafy}, 
    widgets::{
        particles::Particles, 
        ripple::Ripple, 
        waveform::Waveform
    }
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
╺┳┓╻┏━╸╻╺┳╸┏━┓╻     ┏━┓╻┏━╸┏┓╻┏━┓╻  
 ┃┃┃┃╺┓┃ ┃ ┣━┫┃     ┗━┓┃┃╺┓┃┗┫┣━┫┃  
╺┻┛╹┗━┛╹ ╹ ╹ ╹┗━╸   ┗━┛╹┗━┛╹ ╹╹ ╹┗━╸
"};

#[derive(Debug, Default)]
enum Animation {
    #[default]
    None,
    Ripple(Ripple),
    Particles(Particles),
    Waveform(Waveform)
}

#[derive(Debug)]
enum Content<'a> {
    Paragraph(Paragraph<'a>),
    List(Vec<String>, ListState)
}

pub struct Digital<'a> {
    lhs: ContentArea<'a>, 
    rhs: Animation,
}

impl<'a> Default for Digital<'a> {
    fn default() -> Self {
        Digital {
            lhs: ContentArea::default()
                .add_title(TITLE)
                .add_paragraph(indoc! {
                    "• To ***digitize*** a continuous signal means implies discretizing it. \
                    This is made possible by two processes: ***sampling*** & ***quantization***.
                    "
                })
                .add_paragraph(indoc! {
                    "• ***Sampling*** means taking a sample of a signal at a certain frequency/rate (***sample rate***).
                    "
                })
                .add_list(vec![
                    "• **Audio CD**: 44.1 kHz",
                    "• **Pro Audio**: 48/96 kHz",
                    "• **MP3**: ?"
                ])
                .add_paragraph(indoc! {
                    "• Once we take a sample at a given time, we need to determine the scale of its value. \
                    This is called ***Quantization***. \
                    The larger the scale is, the lesser quantization noise (quality vs. storage tradeoff).
                    "
                })
                .add_list(vec![
                    "• **Audio CD**: 16-bits (range of 65,536 values)",
                    "• **Pro Audio**: 24-bits (~16 million)",
                    "• **DSP**: 32-bits floating-point"
                ])
                ,
            rhs: Animation::default(),            
        }
    }
}

impl<'a> WidgetRef for Digital<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        let [lhl, lhr] = horizontal![==50%, ==50%]
            .flex(Flex::Center)
            .areas(area)
        ;
        Block::bordered()
            .borders(Borders::LEFT)
            .border_type(BorderType::Plain)
            .render(lhr, buf)
        ;
        self.lhs.render_ref(lhl, buf);
        match &self.rhs {
            Animation::Ripple(r) => {
                r.render_ref(lhr, buf);
            }
            Animation::Particles(p) => {
                p.render_ref(lhr, buf);
            }
            Animation::Waveform(p) => {
                p.render_ref(lhr, buf);
            }
            _ => ()
        }
    }
}

impl<'a> Screen for Digital<'a> {
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Down | KeyCode::Up => {
                self.lhs.on_key_event(k);
            }
            KeyCode::Enter => {
                match self.lhs.select {
                    2..=3=> {
                        self.rhs = Animation::Waveform(
                            Waveform::new(100, 25)
                        );
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }
    fn on_tick(&mut self, t: usize) {
        match &mut self.rhs {
            Animation::Ripple(r) => {
                r.on_tick(t);
            }
            Animation::Particles(p) => {
                p.on_tick(t);
            }
            Animation::Waveform(w) => {
                w.on_tick(t);
            }
            _ => ()
        }
    }
}