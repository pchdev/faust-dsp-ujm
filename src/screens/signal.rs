

use indoc::indoc;

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
    widgets::{spectrogram::SpectrumCanvas, waveform::Waveform}, 
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━┓╻┏━╸┏┓╻┏━┓╻  
┗━┓┃┃╺┓┃┗┫┣━┫┃  
┗━┛╹┗━┛╹ ╹╹ ╹┗━╸
"};

pub struct Signal<'a> {
    layout: PlainFull<'a>,
}

impl<'a> Screen for Signal<'a> {
    fn title(&self) -> &'static str {
        TITLE
    }
    fn description(&self) -> &'static str {
        "Signal (1/2)"
    }
    fn layout(&self) -> Option<&dyn Layout> {
        Some(&self.layout)
    }
    fn layout_mut(&mut self) -> Option<&mut dyn Layout> {
        Some(&mut self.layout)
    }
}

impl<'a> Default for Signal<'a> {
    fn default() -> Self {
        Signal {
            layout: PlainFull::default()
                .add_title(TITLE)
                .add_paragraph(indoc! {
                    "• A ***signal*** describes the evolution of data *over time*. \
                    In our case, the vibration of an entity (like the *membrane* of a ***microphone***).
                    "
                })
                .add_paragraph(indoc! {
                    "• Just like sound waves turning into nerve impulses, the analyzed data usually needs to be first converted \
                    to another *physical unit*, or *domain* (***transduction***) in order to adapt to measurement/processing tools.
                    "
                })
                .add_paragraph(indoc! {
                    "• The vibration of a microphone's membrane is, for instance, usually converted to ***continuous electrical current***, \
                    before it can be processed and/or analyzed. In this case, the signal is said to be ***\"analog\"***.
                    "
                })    
                ,            
        }
    }
}

pub struct Signal2<'a> {
    layout: SideBySide<'a>,
}

impl<'a> Screen for Signal2<'a> {
    fn title(&self) -> &'static str {
        TITLE
    }
    fn description(&self) -> &'static str {
        "Signal (2/2)"
    }
    fn layout(&self) -> Option<&dyn Layout> {
        Some(&self.layout)
    }
    fn layout_mut(&mut self) -> Option<&mut dyn Layout> {
        Some(&mut self.layout)
    }
}

impl<'a> Default for Signal2<'a> {
    fn default() -> Self {
        Signal2 {
            layout: SideBySide::default()
                .add_title(TITLE)
                .add_paragraph(leafy! {
                    "With an oscilloscope, we can measure the **amplitude** of a signal at a given *point in time* (***time-domain***), \
                    through the visualisation of a ***waveform***."
                })   
                .add_widget(0, Box::new(Waveform::new()))
                .add_paragraph(indoc! {
                    "• An analog signal can already be processed as it is, with ***analog effects***: \
                    *tape delay*, *distortion*, *chorus*, *reverberation (spring, plate)*, etc."
                })
                // TODO: add spectrogram animation?             
                .add_paragraph(leafy! {
                    "On the other hand, it is difficult to extract precise information about ***frequency*** and ***spectrum***. \
                    For this purpose, it's far more efficient to switch to the ***frequency domain***, which requires the analog signal to \
                    be turned into a ***digital signal***..."
                })               
                .add_widget(2, Box::new(SpectrumCanvas::default())) 
                ,            
        }
    }
}

