use iced::Element;
use crate::message::Message;

pub struct PageWidget {
}

impl PageWidget {
    pub fn new() -> PageWidget {
        Self {}
    }

    pub fn update(&mut self, message: Message) {
    }

    pub fn view(&self) -> Element<Message> {
        todo!();
    }
}
