
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
┏━┓╻┏━╸┏┓╻┏━┓╻  
┗━┓┃┃╺┓┃┗┫┣━┫┃  
┗━┛╹┗━┛╹ ╹╹ ╹┗━╸
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

pub struct Signal<'a> {
    lhs: ContentArea<'a>, 
    rhs: Animation,
}

impl<'a> Default for Signal<'a> {
    fn default() -> Self {
        Signal {
            lhs: ContentArea::default()
                .add_title(TITLE)
                .add_paragraph(indoc! {
                    "• A ***signal*** describes the evolution of data *over time*. \
                    In our case, the periodic oscillation of an entity (like the *membrane* of a ***microphone***).
                    "
                })
                .add_paragraph(indoc! {
                    "• Just like *nerve impulses* carried to the brain, the analyzed data usually needs to be first converted \
                    to another *physical unit*, or *domain* (***transduction***) in order to adapt to measurement tools.
                    "
                })
                .add_paragraph(indoc! {
                    "• The vibration of a microphone's membrane is, for instance, usually converted to ***continuous electrical current***, \
                    and can be then processed and/or analyzed. In this case, the signal is said to be ***\"analog\"***.
                    "
                })
                .add_paragraph(leafy! {
                    "With an oscilloscope, we can measure the **amplitude** of a signal at a given *point in time* (***time-domain***), \
                    through the visualisation of a ***waveform***."
                })   
                .add_paragraph(indoc! {
                    "• An analog signal can already be processed as it is, with ***analog effects***: \
                    *tape delay*, *distortion*, *chorus*, *reverberation (spring, plate)*, etc."
                })
                // TODO: add spectrogram animation?             
                .add_paragraph(leafy! {
                    "On the other hand, it is difficult to extract precise information about *frequency* and *spectrum*. \
                    For this purpose, it's far more efficient to switch to the ***frequency domain***, which requires the analog signal to \
                    be turned into a ***digital signal***..."
                })                
                ,
            rhs: Animation::default(),            
        }
    }
}

impl<'a> WidgetRef for Signal<'a> {
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

impl<'a> Screen for Signal<'a> {
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