use iced::Element;
use iced::widget::{button, column};
use videospider::series::Series;
use std::sync::Arc;
use super::state::{State, StateChangedMessage};

pub struct LeftColumn {
    state: State,
    items: Option<Arc<Vec<Series>>>
}

impl LeftColumn {
    pub fn new() -> Self {
        Self {
            state: State::default(),
            items: Option::None,
        }
    }

    pub fn update(&mut self, message: StateChangedMessage) {
        self.state = message;
    }

    pub fn view(&self) -> Element<StateChangedMessage> {
        let home_button = button("home")
            .on_press(StateChangedMessage::Home);
        let search_button = button("search")
            .on_press(StateChangedMessage::Search);
        column![
            home_button,
            search_button,
        ].into()
    }
}
