use std::sync::mpsc;
use std::{io, thread};

use termion::event::Key;
use termion::input::TermRead;
use std::time;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

extern crate chrono;
use chrono::prelude::*;

pub enum Event<I> {
    Input(I),  
    Timer(),
}

pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    _input_handle: thread::JoinHandle<()>,
    _timer_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));

        //Input Keys
        let _input_handle = {
            let tx = tx.clone();
            let ignore_exit_key = ignore_exit_key.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                        if !ignore_exit_key.load(Ordering::Relaxed) && key == Key::Esc {
                            return;
                        }
                    }
                }
            })
        };

        let _timer_handle = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                thread::sleep(time::Duration::from_millis(250));
                tx.send(Event::Timer());
            })
        };

        Events {
            rx,
            _input_handle,
            _timer_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
