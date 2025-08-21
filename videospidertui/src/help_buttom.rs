use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::{
    help_info::{
        HOME_PAGE_HELP_INFO, SEARCH_PAGE_HELP_INFO, SERIES_PAGE_HELP_INFO, SERIESTAB_HELP_INFO,
        TAB_HELP_INFO,
    },
    state::{FocusState, PageState, State, TabState},
};

// Translate [(&str, &str)] to Vec<String>.
// (o, k) => String::from("o : k")
macro_rules! collect_help_info {
    ($i:ident) => {{
        let mut info_list = Vec::new();
        for (o, k) in $i {
            let mut str = String::new();
            str.push_str(o);
            str.push(':');
            str.push_str(k);
            info_list.push(str);
        }
        info_list
    }};
}

pub struct HelpButtom {}

impl HelpButtom {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: &State) {
        let info_list = match &state.focus_state {
            FocusState::Tab => collect_help_info!(TAB_HELP_INFO),
            FocusState::SeriesTab => collect_help_info!(SERIESTAB_HELP_INFO),
            FocusState::Page => match &state.page_state {
                PageState::Tab(tab_state) => match *tab_state.read().unwrap() {
                    TabState::Home => collect_help_info!(HOME_PAGE_HELP_INFO),
                    TabState::Search => collect_help_info!(SEARCH_PAGE_HELP_INFO),
                },
                PageState::Series(_) => collect_help_info!(SERIES_PAGE_HELP_INFO),
            },
        };

        let str = info_list.join("  |  ");
        let paragraph = Paragraph::new(str);
        frame.render_widget(paragraph, area);
    }
}
