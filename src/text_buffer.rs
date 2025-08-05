use ratatui::{
    layout::{Position, Rect},
    style::Style,
    widgets::Widget,
};

use crate::editor::EditorMode;

#[derive(Debug)]
pub struct TextBuffer {
    pub lines: Vec<String>,
    pub cursor: Position,
    pub view: Position,

    /// Current mode of the editor
    pub mode: EditorMode,
}

impl TextBuffer {
    pub fn raw(text: &str) -> Self {
        Self {
            lines: text.lines().map(|x| x.to_string()).collect::<Vec<_>>(),
            ..Default::default()
        }
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: Position { x: 1, y: 1 },
            view: Position { x: 0, y: 0 },
            mode: EditorMode::Normal,
        }
    }
}

impl Widget for &TextBuffer {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if area.height <= TryInto::<u16>::try_into(self.lines.len()).unwrap() {
            for (i, line) in self.lines.iter().skip(self.view.y.into()).enumerate() {
                let y = area.y + TryInto::<u16>::try_into(i).unwrap();

                if y >= area.height {
                    break;
                }

                buf.set_string(area.x, y, line, Style::default());
            }
        } else {
            // just print all cause it fits
            for (i, line) in self.lines.iter().enumerate() {
                buf.set_string(
                    area.x,
                    area.y + TryInto::<u16>::try_into(i).unwrap(),
                    line,
                    Style::default(),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;

    // TODO write actual tests
    #[test]
    fn test_textview() {
        let textbuffer = TextBuffer {
            lines: vec!["Hello there".to_string(); 25],
            ..Default::default()
        };

        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 5));

        textbuffer.render(buf.area, &mut buf);

        let expected = Buffer::with_lines(vec!["aa"]);

        assert_eq!(buf, expected);
    }
}
