use ratatui::crossterm::event::KeyCode;

use crate::{actions::Actions, util::Key};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct KeymapEntry {
    action: Option<Actions>,
    map: HashMap<Key, Self>,
}

impl KeymapEntry {
    pub fn new() -> Self {
        Self {
            action: None,
            map: HashMap::new(),
        }
    }

    /// Does the entry contain an action
    pub fn get_action(&self) -> Option<&Actions> {
        self.action.as_ref()
    }

    /// Get mappings in the entry, returns None if no mappings are defined
    pub fn get_map(&self) -> Option<&HashMap<Key, Self>> {
        if self.map.is_empty() {
            None
        } else {
            Some(&self.map)
        }
    }

    // creates a sequence of keys
    pub fn sequence(keys: &[Key], action: &Actions) -> Self {
        let mut entry = KeymapEntry::new();

        entry.add(&keys, action);

        entry
    }

    pub fn add(&mut self, keys: &[Key], action: &Actions) {
        if keys.is_empty() {
            // no more keys set current entry
            self.action = Some(action.clone());
        } else {
            let first = keys.first().unwrap();

            // create the entry if it does not exist
            if !self.map.contains_key(first) {
                self.map.insert(first.clone(), Self::new());
            }

            self.map.get_mut(first).unwrap().add(&keys[1..], action);
        }
    }

    pub fn remove(&mut self, keys: &[Key]) {
        assert!(!keys.is_empty());

        let first = keys.first().unwrap();

        if keys.len() == 1 {
            // there is only one key left so just remove that key
            self.map.remove(first);
        } else {
            match self.map.get_mut(first) {
                // remove one and recurse
                Some(entry) => entry.remove(&keys[1..]),

                // not found
                None => {}
            }
        }
    }

    pub fn get(&self, keys: &[Key]) -> Option<&Self> {
        if keys.is_empty() {
            // no more keys return current entry
            Some(self)
        } else {
            match self.map.get(keys.first().unwrap()) {
                // remove one and recurse
                Some(entry) => entry.get(&keys[1..]),

                // not found
                None => None,
            }
        }
    }

    pub fn cleanup(&mut self) {
        for key in self.map.keys().cloned().collect::<Vec<_>>() {
            let value = self.map.get_mut(&key).unwrap();

            // remove entries which do not have an action or mapping
            if value.action.is_none() && value.map.is_empty() {
                self.map.remove(&key);
            } else if !value.map.is_empty() {
                value.cleanup();
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Keymap {
    pub name: String,
    keymap: KeymapEntry,
}

impl Keymap {
    pub fn new(name: String) -> Self {
        let mut x = Self {
            name,
            keymap: KeymapEntry::new(),
        };

        x.add(
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
            &Actions::Mode(crate::actions::Mode {
                mode: "TEST".into(),
            }),
        );
        x.add(
            &[
                Key {
                    code: KeyCode::Char('d'),
                    ..Default::default()
                },
                Key {
                    code: KeyCode::Char('d'),
                    ..Default::default()
                },
                Key {
                    code: KeyCode::Char('c'),
                    ..Default::default()
                },
            ],
            &Actions::Mode(crate::actions::Mode {
                mode: "TEST2".into(),
            }),
        );

        x
    }

    pub fn add(&mut self, keys: &[Key], action: &Actions) {
        self.keymap.add(keys, action);
    }

    pub fn remove(&mut self, keys: &[Key], action: &Actions) {
        self.keymap.add(keys, action);
    }

    pub fn get(&self, keys: &[Key]) -> Option<&KeymapEntry> {
        self.keymap.get(keys)
    }
}

// #[derive(Debug)]
// pub struct Keymap {
//     name: String,
//     keymap: HashMap<Vec<Key>, Actions>,
// }

// impl<'a> Keymap {
//     pub fn new(name: String) -> Self {
//         Self {
//             name,
//             keymap: HashMap::new(),
//         }
//     }

//     pub fn add(&mut self, key: Vec<Key>, action: Actions) {
//         self.keymap.insert(key, action);
//     }

//     /// Remove key binding
//     pub fn remove(&mut self, key: &Vec<Key>) {
//         self.keymap.remove(key);
//     }

//     /// Get action for specific key sequence
//     pub fn get(&'a self, key: &Vec<Key>) -> Option<&'a Actions> {
//         self.keymap.get(key)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::Sleep;
    use ratatui::crossterm::event::KeyCode;

    #[test]
    fn test_keymap_entry_set() {
        let entry = KeymapEntry::sequence(
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
            &Actions::Sleep(Sleep { time: 100 }),
        );

        dbg!(&entry);

        // TODO add more tests

        // assert_eq!(entry, KeymapEntry {
        //     action: None,
        //     map: {
        //         Key {
        //             code: Char(
        //                 'd',
        //             ),
        //             modifiers: KeyModifiers(
        //                 0x0,
        //             ),
        //         }: KeymapEntry {
        //             action: None,
        //             map: {
        //                 Key {
        //                     code: Char(
        //                         'd',
        //                     ),
        //                     modifiers: KeyModifiers(
        //                         0x0,
        //                     ),
        //                 }: KeymapEntry {
        //                     action: Some(
        //                         Sleep(
        //                             Sleep {
        //                                 time: 100,
        //                             },
        //                         ),
        //                     ),
        //                     map: {},
        //                 },
        //             },
        //         },
        //     },
        // });
    }
}
