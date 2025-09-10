
use crossterm::event::{KeyCode, KeyEvent};
use num_derive::FromPrimitive;
use ratatui::{
    buffer::Buffer, layout::Flex, prelude::Rect, widgets::{
        Paragraph, StatefulWidgetRef, Widget, WidgetRef
    }
};
use ratatui_macros::{horizontal, vertical};

use crate::{
    screens::Screen, 
    widgets::ripple::{Ripple}
};

use indoc::indoc;

const SOUND: &'static str = indoc!{"
┏━┓┏━┓╻ ╻┏┓╻╺┳┓
┗━┓┃ ┃┃ ┃┃┗┫ ┃┃
┗━┛┗━┛┗━┛╹ ╹╺┻┛
"};

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
        let [lhl, lhr] = horizontal![==50%, ==50%]
            .flex(Flex::Center)
            .areas(area);     
        let lhlv = vertical![==5%, ==20%, ==75%]
            .flex(Flex::Center)
            .split(lhl)
        ;
        let title = Paragraph::new(SOUND)
            .centered();
        title.render(lhlv[1], buf);
        match &self.state {
            State::Ripple(r) => {
                let mut running = true;
                r.render_ref(lhr, buf, &mut running);
            }
            _ => ()
        }
    }
}

impl Screen for Sound {
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Enter => {
                self.state = State::Ripple(
                    Ripple { 
                        tick: 0,
                        frequency: 1,
                        amplitude: 200
                    }
                );
            }
            _ => ()
        }
    }
    fn on_tick(&mut self, t: usize) {
        match &mut self.state {
            State::Ripple(r) => {
                r.on_tick(t);
            }
            _ => ()
        }
    }
}