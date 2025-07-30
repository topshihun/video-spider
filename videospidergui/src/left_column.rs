use iced::Element;
use iced::widget::button;
use super::message::Message;
use videospider::series::Series;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum LeftColumnState {
    Search,
}

pub struct LeftColumn {
    state: LeftColumnState,
    items: Option<Arc<Vec<Series>>>
}

impl LeftColumn {
    pub fn new() -> Self {
        Self {
            state: LeftColumnState::Search,
            items: Option::None,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SearchClicked => self.state = LeftColumnState::Search,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let search_buttom = button("search").on_press(Message::SearchClicked);
        search_buttom.into()
    }

    pub fn get_state(&self) -> LeftColumnState {
        self.state.clone()
    }
}
