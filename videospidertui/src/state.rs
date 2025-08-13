#[derive(Debug)]
pub struct State {
    pub tab_state: TabState,
    pub series_tab_state: SeriesTabState,
    pub page_state: PageState,
    pub focus_state: FocusState,
}

impl State {
    pub fn new() -> Self {
        Self {
            tab_state: TabState::default(),
            series_tab_state: SeriesTabState::default(),
            page_state: PageState::default(),
            focus_state: FocusState::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TabState {
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

impl Default for TabState {
    fn default() -> Self {
        TabState::Home
    }
}

#[derive(Debug, Clone)]
pub struct SeriesTabState {
    index: Option<u32>,
}

impl Default for SeriesTabState {
    fn default() -> Self {
        Self { index: None }
    }
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
        match self {
            FocusState::Page => *self = FocusState::Tab,
            _ => {}
        }
    }
}

impl Default for FocusState {
    fn default() -> Self {
        Self::Tab
    }
}
