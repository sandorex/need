use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::actions::Actions;
use std::collections::HashMap;

// use crossterm::event::KeyEvent;
//
//
// 'x' -> (action / ->) 'x' -> Action

// #[derive(Debug, Clone)]
// pub struct KeySequence {
//     pub keys: Vec<KeyEvent>,
// }

#[derive(Debug, Default)]
pub struct Keymap {
    keymap: HashMap<KeyEvent, Actions>,
}

impl Keymap {
    pub fn add_key(&mut self, code: KeyCode, modifiers: KeyModifiers, action: Actions) {
        // self.keymap
    }
}
