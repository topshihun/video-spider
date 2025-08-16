use crossterm::event::KeyEvent;

#[derive(Debug)]
pub enum Message {
    KeyEvent(KeyEvent),
    Update,
}
