use std::{io, ops::DerefMut, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
};

use crate::{actions::Action, buffer::Buffer, editor::Editor, keymap::Keymap, util::Key};

#[derive(Debug)]
pub struct App {
    editor: Editor,
}

impl Default for App {
    fn default() -> Self {
        let mut editor = Editor::new();

        editor.keymap.add(
            &[Key {
                code: KeyCode::Esc,
                ..Default::default()
            }],
            &Action::Cancel,
        );
        editor.keymap.add(
            &[Key {
                code: KeyCode::Down,
                ..Default::default()
            }],
            &Action::CursorMoveRel { x: 0, y: 1 },
        );
        editor.keymap.add(
            &[Key {
                code: KeyCode::Up,
                ..Default::default()
            }],
            &Action::CursorMoveRel { x: 0, y: -1 },
        );
        editor.keymap.add(
            &[Key {
                code: KeyCode::Left,
                ..Default::default()
            }],
            &Action::CursorMoveRel { x: -1, y: 0 },
        );
        editor.keymap.add(
            &[Key {
                code: KeyCode::Right,
                ..Default::default()
            }],
            &Action::CursorMoveRel { x: 1, y: 0 },
        );

        editor.keymap.add(
            &[
                Key {
                    code: KeyCode::Char('d'),
                    ..Default::default()
                },
                Key {
                    code: KeyCode::Char('d'),
                    ..Default::default()
                },
            ],
            &Action::CursorMoveRel { x: 1, y: 0 },
        );

        editor.keymap.add(
            &[Key {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::CONTROL,
            }],
            &Action::BufferPrev,
        );
        editor.keymap.add(
            &[Key {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }],
            &Action::BufferNext,
        );

        editor.buffers.push_front(Buffer::from_raw(
            r#"aghahwahg

xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
xwxwxwxwxwdfwf
"#,
        ));

        editor.buffers.push_front(Buffer::from_raw("third one"));

        Self { editor }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.editor.should_quit {
            terminal.draw(|frame| self.draw(frame))?;

            // handle events only if there are any
            if ratatui::crossterm::event::poll(Duration::from_millis(100))? {
                let event = ratatui::crossterm::event::read()?;
                self.editor.handle_event(&event);
            }

            // update the editor
            self.editor.update();
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(&self.editor, frame.area());
    }
}
