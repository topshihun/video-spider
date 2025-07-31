use iced::Element;
use iced::widget::row;
use super::left_column::{LeftColumn, LeftColumnMessage};
use super::page::{PageWidget, PageWidgetMessage};

pub struct MainWin {
    left_column: LeftColumn,
    page_widget: PageWidget,
}

#[derive(Debug, Clone)]
pub enum MainWinMessage {
    LeftColumnMessage(LeftColumnMessage),
    PageWidgetMessage(PageWidgetMessage),
}

impl MainWin {
    pub fn title(&self) -> String {
        "VideoSpider".to_string()
    }

    pub fn update(&mut self, message: MainWinMessage) {
        match message {
            MainWinMessage::LeftColumnMessage(msg) => {
                self.left_column.update(msg);
            },
            MainWinMessage::PageWidgetMessage(msg) => {
                self.page_widget.update(msg);
            },
        }
    }

    pub fn view(&self) -> Element<MainWinMessage> {
        row![
            self.left_column
                .view()
                .map(|msg| MainWinMessage::LeftColumnMessage(msg)),
            self.page_widget
                .view()
                .map(|msg| MainWinMessage::PageWidgetMessage(msg)),
        ].into()
    }
}

impl Default for MainWin {
    fn default() -> Self {
        MainWin { 
            left_column: LeftColumn::new(),
            page_widget: PageWidget::new(),
        }
    }
}
