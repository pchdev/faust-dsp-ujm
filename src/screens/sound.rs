
use crossterm::event::{KeyCode, KeyEvent};
use num_derive::FromPrimitive;
use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{
        StatefulWidgetRef, Widget, WidgetRef
    }
};

use crate::{
    screens::Screen, 
    widgets::ripple::{Ripple}
};

#[derive(Debug, Default)]
enum State {
    #[default]
    Start,
    Ripple(Ripple),
    Displacement,
    Waveform
}

impl TryFrom<usize> for State {
    type Error = ();
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Start),
            1 => Ok(Self::Ripple(Ripple::default())),
            2 => Ok(Self::Displacement),
            3 => Ok(Self::Waveform),
            _ => Err(())
        }
    }
}

#[derive(Debug, Default)]
pub struct Sound {
    state: State
}

impl WidgetRef for Sound {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {        
        match &self.state {
            State::Ripple(r) => {
                let mut running = true;
                r.render_ref(area, buf, &mut running);
            }
            _ => ()
        }
    }
}

impl Screen for Sound {
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Enter => {
                self.state = State::Ripple(Ripple::default());
            }
            _ => ()
        }
    }
    fn on_tick(&mut self, t: usize) {
        match &mut self.state {
            State::Ripple(r) => {
                r.tick = t;
            }
            _ => ()
        }
    }
}