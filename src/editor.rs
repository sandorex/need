use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

use crate::buffer::Buffer;
use crate::keymap::Keymap;
use crate::util::{Key, KeySequence};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{crossterm::event::Event, layout::Rect, style::Style, widgets::Widget};

/// Contains whole state of the editor
#[derive(Debug)]
pub struct Editor {
    /// Should the editor close (handled by the app)
    pub should_quit: bool,

    // TODO buffers should be accessed using a function, and advanced like so editor.set_buf, editor.get_buf(None) -> current
    /// Current focused buffer
    pub buffer_index: usize,

    /// All buffers loaded
    pub buffers: Vec<Buffer>,

    /// Millis when last keypress happen
    last_keypress: Instant,

    /// Keys pressed, used for chorded keys
    pub key_buffer: Vec<Key>,

    /// Current keymap
    pub keymap: Rc<Keymap>,

    /// All keymaps defined
    pub keymaps: HashMap<String, Rc<Keymap>>,
}

impl<'a> Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            buffer_index: 0,
            // TODO this could use a scratchpad buffer thingy
            // create empty buffer by default
            buffers: vec![Buffer::default()],
            last_keypress: Instant::now(),
            key_buffer: vec![],
            keymap: Rc::new(Keymap::new("NORMAL".to_string())),
            keymaps: HashMap::new(),
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn handle_crossterm_event(&mut self, event: &Event) {
        // remember time and then use that to detect if no key is pressed for long enough for chorded keys
        match event {
            Event::Key(e) if e.code == KeyCode::Char('q') => {
                self.should_quit = true;
            }
            Event::Key(ev) => {
                if ev.code == KeyCode::Enter {
                    self.key_buffer.clear();
                    return;
                }

                if ev.kind == KeyEventKind::Press {
                    self.key_buffer.push(ev.clone().into());
                    self.last_keypress = Instant::now();

                    // TODO this is really ugly, could also be its own function
                    // if the keymap has no more keys defined then just execute the action
                    if let Some(entry) = self.keymap.get(&self.key_buffer) {
                        if entry.get_map().is_none() {
                            if let Some(action) = entry.get_action().cloned() {
                                self.key_buffer.clear();
                                action.execute(self);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        // execute action if defined with current key buffer after some delay
        if !self.key_buffer.is_empty() {
            let elapsed = self.last_keypress.elapsed().as_millis();
            if elapsed >= 300 {
                if let Some(entry) = self.keymap.get(&self.key_buffer) {
                    if let Some(action) = entry.get_action().cloned() {
                        self.key_buffer.clear();
                        action.execute(self);
                    }
                }
            }
        }
    }
}

impl Widget for &Editor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        use ratatui::layout::{Constraint, Direction, Layout};
        use ratatui::style::Stylize;
        use ratatui::text::Span;

        // always show statusbar on the bottom
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Max(1)])
            .split(area);

        let editor_area = layout[0];

        let layout_statusbar = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50); 2])
            .split(layout[1]);

        Span::from(self.keymap.name[..4].to_uppercase())
            .into_left_aligned_line()
            .render(layout_statusbar[0], buf);

        if let Some(buffer) = self.buffers.get(self.buffer_index) {
            Span::from(format!(
                "{}  {}:{}",
                Into::<KeySequence>::into(&self.key_buffer),
                buffer.cursor.y,
                buffer.cursor.x
            ))
            .into_right_aligned_line()
            .render(layout_statusbar[1], buf);

            buffer.render(editor_area, buf);
        }

        // Span::from(self.mode()[..4].to_uppercase())
        //     .into_left_aligned_line()
        //     .render(layout_statusbar[0], buf);

        // Span::from(format!("{}:{}", self.view.cursor.y, self.view.cursor.x))
        //     .into_right_aligned_line()
        //     .render(layout_statusbar[1], buf);

        // self.view.render(editor_area, buf);
        // Paragraph::new(self.state.text.as_str()).render(editor_area, buf);

        // buf.set_style(
        //     Rect::new(
        //         editor_area.x + self.view.cursor.x,
        //         editor_area.y + self.view.cursor.y,
        //         1,
        //         1,
        //     ),
        //     Style::default().reversed(),
        // );
    }
}
