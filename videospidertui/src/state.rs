#[derive(Debug)]
pub struct State {
    pub exit: bool,
    pub tab_state: TabState,
    pub series_tab_state: SeriesTabState,
    pub page_state: PageState,
    pub focus_state: FocusState,
}

impl State {
    pub fn new() -> Self {
        Self {
            exit: false,
            tab_state: TabState::default(),
            series_tab_state: SeriesTabState::default(),
            page_state: PageState::default(),
            focus_state: FocusState::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum TabState {
    #[default]
    Home,
    Search,
}

impl TabState {
    pub fn next(&mut self) {
        match self {
            TabState::Home => *self = TabState::Search,
            TabState::Search => *self = TabState::Home,
        }
    }

    pub fn prev(&mut self) {
        match self {
            TabState::Home => *self = TabState::Search,
            TabState::Search => *self = TabState::Home,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SeriesTabState {
    index: Option<u32>,
}

#[derive(Debug)]
pub enum PageState {
    Tab(TabState),
    Series(SeriesTabState),
}

impl Default for PageState {
    fn default() -> Self {
        Self::Tab(TabState::default())
    }
}

#[derive(Debug)]
pub enum FocusState {
    Tab,
    SeriesTab,
    Page,
}

impl FocusState {
    pub fn next(&mut self) {
        match self {
            FocusState::Tab => *self = FocusState::SeriesTab,
            FocusState::SeriesTab => *self = FocusState::Tab,
            _ => {}
        }
    }

    pub fn prev(&mut self) {
        match self {
            FocusState::Tab => *self = FocusState::SeriesTab,
            FocusState::SeriesTab => *self = FocusState::Tab,
            _ => {}
        }
    }

    pub fn enter(&mut self) {
        match self {
            FocusState::Tab => *self = FocusState::Page,
            FocusState::SeriesTab => *self = FocusState::Page,
            _ => {}
        }
    }

    pub fn escape(&mut self) {
        if let FocusState::Page = self {
            *self = FocusState::Tab
        }
    }
}

impl Default for FocusState {
    fn default() -> Self {
        Self::Tab
    }
}
