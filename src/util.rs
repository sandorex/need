use std::fmt::Display;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MediaKeyCode};

/// Represents a single key
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Key {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl Default for Key {
    fn default() -> Self {
        Self {
            code: KeyCode::Null,
            modifiers: KeyModifiers::NONE,
        }
    }
}

impl TryFrom<&str> for Key {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (mods_raw, key) = match value.split_once('-') {
            Some(x) => x,
            None => return Err(format!("Invalid key {value:?}")),
        };

        let mut modifiers = KeyModifiers::NONE;
        for ch in mods_raw.chars() {
            match ch.to_ascii_lowercase() {
                'c' => modifiers.set(KeyModifiers::CONTROL, true),
                'h' => modifiers.set(KeyModifiers::HYPER, true),
                'a' => modifiers.set(KeyModifiers::ALT, true),
                's' => modifiers.set(KeyModifiers::SHIFT, true),
                'm' => modifiers.set(KeyModifiers::SUPER, true),
                _ => return Err(format!("Invalid modifier {ch:?}")),
            }
        }

        let code = match key.to_lowercase().as_str() {
            "backspace" => KeyCode::Backspace,
            "enter" => KeyCode::Enter,
            "left" => KeyCode::Left,
            "right" => KeyCode::Right,
            "up" => KeyCode::Up,
            "down" => KeyCode::Down,
            "home" => KeyCode::Home,
            "end" => KeyCode::End,
            "pageup" => KeyCode::PageUp,
            "pagedown" => KeyCode::PageDown,
            "tab" => KeyCode::Tab,
            "backtab" => KeyCode::BackTab,
            "delete" => KeyCode::Delete,
            "insert" => KeyCode::Insert,
            "null" => KeyCode::Null,
            "esc" => KeyCode::Esc,
            "capslock" => KeyCode::CapsLock,
            "scrolllock" => KeyCode::ScrollLock,
            "numlock" => KeyCode::NumLock,
            "printscreen" => KeyCode::PrintScreen,
            "break" => KeyCode::Pause,
            "menu" => KeyCode::Menu,

            // function keys
            "f1" => KeyCode::F(1),
            "f2" => KeyCode::F(2),
            "f3" => KeyCode::F(3),
            "f4" => KeyCode::F(4),
            "f5" => KeyCode::F(5),
            "f6" => KeyCode::F(6),
            "f7" => KeyCode::F(7),
            "f8" => KeyCode::F(8),
            "f9" => KeyCode::F(9),
            "f10" => KeyCode::F(10),
            "f11" => KeyCode::F(11),
            "f12" => KeyCode::F(12),
            "f13" => KeyCode::F(13),
            "f14" => KeyCode::F(14),
            "f15" => KeyCode::F(15),
            "f16" => KeyCode::F(16),
            "f17" => KeyCode::F(17),
            "f18" => KeyCode::F(18),
            "f19" => KeyCode::F(19),
            "f20" => KeyCode::F(20),

            // media keys
            "play" => KeyCode::Media(MediaKeyCode::Play),
            "pause" => KeyCode::Media(MediaKeyCode::Pause),
            "playpause" => KeyCode::Media(MediaKeyCode::PlayPause),
            "reverse" => KeyCode::Media(MediaKeyCode::Reverse),
            "stop" => KeyCode::Media(MediaKeyCode::Stop),
            "fastforward" => KeyCode::Media(MediaKeyCode::FastForward),
            "rewind" => KeyCode::Media(MediaKeyCode::Rewind),
            "tracknext" => KeyCode::Media(MediaKeyCode::TrackNext),
            "trackprevious" => KeyCode::Media(MediaKeyCode::TrackPrevious),
            "record" => KeyCode::Media(MediaKeyCode::Record),
            "lowervolume" => KeyCode::Media(MediaKeyCode::LowerVolume),
            "raisevolume" => KeyCode::Media(MediaKeyCode::RaiseVolume),
            "mutevolume" => KeyCode::Media(MediaKeyCode::MuteVolume),

            x if x.is_ascii() && x.len() == 1 => KeyCode::Char(x.chars().next().unwrap()),

            _ => return Err(format!("Unknown key {key:?}")),
        };

        Ok(Self { code, modifiers })
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            f.write_str("C")?;
        }

        if self.modifiers.contains(KeyModifiers::HYPER) {
            f.write_str("H")?;
        }

        if self.modifiers.contains(KeyModifiers::ALT) {
            f.write_str("A")?;
        }

        if self.modifiers.contains(KeyModifiers::SHIFT) {
            f.write_str("S")?;
        }

        // im gonna assume super and meta are the same modifier
        if self.modifiers.contains(KeyModifiers::SUPER)
            || self.modifiers.contains(KeyModifiers::META)
        {
            f.write_str("M")?;
        }

        // write the dash
        if !self.modifiers.is_empty() {
            f.write_str("-")?;
        }

        // if self.modifiers.contains(KeyModifiers::SHIFT) {
        let key: String = match self.code {
            KeyCode::Char(x) => x.to_string().to_ascii_uppercase(),
            // to differentiate it from pause media key
            KeyCode::Pause => "BREAK".to_string(),

            // KeyCode already has Display implemented
            x => format!("{x}").to_uppercase(),
        };

        f.write_str(&key)?;

        Ok(())
    }
}

impl From<KeyEvent> for Key {
    fn from(value: KeyEvent) -> Self {
        Key {
            code: value.code,
            modifiers: value.modifiers,
        }
    }
}

/// Wraps a `Vec<Key>` so it implements `Display`
#[derive(Debug, Clone)]
pub struct KeySequence(pub Vec<Key>);

impl From<Key> for KeySequence {
    fn from(value: Key) -> Self {
        Self(vec![value])
    }
}

impl From<&Key> for KeySequence {
    fn from(value: &Key) -> Self {
        Self(vec![value.clone()])
    }
}

impl From<&[Key]> for KeySequence {
    fn from(value: &[Key]) -> Self {
        Self(value.to_vec())
    }
}

impl From<Vec<Key>> for KeySequence {
    fn from(value: Vec<Key>) -> Self {
        Self(value)
    }
}

impl From<&Vec<Key>> for KeySequence {
    fn from(value: &Vec<Key>) -> Self {
        Self(value.clone())
    }
}

impl Display for KeySequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;

        for key in &self.0 {
            f.write_str(&format!(" {}", key))?;
        }

        f.write_str(" ]")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_to_string() {
        assert_eq!(
            format!(
                "{}",
                Key {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::all(),
                }
            ),
            "CHASM-A"
        );

        assert_eq!(
            format!(
                "{}",
                Key {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::all(),
                }
            ),
            "CHASM-RIGHT"
        );
    }

    #[test]
    fn test_key_from_string() {
        assert_eq!(
            TryInto::<Key>::try_into("chasm-a"),
            Ok(Key {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::all() - KeyModifiers::META,
            })
        );

        assert_eq!(
            TryInto::<Key>::try_into("chasm-left"),
            Ok(Key {
                code: KeyCode::Left,
                modifiers: KeyModifiers::all() - KeyModifiers::META,
            })
        );
    }

    #[test]
    fn test_keysequence_to_string() {
        let seq = KeySequence(vec![
            Key {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            },
            Key {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::SHIFT,
            },
        ]);
        assert_eq!(format!("{}", seq), "[ CS-D S-D ]");
    }
}
