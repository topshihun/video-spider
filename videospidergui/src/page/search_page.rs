use iced::Element;
use iced::widget::text_input;

pub struct SearchPage {
    content: String,
}

#[derive(Debug, Clone)]
pub enum SearchPageMessage {
    SearchInputChanged(String),
}

impl SearchPage {
    pub fn new() -> Self {
        Self {
            content: String::from(""),
        }
    }

    pub fn update(&mut self, message: SearchPageMessage) {
        match message {
            SearchPageMessage::SearchInputChanged(str) => {
                self.content = str;
            },
        }
    }

    pub fn view(&self) -> Element<SearchPageMessage> {
        text_input("search input", &self.content)
            .on_input(SearchPageMessage::SearchInputChanged)
            .into()
    }
}
