use ratatui::{
    buffer::Buffer as RBuffer,
    crossterm::event::Event,
    layout::{Position, Rect},
    style::Stylize,
    widgets::Widget,
};
use std::{fmt::Debug, path::Path};

#[derive(Debug, Clone)]
pub struct Buffer {
    /// Name shown for the buffer
    pub name: String,

    // NOTE for now just use this
    pub lines: Vec<String>,

    /// Where the cursor is in the file
    pub cursor: Position,

    /// Basically where in the file the current view starts
    pub view: Position,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            name: "Scratch".to_string(),
            lines: vec!["".to_string()],
            cursor: Position { x: 0, y: 0 },
            view: Position { x: 0, y: 0 },
        }
    }
}

impl Buffer {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let file = std::fs::read_to_string(path)?;

        let mut buffer = Self::from_raw(&file);

        // rename buffer
        buffer.name = Path::new(path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        Ok(buffer)
    }

    pub fn from_raw(raw: &str) -> Self {
        Self {
            lines: raw.lines().map(|x| x.to_string()).collect::<Vec<_>>(),
            ..Default::default()
        }
    }

    pub fn handle_event(&mut self, event: &Event) {}
}

impl Widget for &Buffer {
    fn render(self, area: Rect, buf: &mut RBuffer)
    where
        Self: Sized,
    {
        use ratatui::style::Style;

        if area.height <= TryInto::<u16>::try_into(self.lines.len()).unwrap() {
            // fit as many as possible
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

        buf.set_style(
            Rect::new(
                area.x + (self.cursor.x - self.view.x),
                area.y + (self.cursor.y - self.view.y),
                1,
                1,
            ),
            Style::default().reversed(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer as RBuffer;

    // TODO write actual tests
    #[test]
    fn test_buffer_basic() {
        let textbuffer = Buffer {
            lines: vec!["Hello there".to_string(); 25],
            ..Default::default()
        };

        let mut buf = RBuffer::empty(Rect::new(0, 0, 20, 5));

        textbuffer.render(buf.area, &mut buf);

        let expected = RBuffer::with_lines(vec!["aa"]);

        assert_eq!(buf, expected);
    }
}
