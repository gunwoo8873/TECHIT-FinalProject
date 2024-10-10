use std::io::{Result};
use crossterm::event::Event;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    prelude::*,
    widgets::*,
    layout::*,
    style::Stylize,
    DefaultTerminal,
    Frame,
    Terminal,
};

//// Public Modules Path
pub mod ui;
pub mod api;

//// Terminal State
pub struct State {
    Stop: String,
    Parsue: String,
    Run: String,
}

pub enum SelectMenu {
    Pod,
    PodSpec,
    PodStatus,
    Service,
    Settings,
    Exit,
}

pub enum KeyPress {
    up,
    down,
    enter,
    space,
    esc,
}

pub trait Widget {
    fn render_ref(self, area: Rect, buf: &mut Buffer);
}

pub trait StatefulWidget {
    type State;
    fn render_ref(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}