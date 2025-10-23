

use indoc::indoc;
use ratatui::widgets::WidgetRef;

use crate::{
    leafy,
    screens::{
        layouts::{
            plainfull::PlainFull, 
            sidebyside::SideBySide, 
            Layout
        }, 
        Screen
    }, 
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
    layout: SideBySide<'a>,
}

impl<'a> WidgetRef for Sound<'a> {
    fn render_ref(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.layout.render_ref(area, buf);
    }
}

impl<'a> Screen for Sound<'a> {
    fn title(&self) -> &'static str {
        TITLE
    }
    fn description(&self) -> &'static str {
        "Sound (1/2)"
    }
    fn layout(&self) -> Option<&dyn Layout> {
        Some(&self.layout)
    }
    fn layout_mut(&mut self) -> Option<&mut dyn Layout> {
        Some(&mut self.layout)
    }
}



impl<'a> Default for Sound<'a> {
    fn default() -> Self {
        Sound {
            layout: SideBySide::default()
                .add_title(TITLE)
                .add_paragraph(leafy!(
                    "Sound is a ***pressure wave*** that propagates \
                    through a **medium** (*gas*, *liquid* or *solid*).
                    "
                ))
                .add_widget(Box::new(Ripple::new()))
                .add_paragraph(leafy! {
                    "Propagation is caused by the **oscillation** (*vibration*) of \
                    the medium's *particles*, around their ***equilibrium*** positions.
                    "
                })
                .add_widget(Box::new(Particles::new(400)))
                .add_paragraph(indoc! {
                    "• Sound has the following **properties**:"
                })
                .add_list(vec! {
                    "• **Speed**: ~343 m/s in **air**",
                    "• **Amplitude**: in *Pascals* (***Pa***) or *Decibels* (***dB***)",
                    "• **Period**: time between two oscillations",
                    "• **Wavelength**: distance between two oscillations",
                    "• **Frequency**: cycles/sec., in *Hertz* (***Hz***, ***kHz***, ***MHz***)",
                    "• **Spectrum**, or *Timbre*"
                })
        }
        // • Speed:
        //  - Air: ~340 m/s
        //  - Water: ~1,480 m/s
        //  - Steel: ~5,960 m/s
        //  - Solid atomic hydrogen: ~36,000 m/s
        //  - Speed of light: 299,792,458 m/s
    }
}

pub struct Sound2<'a> {
    layout: PlainFull<'a>,
}

impl<'a> WidgetRef for Sound2<'a> {
    fn render_ref(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.layout.render_ref(area, buf);
    }
}

impl<'a> Screen for Sound2<'a> {
    fn title(&self) -> &'static str {
        TITLE
    }
    fn description(&self) -> &'static str {
        "Sound (2/2)"
    }
    fn layout(&self) -> Option<&dyn Layout> {
        Some(&self.layout)
    }
    fn layout_mut(&mut self) -> Option<&mut dyn Layout> {
        Some(&mut self.layout)
    }
}

impl<'a> Default for Sound2<'a> {
    fn default() -> Self {
        Sound2 {
            layout: PlainFull::default()
                .add_title(TITLE)
                .add_paragraph(indoc! {
                    "• Our ***perception*** of sound is made from the conversion of the vibrations reaching our ***eardrums*** to \
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
    }
}

