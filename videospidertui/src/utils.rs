use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::Block,
};

pub fn style_block<'a>(title: impl Into<&'a str>, selected: bool) -> Block<'a> {
    const NORMAL_STYLE: Style = Style::new().fg(Color::White);
    const SELECTED_STYLE: Style = Style::new().fg(Color::Blue);

    Block::bordered()
        .style(if selected {
            SELECTED_STYLE
        } else {
            NORMAL_STYLE
        })
        .title(title.into())
}

pub fn style_text<'a>(text: impl Into<&'a str>, selected: bool) -> Text<'a> {
    const NORMAL_STYLE: Style = Style::new().fg(Color::White);
    const SELECTED_STYLE: Style = Style::new().fg(Color::White).bg(Color::Blue);

    Text::from(text.into())
        .style(if selected {
            SELECTED_STYLE
        } else {
            NORMAL_STYLE
        })
        .centered()
}
