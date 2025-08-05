use crate::{buffer::Buffer, view::View};
use ratatui::widgets::Widget;

// NOTE simple single buffer window, leave tiling and the rest for later
#[derive(Debug)]
pub struct Window {
    pub buffer: Box<dyn Buffer>,
}

impl View for Window {
    fn focus_gained(&mut self) {}
    fn focus_lost(&mut self) {}

    fn handle_event(&mut self, event: &crossterm::event::Event) {}

    fn mode_long(&self) -> &str {
        "edit"
    }

    fn mode(&self) -> &str {
        self.mode_long()
    }

    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        <&Self as Widget>::render(self, area, buf);
    }
}

impl Widget for &Window {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // <&self.buffer>render(area, buf);
    }
}
