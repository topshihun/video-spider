use iced::Element;
use iced::Length::Fill;
use iced::widget::{column, container, text};

pub struct HomePage {}

impl HomePage {
    pub fn new() -> Self {
        HomePage {}
    }

    pub fn view(&self) -> Element<()> {
        let text_notice = text("Here is the home page").center();
        let text_nothing = text("Nothing").center();
        container(column![text_notice, text_nothing])
            .center(Fill)
            .into()
    }
}
