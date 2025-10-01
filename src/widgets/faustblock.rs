use crossterm::event::KeyEvent;
use edtui::{
    EditorEventHandler, 
    EditorState, 
    EditorStatusLine, 
    EditorTheme, 
    EditorView, 
    Lines, 
    SyntaxHighlighter
};
use ratatui::{
    buffer::Buffer, 
    layout::{Alignment, Flex, Rect}, 
    style::{Style, Stylize}, 
    text::Line, 
    widgets::{Block, BorderType, Borders, Tabs, Widget, WidgetRef}
};

use ratatui_macros::vertical;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Clone)]
pub struct FaustEditor {
    state: EditorState,
}

impl Default for FaustEditor {
    fn default() -> Self {
        FaustEditor { 
            state: EditorState::new(
                Lines::from(
                    "process = +;"
                )
            ),
         }
    }
}

impl FaustEditor {
    pub(crate) fn on_key_event(&mut self, k: KeyEvent) {
        let mut event_handler = EditorEventHandler::default();
        event_handler.on_key_event(k, &mut self.state);
    }
    pub(crate) fn on_tick(&mut self, tick: usize) {

    }
}

impl WidgetRef for FaustEditor {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut state = self.state.clone();
        EditorView::new(&mut state)
            .theme(
                EditorTheme::default()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title(" faust ".bold())
                            .title_alignment(Alignment::Center)
                    )
                    .base(Style::default().black().on_white())
                    .cursor_style(Style::default().black().on_gray())
                    .selection_style(Style::default().gray().on_dark_gray())
                    .status_line(
                        EditorStatusLine::default()
                            .style_text(Style::default().black().on_white())
                            .style_line(Style::default().black().on_white())
                            .align_left(true)
                    )
            )
            .tab_width(4)
            .syntax_highlighter(Some(SyntaxHighlighter::new("gruvbox-light", "rs")))
            .wrap(true)
            .render(area, buf)
        ;
    }
}


#[derive(Clone)]
#[derive(Display, FromRepr, EnumIter)]
enum TabSelect {
    // -------------------------------
    #[strum(to_string = "editor")]
    Editor(FaustEditor),
    // -------------------------------
    #[strum(to_string = "control")]
    Control,
    // -------------------------------
    #[strum(to_string = "graph")]
    Graph,
    // -------------------------------
    #[strum(to_string = "wave")]
    Waveform,
    // -------------------------------
    #[strum(to_string = "spectrum")]
    Spectrum
}

impl TabSelect {
    fn title(self) -> Line<'static> {
        format!(" {self} ")
            .black().on_white()
            .into()
    }
    fn previous(self) -> Self {
        let idx = 0usize;
        let prv = idx.saturating_sub(1);
        Self::from_repr(prv).unwrap_or(self)
    }
    fn next(self) -> Self {
        let idx = 0usize;
        let nxt = idx.saturating_add(1);
        Self::from_repr(nxt).unwrap_or(self)
    }
}

pub(crate) struct FaustWidget {
    select: TabSelect
}

impl Default for FaustWidget {
    fn default() -> Self {
        FaustWidget {
            select: TabSelect::Editor(
                FaustEditor::default()
            )
        }
    }
}

impl FaustWidget {
    /// Propagate key event to underlying widget:
    pub(crate) fn on_key_event(&mut self, k: KeyEvent) {
        match &mut self.select {
            TabSelect::Editor(e) => {
                e.on_key_event(k);
            }
            _ => ()
        }
    }
}

impl WidgetRef for FaustWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let [t0, t1] = vertical![==2%, ==98%]
            .flex(Flex::Center)
            .areas(area)
        ;
        Tabs::new(
                TabSelect::iter().map(TabSelect::title)
            )
            .highlight_style(Style::default().white().on_black())
            .select(0usize)
            .padding("", "")
            .divider(" ")
            .render(t0, buf)
        ;
        match &self.select {
            TabSelect::Editor(e) => {
                e.render_ref(t1, buf);
            }
            _ => ()
        }
    }
}




