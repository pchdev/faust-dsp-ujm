
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use ratatui::{
    buffer::Buffer, 
    layout::{
        Flex, 
    }, 
    prelude::Rect, 
    widgets::{
        Block, 
        BorderType, 
        Borders, 
        ListState, 
        Paragraph, 
        Widget, WidgetRef, 
    }
};

use indoc::indoc;

use ratatui_macros::{
    horizontal, 
};

use crate::{
    screens::{ContentArea, Screen, leafy}, 
    widgets::{faustblock::FaustWidget}
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━╸┏┓╻╺┳╸┏━╸┏━┓   ┏━╸┏━┓╻ ╻┏━┓╺┳╸
┣╸ ┃┗┫ ┃ ┣╸ ┣┳┛   ┣╸ ┣━┫┃ ┃┗━┓ ┃ 
┗━╸╹ ╹ ╹ ┗━╸╹┗╸   ╹  ╹ ╹┗━┛┗━┛ ╹ 
"};

const VIRTUAL_ANALOG: &'static str = indoc! {"
import(\"stdfaust.lib\");

// Sliders:
oscFreq = hslider(\"oscFreq [knob:1]\",80,50,500,0.01);
lfoFreq = hslider(\"lfoFreq [knob:2]\",1,0.01,8,0.01);
lfoRange = hslider(\"lfoRange [knob:3]\",1000,10,5000,0.01) : si.smoo;
noiseGain = hslider(\"noiseGain [slider:7]\",0,0,1,0.01) <: _*_;
masterVol = hslider(\"masterVol [slider:8]\",0.8,0,1,0.01) <: _*_;
panning = hslider(\"pan [knob:4]\",0.5,0,1,0.01)  : si.smoo;

// Buttons:
activateNoise = button(\"activateNoise [switch:6]\");
killSwitch = 1-button(\"killSwitch [switch:5]\");

LFO = os.lf_triangle(lfoFreq) * 0.5 + 0.5;

process = os.oscrc(440) 
        * 0.25 
        * killSwitch 
        * os.sawtooth(oscFreq) 
        + no.noise * noiseGain
        * activateNoise : fi.resonlp(
            LFO * lfoRange + 50,
            5,
            1)
        * masterVol <: _ * (1-panning), _ * panning;
"};

#[derive(Default)]
enum Animation {
    #[default]
    None,
    CodeBlock(FaustWidget)
}

#[derive(Debug)]
enum Content<'a> {
    Paragraph(Paragraph<'a>),
    List(Vec<String>, ListState)
}

pub struct Faust<'a> {
      lhs: ContentArea<'a>, 
      rhs: Animation,
    rhs_focus: bool,
     rhs_init: bool,
}

impl<'a> Default for Faust<'a> {
    fn default() -> Self {
        Faust {
            lhs: ContentArea::default()
                .add_title(TITLE)
                .add_paragraph(indoc! {
                    "• **Faust** (*Functional Audio Stream*) is a programming language \
                    specifically made for ***audio DSP and synthesis***. \
                    It was created by Yann Orlarey, Dominique Fober & Stéphane Letz at GRAME in 2002."
                })
                .add_list(vec! [
                    "• *Functional* paradigm",
                    "• Declarative, math-like syntax",
                ])
                .add_paragraph(indoc! {
                    "• Compiles to **many targets** and **architectures**:",
                })
                .add_list(vec![
                    "• **Plugins**: VST, CLAP, AudioUnit",
                    "• **Software**: Max, Pd, SuperCollider, Csound",
                    "• **OS**: Linux, macOS, Windows, Android, iOS",
                    "• **Embedded**: Bela, Teensy, Daisy, ESP32, FPGA",
                    "• **Code**: C/C++, Rust, WASM, Java..."
                ])
                .add_paragraph(leafy! {
                    "Tutorials & documentation available at: ***https://faust.grame.fr***"
                })
                .add_paragraph(leafy! {
                    "Dedicated online IDE: ***https://faustide.grame.fr***"
                })
                ,
            rhs: Animation::None,            
            rhs_focus: false,
            rhs_init: false,
        }
    }
}

impl<'a> WidgetRef for Faust<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        let [lhl, lhr] = horizontal![==50%, ==50%]
            .flex(Flex::Center)
            .areas(area)
        ;
        Block::bordered()
            .borders(Borders::LEFT)
            .border_type(BorderType::Plain)
            .render(lhr, buf)
        ;
        self.lhs.render_ref(lhl, buf);
        match &self.rhs {
            Animation::CodeBlock(f) => {
                f.render_ref(lhr, buf);
            }
            _ => ()
        }
    }
}

impl<'a> Screen for Faust<'a> {
    fn title(&self) -> &'static str {
        "Enter Faust!"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Right => {
                if k.modifiers.contains(KeyModifiers::CONTROL)
                && k.modifiers.contains(KeyModifiers::SHIFT) {
                    if self.rhs_init == false {
                        self.rhs = Animation::CodeBlock(
                            FaustWidget::new(VIRTUAL_ANALOG)
                        );
                    }
                    self.rhs_focus = true;
                } else {
                    match &mut self.rhs {
                        Animation::None => {
                            self.lhs.on_key_event(k);
                        }
                        Animation::CodeBlock(cb) => {
                            if self.rhs_focus {
                                cb.on_key_event(k);
                            } else {
                                self.lhs.on_key_event(k);
                            }
                        }
                    }
                }
            }
            KeyCode::Left => {
                if k.modifiers.contains(KeyModifiers::CONTROL)
                && k.modifiers.contains(KeyModifiers::SHIFT) {
                    self.rhs_focus = false;
                } else {
                    // match &mut self.rhs {
                        
                    // }
                    // if self.rhs_focus {
                    //     cb.on_key_event(k);
                    // } else {
                    //     self.lhs.on_key_event(k);
                    // }                            
                }
            }
            _ => {
                
            }
        }
        match &mut self.rhs {
            Animation::None => {
                match k.code {
                    // Ctrl+Right: switch to right panel
                    KeyCode::Right => {
                        if k.modifiers.contains(KeyModifiers::CONTROL) 
                        && k.modifiers.contains(KeyModifiers::SHIFT) {
                            if self.rhs_init == false {
                                self.rhs = Animation::CodeBlock(
                                    FaustWidget::default()
                                );
                            }
                            self.rhs_focus = true;
                        }
                    }
                    _ => {
                        self.lhs.on_key_event(k);
                    }
                }
            }
            Animation::CodeBlock(cb) => {
                match k.code {
                    KeyCode::Left => {
                        if k.modifiers.contains(KeyModifiers::CONTROL) 
                        && k.modifiers.contains(KeyModifiers::SHIFT) {
                            self.rhs_focus = false
                        } else {
                            if self.rhs_focus {
                                cb.on_key_event(k);
                            } else {
                                self.lhs.on_key_event(k);
                            }                            
                        }
                    }
                    _ => {
                        if self.rhs_focus {
                            cb.on_key_event(k);
                        } else {
                            self.lhs.on_key_event(k);
                        }
                        
                    }
                }
            }
        }
    }

    fn on_tick(&mut self, t: usize) {
        match &mut self.rhs {
            _ => ()
        }
    }
}