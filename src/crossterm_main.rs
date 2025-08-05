use anyhow::{Context, Result};
use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, KeyEvent},
    style, terminal,
};
use std::ops::RangeBounds;
use std::{
    io::{Write, stdout},
    sync::Arc,
};

fn main() -> Result<()> {
    let mut app = App::new()?;

    app.init()?;

    let results = app.main();

    app.teardown()?;

    results
}

#[derive(Debug)]
pub struct App {
    pub tty: std::io::Stdout,
    pub exit: bool,
    pub size: (u16, u16),
    // pub buffers:
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(App {
            tty: stdout(),
            exit: false,
            size: terminal::size()?,
        })
    }

    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        self.tty.queue(event::EnableBracketedPaste)?;
        self.tty.queue(event::EnableFocusChange)?;
        self.tty.queue(event::EnableMouseCapture)?;
        self.tty.queue(terminal::EnterAlternateScreen)?;
        self.tty.queue(terminal::Clear(terminal::ClearType::All))?;
        self.tty.queue(cursor::Hide)?;
        self.tty.queue(terminal::SetTitle(concat!(
            env!("CARGO_PKG_NAME"),
            " ",
            env!("CARGO_PKG_VERSION")
        )))?;
        self.tty.flush()?;

        Ok(())
    }

    pub fn teardown(&mut self) -> Result<()> {
        self.tty.queue(event::DisableBracketedPaste)?;
        self.tty.queue(event::DisableFocusChange)?;
        self.tty.queue(event::DisableMouseCapture)?;
        self.tty.queue(terminal::LeaveAlternateScreen)?;
        self.tty.queue(cursor::Show)?;
        self.tty.flush()?;
        terminal::disable_raw_mode()?;

        Ok(())
    }

    pub fn main(&mut self) -> Result<()> {
        while !self.exit {
            self.render()?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                event::Event::Key(event) => match event.code {
                    // NOTE char is case sensitive
                    event::KeyCode::Char('q') => self.exit = true,
                    _ => {}
                },
                event::Event::Resize(x, y) => self.resize(self.size, (x, y)),
                _ => {}
            }
        }

        Ok(())
    }

    fn resize(&mut self, _old: (u16, u16), new: (u16, u16)) {
        self.size = new;
    }

    fn render(&mut self) -> Result<()> {
        self.tty.queue(terminal::Clear(terminal::ClearType::All))?;
        self.tty.queue(cursor::MoveTo(0, 0))?;

        self.tty
            .queue(style::PrintStyledContent(style::style(format!(
                "Got {}, {}",
                self.size.0, self.size.1
            ))))?;
        self.tty.flush()?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum Node {
    Value {
        raw_prefix: String,

        /// Contains only the relevant information (contains whitespace if relevant)
        text: String,

        raw_suffix: String,
    },
    Parent {
        children: Vec<Self>,
    },
}

impl Node {
    pub fn get_raw(&self) -> String {
        match self {
            Self::Value {
                raw_prefix,
                text,
                raw_suffix,
            } => format!("{}{}{}", raw_prefix, text, raw_suffix),
            Self::Parent { children } => {
                let mut output = String::new();
                for child in children {
                    output += &child.get_raw();
                }

                output
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_basic() {
        let node = Node::Value {
            raw_prefix: " ".into(),
            text: "x".into(),
            raw_suffix: "".into(),
        };
        let node2 = Node::Value {
            raw_prefix: "".into(),
            text: "==".into(),
            raw_suffix: " ".into(),
        };
        let node3 = Node::Value {
            raw_prefix: " ".into(),
            text: "y".into(),
            raw_suffix: "".into(),
        };

        let nodep = Node::Parent {
            children: vec![node, node2, node3],
        };
        assert_eq!(nodep.get_raw(), r#" x==  y"#);
    }
}

// pub struct Node {
//     pub raw: String,

//     /// Contains only the relevant information (contains whitespace if relevant)
//     pub text: String,

//     /// Children
//     pub children: Vec<Self>,
// }

// Represents text in the file, whatever structure it has
// #[derive(Debug)]
// pub struct Tree {
//     /// Contains whole file with whitespace etc
//     pub whole: Vec<Arc<String>>,

//     /// Contains only the
//     pub nodes: Vec<Node>,
//     // pub objects: Vec<Vec<Arc<String>>>,
//     // pub objects:
// }

// pub struct Buffer {

// }
