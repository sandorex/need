use ratatui::crossterm::event::Event;
use std::io;

pub trait InteractiveWidget {
    type AppState;

    /// Called when an event is triggered but not handled
    // fn event(&mut self, app: &mut Self::AppState, event: io::Result<Event>) {}
    fn event(&mut self, app: &mut Self::AppState, event: io::Result<Event>) -> io::Result<()>;

    // fn key_event(&mut self, event: KeyEvent);
}
