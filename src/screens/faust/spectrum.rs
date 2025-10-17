
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
┏━┓┏━┓┏━╸┏━╸╺┳╸┏━┓╻ ╻┏┳┓
┗━┓┣━┛┣╸ ┃   ┃ ┣┳┛┃ ┃┃┃┃
┗━┛╹  ┗━╸┗━╸ ╹ ╹┗╸┗━┛╹ ╹
"};

macro_rules! example {
    ($path:literal) => {
        Box::new(
            FaustWidget::new(
                include_str!(
                    concat!("../../../examples/basics2/", $path)
                )
            )
        )        
    };
}

pub struct FaustSpectrum<'a> {
    screen: SideBySide<'a>,
}

impl<'a> WidgetRef for FaustSpectrum<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        self.screen.render_ref(area, buf);
    }
}

impl<'a> Screen for FaustSpectrum<'a> {
    fn title(&self) -> &'static str {
        "Faust: spectral considerations"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.screen.on_key_event(k);
    }
    fn on_tick(&mut self, t: usize) {
        self.screen.on_tick(t);
    }
}

impl<'a> Default for FaustSpectrum<'a> {
    fn default() -> Self {
        FaustSpectrum {
            screen: SideBySide::default()
                .add_title(TITLE)
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "In Faust, *connecting DSP functions* can be done using the ***sequential operator*** '**:**'."
                })
                .add_widget(0, example!("sequential.dsp"))
                // ---------------------------------------------------------------------------------------
                ,
        }
    }
}

