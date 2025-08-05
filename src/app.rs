use std::{io, ops::DerefMut, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
};

use crate::{buffer::Buffer, editor::Editor, keymap::Keymap};

#[derive(Debug)]
pub struct App {
    // exit: bool,
    editor: Editor,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // exit: false,
            editor: Editor::new(),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.editor.buffers[0] = Buffer::from_raw(
            r#"Hello there

This is the third line

The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line
The fifth line


I dont relaly know what line this is"#,
        );

        while !self.editor.should_quit {
            terminal.draw(|frame| self.draw(frame))?;

            self.editor.update();

            // handle events only if there are any
            if ratatui::crossterm::event::poll(Duration::from_millis(100))? {
                let event = ratatui::crossterm::event::read()?;
                self.editor.handle_crossterm_event(&event);
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(&self.editor, frame.area());
    }
}
