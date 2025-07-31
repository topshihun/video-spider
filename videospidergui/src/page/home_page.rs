use iced::Element;
use iced::widget::text;

pub struct HomePage {}

impl HomePage {
    pub fn new() -> Self {
        HomePage {  }
    }

    pub fn view(&self) -> Element<()> {
        text("Here is the home page").into()
    }
}
