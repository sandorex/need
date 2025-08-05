use crate::editor::Editor;
use serde::{Deserialize, Serialize};

pub trait Action {
    // fn name(&self) -> &str;
    // fn description(&self) -> &str;
    // maybe add &mut Buffer? so the action can edit the current buffer directly?
    fn execute(&self, editor: &mut Editor);
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "action")]
#[serde(rename_all(serialize = "kebab-case"))]
pub enum Actions {
    Chain(Chain),
    Sleep(Sleep),
    Mode(Mode),
    MoveTo(MoveTo),
}

impl Actions {
    pub fn execute(&self, editor: &mut Editor) {
        match self {
            Self::Chain(x) => x.execute(editor),
            Self::Sleep(x) => x.execute(editor),
            Self::Mode(x) => x.execute(editor),
            Self::MoveTo(x) => x.execute(editor),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chain {
    chain: Vec<Actions>,
}

impl Action for Chain {
    fn execute(&self, editor: &mut Editor) {
        for action in &self.chain {
            action.execute(editor);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sleep {
    /// Time in millis
    time: u64,
}

impl Action for Sleep {
    fn execute(&self, editor: &mut Editor) {
        // TODO if i do raw sleep then will the editor freeze?
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mode {
    mode: String,
}

impl Action for Mode {
    fn execute(&self, editor: &mut Editor) {
        // editor.
        // TODO editor does not have a global mode
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveTo {
    pub x: u16,
    pub y: u16,
}

impl Action for MoveTo {
    fn execute(&self, editor: &mut Editor) {
        // editor.view.cursor.x = self.x;
        // editor.view.cursor.y = self.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actions() {
        let action = Actions::Chain(Chain {
            chain: vec![
                Actions::MoveTo(MoveTo { x: 16, y: 12 }),
                Actions::Mode(Mode {
                    mode: "aaa".to_string(),
                }),
            ],
        });

        assert_eq!(toml::to_string(&action).unwrap(), "xx".to_string());
    }
}
