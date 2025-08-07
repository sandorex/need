use std::collections::VecDeque;
use std::time::Instant;

use crate::actions::Action;
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

    pub buffer: Buffer,

    /// All buffers loaded
    pub buffers: VecDeque<Buffer>,

    /// Millis when last keypress happen
    last_keypress: Instant,

    /// Keys pressed, used for chorded keys
    pub key_buffer: Vec<Key>,

    /// Current keymap
    pub keymap: Keymap,

    /// Stores loaded keymaps
    pub keymaps: Vec<Keymap>,
}

impl<'a> Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            buffer: Buffer::default(),
            buffers: VecDeque::new(),
            last_keypress: Instant::now(),
            key_buffer: vec![],
            keymap: Keymap::new(crate::MODE_NORM.into()),
            keymaps: vec![],
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Key(e) if e.code == KeyCode::Char('q') => {
                self.should_quit = true;
            }
            Event::Key(ev) if ev.kind == KeyEventKind::Press => {
                self.key_buffer.push(ev.clone().into());
                self.last_keypress = Instant::now();

                // see if the current key sequence is mapped
                if let Some(entry) = self.keymap.get(&self.key_buffer) {
                    if !entry.has_children() {
                        // the entry has no cihldren, just run it instantly
                        self.execute_action(entry.action.clone());
                        self.key_buffer.clear();
                    }
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        const KEY_SEQ_TIMEOUT: u128 = 300;

        // wait for some time until the key sequence is executed so user can enter more keys
        if !self.key_buffer.is_empty()
            && self.last_keypress.elapsed().as_millis() >= KEY_SEQ_TIMEOUT
        {
            // check if current sequence is mapped
            if let Some(entry) = self.keymap.get(&self.key_buffer) {
                if entry.action == Action::None {
                    if !entry.has_children() {
                        // the entry is basically invalid with no children or action
                        self.key_buffer.clear();
                    }

                    // in this case the entry had children so it should wait for next key as it cannot be executed
                } else {
                    self.execute_action(entry.action.clone());
                    self.key_buffer.clear();
                }
            } else {
                // just clear it as it was a unmapped sequence
                self.key_buffer.clear();
            }
        }
    }

    pub fn set_keymap(&mut self, name: &str) {
        // find the requested keymap and swap them
        if let Some(keymap) = self.keymaps.iter_mut().find(|x| x.name == name) {
            std::mem::swap(keymap, &mut self.keymap);
        }
    }

    pub fn set_buffer(&mut self, index: usize) {
        assert!(index < self.buffers.len());

        // swap the buffers
        std::mem::swap(self.buffers.get_mut(index).unwrap(), &mut self.buffer);
    }

    pub fn next_buffer(&mut self) {
        if let Some(mut buffer) = self.buffers.pop_back() {
            std::mem::swap(&mut buffer, &mut self.buffer);
            self.buffers.push_front(buffer);
        }
    }

    pub fn prev_buffer(&mut self) {
        if let Some(mut buffer) = self.buffers.pop_front() {
            std::mem::swap(&mut buffer, &mut self.buffer);
            self.buffers.push_back(buffer);
        }
    }

    fn execute_action(&mut self, action: Action) {
        action.execute(self)
    }
}

impl Widget for &Editor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        use ratatui::layout::{Constraint, Direction, Layout};
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

        // show mode
        Span::from(format!(
            "{}   {}",
            self.keymap.name[..4].to_uppercase(),
            self.buffer.name
        ))
        .into_left_aligned_line()
        .render(layout_statusbar[0], buf);

        // statusline
        Span::from(format!(
            "{}  {}:{}",
            Into::<KeySequence>::into(&self.key_buffer), // TODO dont show if no keys pressed
            self.buffer.cursor.y,
            self.buffer.cursor.x
        ))
        .into_right_aligned_line()
        .render(layout_statusbar[1], buf);

        // show the buffer
        self.buffer.render(editor_area, buf);
    }
}
