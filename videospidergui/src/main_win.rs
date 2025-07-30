use iced::Element;
use super::message::Message;
use super::left_column::LeftColumn;

pub struct MainWin {
    left_column: LeftColumn,
}

impl MainWin {
    pub fn title(&self) -> String {
        "VideoSpider".to_string()
    }

    pub fn update(&mut self, message: Message) {
        self.left_column.update(message);
    }

    pub fn view(&self) -> Element<Message> {
        self.left_column.view()
    }
}

impl Default for MainWin {
    fn default() -> Self {
        MainWin { 
            left_column: LeftColumn::new(),
        }
    }
}
