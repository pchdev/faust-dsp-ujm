
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{WidgetRef}
};

use indoc::indoc;

use crate::{
    screens::{leafy, Screen, SideBySide}, 
    widgets::faustblock::FaustWidget
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏┓ ┏━┓┏━┓╻┏━╸┏━┓
┣┻┓┣━┫┗━┓┃┃  ┗━┓
┗━┛╹ ╹┗━┛╹┗━╸┗━┛
"};

macro_rules! example {
    ($path:literal) => {
        Box::new(
            FaustWidget::new(
                include_str!(
                    concat!("../../../examples/basics/", $path)
                )
            )
        )        
    };
}


pub struct FaustBasics<'a> {
    screen: SideBySide<'a>,
}

impl<'a> WidgetRef for FaustBasics<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        self.screen.render_ref(area, buf);
    }
}

impl<'a> Screen for FaustBasics<'a> {
    fn title(&self) -> &'static str {
        "Faust: basics"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.screen.on_key_event(k);
    }
    fn on_tick(&mut self, t: usize) {
        self.screen.on_tick(t);
    }
}

impl<'a> Default for FaustBasics<'a> {
    fn default() -> Self {
        FaustBasics {
            screen: SideBySide::default()
                .add_title(TITLE)
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "Basic program: ***import*** and ***process*** statements."
                })
                .add_widget(0, example!("comments.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "Use a ***Sinewave Oscillator*** from the library."
                })
                .add_widget(1, example!("libraries.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "In DSP, we represent an audio signal with ***floating-point*** (decimal) numbers \
                    and *scale* its values ***between -1.0 and +1.0***."
                })
                .add_widget(2, example!("amplitude.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "***Mixing*** two signals can be done using the '**+**' operator."
                })
                .add_widget(3, example!("mixing.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "When multiplying two signals together, we can already make a simple \
                    DSP effect: ***Amplitude Modulation*** (or ***Ring Modulation***)."
                })
                .add_widget(4, example!("ampmod.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "Faust can process **multiple channels**, we can use the '**,**' symbol \
                    (***Parallel Operator***) to put signals *in parallel*."
                })
                .add_widget(5, example!("parallel.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "We can declare **custom variables** (***functions***)."
                })
                .add_widget(6, example!("custom-functions.dsp"))
                // ---------------------------------------------------------------------------------------
                ,
        }
    }
}

