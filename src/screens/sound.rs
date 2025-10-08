
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, prelude::Rect, 
    widgets::{WidgetRef}
};

use indoc::indoc;

use crate::{
    screens::{leafy, Screen, SideBySide}, 
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

pub struct Sound<'a> {
    screen: SideBySide<'a>,
}

impl<'a> WidgetRef for Sound<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        self.screen.render_ref(area, buf);
    }
}

impl<'a> Screen for Sound<'a> {
    fn title(&self) -> &'static str {
        "Sound"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.screen.on_key_event(k);
    }

    fn on_tick(&mut self, t: usize) {
        self.screen.on_tick(t);
    }
}

impl<'a> Default for Sound<'a> {
    fn default() -> Self {
        Sound {
            screen: SideBySide::default()
                .add_title(TITLE)
                .add_paragraph(leafy!(
                    "Sound is a ***pressure wave*** that propagates \
                    through a **medium** (*gas*, *liquid* or *solid*).
                    "
                ))
                .add_widget(0, Box::new(Ripple::new(200)))
                .add_paragraph(leafy! {
                    "Propagation is carried by the **periodic oscillation** (*vibration*) of \
                    the medium's *particles* around their point of origin.
                    "
                })
                .add_widget(1, Box::new(Particles::new(400)))
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
                ]
            )
        }
        // • Speed:
        //  - Air: ~340 m/s
        //  - Water: ~1,480 m/s
        //  - Steel: ~5,960 m/s
        //  - Solid atomic hydrogen: ~36,000 m/s
        //  - Speed of light: 299,792,458 m/s
    }
}

