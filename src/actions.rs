use crate::editor::Editor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "action")]
#[serde(rename_all(serialize = "kebab-case"))]
pub enum Action {
    /// Basically a no-op
    None,

    /// Inserts character
    Insert {
        ch: Option<char>,
    },

    /// Cancels current chorded key
    Cancel,
    Chain {
        actions: Vec<Self>,
    },
    CursorMove {
        x: u16,
        y: u16,
    },
    CursorMoveRel {
        x: i32,
        y: i32,
    },

    SetKeymap {
        mode: String,
    },

    BufferNext,
    BufferPrev,
}

impl Action {
    pub fn execute(&self, editor: &mut Editor) {
        match self {
            Self::None => {}
            Self::Insert { ch } => todo!(),
            Self::Cancel => editor.key_buffer.clear(),

            // just run each one sequentially
            Self::Chain { actions } => {
                for i in actions {
                    i.execute(editor);
                }
            }
            Self::CursorMove { x, y } => {
                editor.buffer.cursor.x = *x;
                editor.buffer.cursor.y = *y;
            }
            Self::CursorMoveRel { x, y } => {
                editor.buffer.cursor.x = editor
                    .buffer
                    .cursor
                    .x
                    .saturating_add_signed(TryInto::try_into(*x).unwrap());
                editor.buffer.cursor.y = editor
                    .buffer
                    .cursor
                    .y
                    .saturating_add_signed(TryInto::try_into(*y).unwrap());
            }

            Self::SetKeymap { mode } => editor.set_keymap(mode),
            Self::BufferNext => editor.next_buffer(),
            Self::BufferPrev => editor.prev_buffer(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_action_serialization() {
//         let action = Action::Chain {
//             actions: vec![Action::Cancel, Action::Cancel],
//         };

//         let output = toml::to_string(&action).unwrap();

//         dbg!(&output);

//         assert_eq!(output, "xx".to_string());
//     }
// }
