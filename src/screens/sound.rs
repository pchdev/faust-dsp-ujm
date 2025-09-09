
use crossterm::event::KeyEvent;
use num_derive::FromPrimitive;
use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{
        StatefulWidgetRef, 
        WidgetRef
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

impl StatefulWidgetRef for Sound {
    type State = usize;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut usize) {        
        match State::try_from(*state).unwrap() {
            State::Ripple(f) => {
                f.render_ref(area, buf, state);
            }
            _ => ()
        }
        *state += 1;
    }
}

impl Screen for Sound {
    fn on_key_event(&mut self, k: KeyEvent) {
        
    }
}