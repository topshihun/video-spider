use iced::{Element, Theme};
use iced::{Color, Border, Background, border::Radius, Shadow};
use iced::widget::{button, column, container};
use iced::Length::Fill;
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
            .width(Fill)
            .style(button_style)
            .on_press(StateChangedMessage::Home);
        let search_button = button("search")
            .width(Fill)
            .style(button_style)
            .on_press(StateChangedMessage::Search);
        let container = container(column![
            home_button,
            search_button,
        ]
            .width(150)
            .spacing(0))
            .style(container::dark)
            .height(Fill);
        container.into()
    }
}

fn button_style(_: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb8(105, 115, 255))),
        text_color: Color::WHITE,
        border: Border {
            color: Color::BLACK,
            width: 0.0,
            radius: Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_right: 0.0,
                bottom_left: 0.0,
            },
        },
        shadow: Shadow::default(),
    }
}
