extern crate termion;
use ctui::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;
use std::{io, io::stdin, thread, time, 
        sync::mpsc, sync::mpsc::{Receiver, TryRecvError}};

fn main() -> Result<(), std::io::Error> {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    let mut app = App::new("UI");
    let event_key = key_handle();

    loop {
        ui::draw_ui(&mut terminal, &mut app).unwrap();
        match event_key.try_recv() {
            Ok(k) => {
                match k {
                    Key::Esc => return Ok(()),
                    _ => {}
                }
            },
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
    }  //end of loop
}

fn key_handle() -> Receiver<Key> {
    let (tx, rx) = mpsc::channel::<Key>();
    let stdin = stdin();
    thread::spawn(move || for c in stdin.keys() {
        match c {
            Ok(c) => {
                tx.send(c).unwrap();
            },
            Err(e) => {

            }
        }
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}


