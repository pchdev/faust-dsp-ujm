
use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    buffer::Buffer, 
    layout::{
        Constraint, 
        Flex, 
        Layout
    }, 
    prelude::Rect, 
    style::{
        Style, 
        Stylize
    }, 
    widgets::{
        Block, 
        BorderType, 
        Borders, 
        HighlightSpacing, 
        List, 
        ListItem, 
        ListState, 
        Paragraph, 
        StatefulWidget, 
        Widget, WidgetRef, 
        Wrap
    }
};

use indoc::indoc;

use ratatui_macros::{
    horizontal, 
    vertical
};

use crate::{
    screens::Screen, 
    widgets::{
        particles::Particles, 
        ripple::Ripple, 
        waveform::Waveform
    }
};

const SOUND: &'static str = indoc!{"
┏━┓┏━┓╻ ╻┏┓╻╺┳┓
┗━┓┃ ┃┃ ┃┃┗┫ ┃┃
┗━┛┗━┛┗━┛╹ ╹╺┻┛
"};

#[derive(Debug, Default)]
enum Animation {
    #[default]
    None,
    Ripple(Ripple),
    Particles(Particles),
    Waveform(Waveform)
}

#[derive(Debug)]
enum Content<'a> {
    Paragraph(Paragraph<'a>),
    List(Vec<String>, ListState)
}

#[derive(Debug)]
pub struct Sound<'a> {
         rhs: Animation,
      select: usize,
    contents: Vec<Content<'a>>
}

impl<'a> Sound<'a> {
    fn add_paragraph(&mut self, txt: &'static str) {
        self.contents.push(
            Content::Paragraph(
                Paragraph::new(
                    tui_markdown::from_str(txt)
                ).wrap(Wrap {trim: true})
            )
        );
    }
    fn add_list(&mut self, list: Vec<&'static str>) {
        let mut items = vec![];
        for i in list {
            items.push(String::from(i));
        }
        self.contents.push(
            Content::List(items, ListState::default())
        );
    }
}

impl<'a> Default for Sound<'a> {
    fn default() -> Self {
        let mut s = Sound {
            rhs: Animation::default(),
            select: 0,
            contents: vec![]
        };
        s.add_paragraph(indoc! { 
            "• Sound is a ***pressure wave*** that propagates \
            through a **medium** (*gas*, *liquid* or *solid*).
            "
        });
        s.add_paragraph(indoc! { 
            "• Propagation is carried by the **periodic oscillation** (*vibration*) of \
            the medium's particles around their point of origin.
            "
        });
        s.add_paragraph(indoc! { 
            "• We **measure** sound and its properties by analyzing the periodic oscillation of \
            an object (usually the *membrane* of a *microphone*):            
            "
        });
        s.add_list(vec![
              "- **Speed**: ~343 m/s in air",
              "- **Amplitude**: in *Pascals* (***Pa***) or *Decibels* (***dB***)",
              "- **Period**: the time between two oscillations",
              "- **Wavelength**: the distance between two oscillations",
              "- **Frequency**: cycles/sec., in *Hertz* (***Hz***, ***kHz***, ***MHz***)",
              "- **Spectrum**: or *Timbre*"
        ]);
        // • Speed:
        //  - Air: ~340 m/s
        //  - Water: ~1,480 m/s
        //  - Steel: ~5,960 m/s
        //  - Solid atomic hydrogen: ~36,000 m/s
        //  - Speed of light: 299,792,458 m/s

        return s;
    }
}

impl<'a> WidgetRef for Sound<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        let [lhl, lhr] = horizontal![==50%, ==50%]
            .flex(Flex::Center)
            .areas(area);     
        let lhlv = vertical![==5%, ==20%, ==75%]
            .flex(Flex::Center)
            .horizontal_margin(5)
            .split(lhl)
        ;
        Block::bordered()
            .borders(Borders::LEFT)
            .border_type(BorderType::Plain)
            .render(lhr, buf)
        ;
        Paragraph::new(SOUND)
            .centered()
            .render(lhlv[1], buf)
        ;
        // Compute layout constraints:
        let mut constraints = vec![];
        for content in self.contents.iter() {
            match content {
                Content::Paragraph(ph) => {
                    let lc = ph.line_count(lhlv[2].width);
                    constraints.push(
                        Constraint::Length(lc.try_into().unwrap())
                    );
                }
                Content::List(list, ..) => {
                    constraints.push(
                        Constraint::Length(list.len() as u16)
                    )
                }
            }
        }
        // Build the layout:
        let lp = Layout::vertical(constraints)
            .spacing(1)
            .split(lhlv[2])
        ;
        // Render everything:
        for (n, content) in self.contents.iter().enumerate() {
            match content {
                Content::Paragraph(ph) => {
                    if self.select == n {
                        // If paragraph is selected:
                        let p = ph.clone().black().on_gray();
                        p.render(lp[n], buf);
                    } else {
                        ph.render(lp[n], buf);
                    }                    
                }
                Content::List(svec, state) => {
                    let mut ivec = vec![];
                    let mut s = state.clone();
                    let select = self.select as isize - n as isize;
                    if select < 0 {
                        s.select(None);
                    } else {
                        s.select(Some(select as usize));
                    }
                    for str in svec {
                        ivec.push(ListItem::new(
                            tui_markdown::from_str(
                                str.as_str()
                            )
                        ));
                    }   
                    let l = List::new(ivec)
                        .highlight_symbol("> ")
                        .highlight_style(Style::new().black().on_gray())
                        .highlight_spacing(HighlightSpacing::Always)
                    ;
                    StatefulWidget::render(l, lp[n], buf, &mut s);
                }
            }
        }
        match &self.rhs {
            Animation::Ripple(r) => {
                r.render_ref(lhr, buf);
            }
            Animation::Particles(p) => {
                p.render_ref(lhr, buf);
            }
            Animation::Waveform(p) => {
                p.render_ref(lhr, buf);
            }
            _ => ()
        }
    }
}

impl<'a> Screen for Sound<'a> {
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Down => {
                self.select += 1;
            }
            KeyCode::Up => {
                if self.select > 0 {
                   self.select -= 1;
                }
            }
            KeyCode::Enter => {
                match self.select {
                    0  => {
                        self.rhs = Animation::Ripple(
                            Ripple::new(200)
                        );
                    }
                    1 => {
                        self.rhs = Animation::Particles(
                            Particles::new(400)
                        )
                    }
                    2 => {
                        self.rhs = Animation::Waveform(
                            Waveform::new(100, 25)
                        );
                    }
                    3  => {
                        self.rhs = Animation::Ripple(
                            Ripple::new(200)
                        );
                    }
                    4 => {
                        self.rhs = Animation::Waveform(
                            Waveform::new(100, 25)
                        );
                    }
                    5  => {
                        self.rhs = Animation::Ripple(
                            Ripple::new(200)
                        );
                    }
                    _ => ()
                }

            }
            _ => ()
        }
    }
    fn on_tick(&mut self, t: usize) {
        match &mut self.rhs {
            Animation::Ripple(r) => {
                r.on_tick(t);
            }
            Animation::Particles(p) => {
                p.on_tick(t);
            }
            Animation::Waveform(w) => {
                w.on_tick(t);
            }
            _ => ()
        }
    }
}