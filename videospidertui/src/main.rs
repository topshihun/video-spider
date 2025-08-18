use std::{sync::mpsc::channel, thread};

use crossterm::event::{self, KeyEventKind};

use crate::message::Message;

mod app;
mod message;
mod page;
mod series_tab;
mod state;
mod tab;
mod utils;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let (sender, recv) = channel::<Message>();
    let sender_event = sender.clone();
    let sender_app = sender.clone();
    thread::spawn(move || {
        loop {
            match event::read().unwrap() {
                event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    sender_event
                        .send(Message::KeyEvent(key_event))
                        .expect("key event send error");
                }
                event::Event::Resize(_, _) => {
                    sender_event
                        .send(Message::Update)
                        .expect("resize event send error");
                }
                _ => {}
            }
        }
    });
    let result = app::App::new(sender_app).run(&mut terminal, recv);
    ratatui::restore();
    result
}
