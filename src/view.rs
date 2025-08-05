use crossterm::event::Event;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use std::fmt::Debug;

pub trait View: Debug {
    fn focus_gained(&mut self);
    fn focus_lost(&mut self);

    fn handle_event(&mut self, event: &Event);

    // fn modes(&self) -> Vec<&'static str>;
    fn mode_long(&self) -> &str;
    fn mode(&self) -> &str;

    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized;
}
