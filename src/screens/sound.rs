

use indoc::indoc;
use macros::Screen;
use ratatui::widgets::{Paragraph, WidgetRef};

use crate::{
    leafy,
    screens::{
        layouts::{
            plainfull::PlainFull, 
            sidebyside::SideBySide, 
            Layout
        }, Screen, ScreenList, ScreenParagraph
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


use crate::screens::layouts::LayoutEnum;




#[derive(Screen, Default)]
#[screen(layout = LayoutEnum::SideBySide)]
#[screen(title = TITLE)]
#[screen(description = "Sound (1/2)")]
pub struct Sound {
    // ---------------------------------------------------
    /// Sound is a ***pressure wave*** that propagates
    /// through a **medium** (*gas*, *liquid* or *solid*).
    p1: (ScreenParagraph, Ripple),
    // ---------------------------------------------------
    /// Propagation is caused by the **oscillation** (*vibration*) of
    /// the medium's *particles*, around their ***equilibrium*** positions.
    p2: (ScreenParagraph, Particles),
    // ---------------------------------------------------
    /// Sound has the following **properties**:
    p3: ScreenParagraph,
    // ---------------------------------------------------
    /// • **Speed**: ~343 m/s in **air**",
    /// • **Amplitude**: in *Pascals* (***Pa***) or *Decibels* (***dB***)",
    /// • **Period**: time between two oscillations",
    /// • **Wavelength**: distance between two oscillations",
    /// • **Frequency**: cycles/sec., in *Hertz* (***Hz***, ***kHz***, ***MHz***)",
    /// • **Spectrum**, or *Timbre*"
    l0: ScreenList
}

// pub struct Sound2<'a> {
//     layout: PlainFull<'a>,
// }

// impl<'a> WidgetRef for Sound2<'a> {
//     fn render_ref(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
//         self.layout.render_ref(area, buf);
//     }
// }

// impl<'a> Screen for Sound2<'a> {
//     fn title(&self) -> &'static str {
//         TITLE
//     }
//     fn description(&self) -> &'static str {
//         "Sound (2/2)"
//     }
//     fn layout(&self) -> Option<&dyn Layout> {
//         Some(&self.layout)
//     }
//     fn layout_mut(&mut self) -> Option<&mut dyn Layout> {
//         Some(&mut self.layout)
//     }
// }

// impl<'a> Default for Sound2<'a> {
//     fn default() -> Self {
//         Sound2 {
//             layout: PlainFull::default()
//                 .add_title(TITLE)
//                 .add_paragraph(indoc! {
//                     "• Our ***perception*** of sound is made from the conversion of the vibrations reaching our ***eardrums*** to \
//                     a *signal of nerve impulses*, transmitted and interpreted by **the brain**."
//                 })
//                 .add_paragraph(indoc! {
//                     "• Human ears can typically identify sounds ***from 20 Hz to 20 kHz***."
//                 })
//                 .add_list(vec![
//                     "• **Bat**: 2000 to 110,000 Hz",
//                     "• **Porpoise**: 75 to 150,000 Hz",
//                     "• **Cat**: 45 to 64,000 Hz",
//                     "• **Dog**: 67 to 45,000 Hz",
//                     "• **Chicken**: 125 to 2,000 Hz"
//                 ]
//             )
//         }
//     }
// }

