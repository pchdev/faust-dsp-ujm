use crossterm::event::KeyEvent;
use ratatui::widgets::WidgetRef;

pub mod ripple;
pub mod particles;
pub mod waveform;
pub mod faustblock;
pub mod control;
pub mod sampling;
pub mod aliasing;
pub mod quantization;
pub mod spectrogram;
pub mod popup_menu;
pub mod db;

pub trait InteractiveWidget : WidgetRef {
    fn on_key_event(&mut self, k: KeyEvent) {}
    fn on_tick(&mut self, t: usize) {}
}