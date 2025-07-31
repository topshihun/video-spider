use iced::Element;
use iced::widget::button;
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

#[derive(Debug, Clone)]
pub enum LeftColumnMessage {
    SearchClicked,
}

impl LeftColumn {
    pub fn new() -> Self {
        Self {
            state: LeftColumnState::Search,
            items: Option::None,
        }
    }

    pub fn update(&mut self, message: LeftColumnMessage) {
        match message {
            LeftColumnMessage::SearchClicked => self.state = LeftColumnState::Search,
        }
    }

    pub fn view(&self) -> Element<LeftColumnMessage> {
        let search_buttom = button("search").on_press(LeftColumnMessage::SearchClicked);
        search_buttom.into()
    }

    pub fn get_state(&self) -> LeftColumnState {
        self.state.clone()
    }
}
