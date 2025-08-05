use ratatui::crossterm::event::{KeyCode, KeyModifiers, MediaKeyCode};

/// Represents a single key
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Key {
    code: KeyCode,
    modifiers: KeyModifiers,
}

impl TryFrom<&str> for Key {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO error checking

        let mut segments: Vec<&str> = value.split('-').collect();
        let key = segments.pop().unwrap();

        let mut modifiers = KeyModifiers::NONE;
        for seg in segments {
            match seg.to_lowercase().as_str() {
                "c" => modifiers.set(KeyModifiers::CONTROL, true),
                "h" => modifiers.set(KeyModifiers::HYPER, true),
                "a" => modifiers.set(KeyModifiers::ALT, true),
                "s" => modifiers.set(KeyModifiers::SHIFT, true),
                "m" => modifiers.set(KeyModifiers::SUPER, true),
                _ => return Err(format!("Invalid modifier {seg:?}")),
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
            "pageUp" => KeyCode::PageUp,
            "pageDown" => KeyCode::PageDown,
            "tab" => KeyCode::Tab,
            "backTab" => KeyCode::BackTab,
            "delete" => KeyCode::Delete,
            "insert" => KeyCode::Insert,
            "null" => KeyCode::Null,
            "esc" => KeyCode::Esc,
            "capsLock" => KeyCode::CapsLock,
            "scrollLock" => KeyCode::ScrollLock,
            "numLock" => KeyCode::NumLock,
            "printScreen" => KeyCode::PrintScreen,
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

            // _ => {
            //     // use key as
            //     if key.len() == 1 && key.is_ascii() {
            //         let char = key.chars().next().unwrap();

            //         KeyCode::Char(key.chars().next().unwrap())
            //     } else {
            //         return Err(format!("Unknown key {key:?}"));
            //     }
            // }
            x if x.is_ascii() && x.len() == 1 => KeyCode::Char(x.chars().next().unwrap()),

            _ => return Err(format!("Unknown key {key:?}")),
        };

        Ok(Self { code, modifiers })
    }
}

impl Into<String> for Key {
    fn into(self) -> String {
        let mut output = "".to_string();

        if self.modifiers.contains(KeyModifiers::CONTROL) {
            output += "C-";
        }

        if self.modifiers.contains(KeyModifiers::HYPER) {
            output += "H-";
        }

        if self.modifiers.contains(KeyModifiers::ALT) {
            output += "A-";
        }

        if self.modifiers.contains(KeyModifiers::SHIFT) {
            output += "S-";
        }

        // TODO they should be the same thing right?
        if self.modifiers.contains(KeyModifiers::SUPER)
            || self.modifiers.contains(KeyModifiers::META)
        {
            output += "M-";
        }

        // if self.modifiers.contains(KeyModifiers::SHIFT) {
        let key: String = match self.code {
            KeyCode::Char(x) => x.to_string().to_ascii_uppercase(),
            // to differentiate it from pause media key
            KeyCode::Pause => "BREAK".to_string(),
            x => format!("{x}").to_uppercase(),
            // KeyCode::Right => "RIGHT".to_string(),
        };

        output += &key;

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_to_string() {
        assert_eq!(
            Into::<String>::into(Key {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::all(),
            }),
            "C-H-A-S-M-A"
        );

        assert_eq!(
            Into::<String>::into(Key {
                code: KeyCode::Right,
                modifiers: KeyModifiers::all(),
            }),
            "C-H-A-S-M-RIGHT"
        );
    }

    #[test]
    fn test_key_from_string() {
        assert_eq!(
            TryInto::<Key>::try_into("c-h-a-s-m-a"),
            Ok(Key {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::all() - KeyModifiers::META,
            })
        );

        assert_eq!(
            TryInto::<Key>::try_into("c-h-a-s-m-left"),
            Ok(Key {
                code: KeyCode::Left,
                modifiers: KeyModifiers::all() - KeyModifiers::META,
            })
        );
    }
}
