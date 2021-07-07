/* TUI */
extern crate termion;

use ctui::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;
use std::io;
use std::io::stdin;

use std::{thread, time};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, TryRecvError};

fn main() {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    let mut events = Events::new();
    let mut app = App::new("UI");

    loop {
        ui::draw_ui(&mut terminal, &mut app).unwrap();
        match events.next().unwrap() {
            Event::Input(key) => match key {
                Key::Esc => { break; },
                _ => {}
            },
            Event::Timer() => app.start(),
        }
    }  //end of loop
}

