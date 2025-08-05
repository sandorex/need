mod actions;
mod app;
mod buffer;
mod editor;
mod keymap;
mod util;

use app::App;

pub const MODE_NORM: &str = "Normal";
pub const MODE_EDIT: &str = "Edit";

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

// use std::time::Duration;

// use crossterm::event::{self, Event, KeyCode};
// use ratatui::{Frame, text::Text};

// fn main() -> anyhow::Result<()> {
//     let mut terminal = ratatui::init();

//     let mut run = true;

//     while run {
//         terminal.draw(|frame| draw(frame))?;

//         // TODO not working properly, the update is not called each time
//         println!("hello");

//         // handle events only if there are any
//         if crossterm::event::poll(Duration::from_millis(100))? {
//             match crossterm::event::read()? {
//                 Event::Key(x) if x.code == KeyCode::Char('q') => {
//                     run = false;
//                 }
//                 x => println!("{x:?}"),
//             }
//         }
//     }
//     // loop {
//     //     terminal.draw(draw).expect("failed to draw frame");
//     //     if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
//     //         break;
//     //     }
//     // }
//     ratatui::restore();
//     Ok(())
// }

// fn draw(frame: &mut Frame) {
//     let text = Text::raw("Hello World!");
//     frame.render_widget(text, frame.area());
// }
