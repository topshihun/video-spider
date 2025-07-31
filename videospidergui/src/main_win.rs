use iced::Element;
use iced::widget::row;
use super::left_column::LeftColumn;
use super::page::{PageWidget, PageWidgetMessage};
use super::state::StateChangedMessage;

pub struct MainWin {
    left_column: LeftColumn,
    page_widget: PageWidget,
}

#[derive(Debug, Clone)]
pub enum MainWinMessage {
    StateChangedMessage(StateChangedMessage),
    PageWidgetMessage(PageWidgetMessage),
}

impl MainWin {
    pub fn title(&self) -> String {
        "VideoSpider".to_string()
    }

    pub fn update(&mut self, message: MainWinMessage) {
        match message {
            MainWinMessage::StateChangedMessage(msg) => {
                self.left_column.update(msg.clone());
                self.page_widget.update(PageWidgetMessage::StateChanged(msg));
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
                .map(|msg| MainWinMessage::StateChangedMessage(msg)),
            self.page_widget
                .view()
                .map(|msg| MainWinMessage::PageWidgetMessage(msg)),
        ]
            .spacing(5)
            .into()
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
