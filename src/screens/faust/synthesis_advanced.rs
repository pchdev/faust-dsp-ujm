
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{WidgetRef}
};

use indoc::indoc;

use crate::{
    screens::{leafy, Screen, SideBySide}, 
    widgets::{faustblock::FaustWidget}
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━┓╻ ╻┏┓╻╺┳╸╻ ╻┏━╸┏━┓╻┏━┓
┗━┓┗┳┛┃┗┫ ┃ ┣━┫┣╸ ┗━┓┃┗━┓
┗━┛ ╹ ╹ ╹ ╹ ╹ ╹┗━╸┗━┛╹┗━┛
"};

macro_rules! example {
    ($path:literal) => {
        Box::new(
            FaustWidget::new(
                include_str!(
                    concat!("../../../examples/synthesis_advanced/", $path)
                )
            )
        )        
    };
}

pub struct FaustSynthesisAdvanced<'a> {
    screen: SideBySide<'a>,
}

impl<'a> WidgetRef for FaustSynthesisAdvanced<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        self.screen.render_ref(area, buf);
    }
}

impl<'a> Screen for FaustSynthesisAdvanced<'a> {
    fn title(&self) -> &'static str {
        "Faust: advanced synthesis"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.screen.on_key_event(k);
    }
    fn on_tick(&mut self, t: usize) {
        self.screen.on_tick(t);
    }
}

impl<'a> Default for FaustSynthesisAdvanced<'a> {
    fn default() -> Self {
        FaustSynthesisAdvanced {
            screen: SideBySide::default()
                .add_title(TITLE)
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "Faust has one final composition operator: the ***recursive operator*** ('**~**') \
                     which allows to ***connect two signals recursively*** (with ***a delay of one sample***)."
                })
                .add_widget(0, example!("recursive.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "The recursive operator is typically used to implement **counters**, and/or ***to represent time***. \
                    With it, we can count for instance *up to 44100 or 48000 samples* to ***represent one second of \
                    audio time***."
                })
                .add_widget(1, example!("counter_samples.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "When implementing ***time-related functions***, it can prove very useful to \
                    ***monitor values from the GUI***."
                })
                .add_widget(2, example!("monitoring_values.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "We can use it here to ***switch our synth's waveform automatically every second.***"
                })
                .add_widget(3, example!("waveform_switch_counter.dsp"))
                // ---------------------------------------------------------------------------------------
                ,
        }
    }
}

