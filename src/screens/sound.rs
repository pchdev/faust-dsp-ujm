
use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    buffer::Buffer, layout::Flex, prelude::Rect, 
    style::{Modifier, Style, Stylize}, 
    widgets::{
        Block, 
        BorderType, 
        Borders, 
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
    }
};

const TITLE: &'static str = indoc!{"
┏━┓┏━┓╻ ╻┏┓╻╺┳┓
┗━┓┃ ┃┃ ┃┃┗┫ ┃┃
┗━┛┗━┛┗━┛╹ ╹╺┻┛
"};

#[derive(Debug, Default)]
enum Animation {
    #[default]
    None,
    Ripple(Ripple),
    Particles(Particles),
}

impl Animation {
    pub(crate) fn on_key_event(&mut self, k: KeyEvent) {
        match self {
            Self::Ripple(r) => {
                r.on_key_event(k);
            }
            Self::Particles(p) => {

            }
            _ => ()
        }
    }
}

pub struct Sound<'a> {
    lhs: ContentArea<'a>, 
    rhs: Animation,
    rhs_focus: bool,
}

impl<'a> Default for Sound<'a> {
    fn default() -> Self {
        Sound {
            lhs: ContentArea::default()
                .add_title(TITLE)
                .add_paragraph(leafy!(
                    "Sound is a ***pressure wave*** that propagates \
                    through a **medium** (*gas*, *liquid* or *solid*).
                    "
                ))
                .add_paragraph(leafy! {
                    "Propagation is carried by the **periodic oscillation** (*vibration*) of \
                    the medium's *particles* around their point of origin.
                    "
                })
                .add_paragraph(indoc! {
                    "• A sound has the following **properties**:"
                })
                .add_list(vec! {
                    "• **Speed**: ~343 m/s in **air**",
                    "• **Amplitude**: in *Pascals* (***Pa***) or *Decibels* (***dB***)",
                    "• **Period**: the time between two oscillations",
                    "• **Wavelength**: the distance between two oscillations",
                    "• **Frequency**: oscillations/sec., in *Hertz* (***Hz***, ***kHz***, ***MHz***)",
                    "• **Spectrum**, or *Timbre*"
                })
                .add_paragraph(indoc! {
                    "• Our ***perception*** of sound is made from the translation of the vibrations reaching our ***eardrums*** to \
                    a *signal of nerve impulses*, transmitted and interpreted by **the brain**."
                })
                .add_paragraph(indoc! {
                    "• Human ears can typically identify sounds ***from 20 Hz to 20 kHz***."
                })
                .add_list(vec![
                    "• **Bat**: 2000 to 110,000 Hz",
                    "• **Porpoise**: 75 to 150,000 Hz",
                    "• **Cat**: 45 to 64,000 Hz",
                    "• **Dog**: 67 to 45,000 Hz",
                    "• **Chicken**: 125 to 2,000 Hz"
                ])
                ,
            rhs: Animation::default(),       
            rhs_focus: false,     
        }
        // • Speed:
        //  - Air: ~340 m/s
        //  - Water: ~1,480 m/s
        //  - Steel: ~5,960 m/s
        //  - Solid atomic hydrogen: ~36,000 m/s
        //  - Speed of light: 299,792,458 m/s
    }
}

impl<'a> WidgetRef for Sound<'a> {
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
        if self.rhs_focus {
            Block::bordered()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .style(Style::default().dark_gray())
                .render(lhr, buf)
            ;
        }
        self.lhs.render_ref(lhl, buf);
        match &self.rhs {
            Animation::Ripple(r) => {
                r.render_ref(lhr, buf);
            }
            Animation::Particles(p) => {
                p.render_ref(lhr, buf);
            }
            _ => ()
        }
    }
}

impl<'a> Screen for Sound<'a> {
    fn title(&self) -> &'static str {
        "Sound"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        if self.rhs_focus {
            if k.code == KeyCode::Backspace {
                self.rhs_focus = false;
            } else {
                self.rhs.on_key_event(k);
            }
        } else {
            match k.code {
                KeyCode::Enter => {
                    self.rhs_focus = true;
                    match self.lhs.select {
                        0  => {
                            self.rhs = Animation::Ripple(
                                Ripple::new(200)
                            );
                        }
                        1 => {
                            self.rhs = Animation::Particles(
                                Particles::new(400)
                            )
                        }
                        2 => {}
                        3  => {
                            self.rhs = Animation::Ripple(
                                Ripple::new(200)
                            );
                        }
                        4 => {}
                        5  => {
                            self.rhs = Animation::Ripple(
                                Ripple::new(200)
                            );
                        }
                        _ => ()
                    }
                }
                _ => {
                    self.lhs.on_key_event(k);
                }
            }
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
            _ => ()
        }
    }
}