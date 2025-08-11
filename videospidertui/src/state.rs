
#[derive(Debug)]
pub struct State {
    pub ui: UiState,
    pub focus: FocusState,
}

impl State {
    pub fn new() -> Self {
        Self {
            ui: UiState::default(),
            focus: FocusState::default(),
        }
    }
}

#[derive(Debug)]
pub enum UiState {
    Home,
    Search,
}

impl UiState {
    pub fn next(&mut self) {
        match self {
            UiState::Home =>
                *self = UiState::Search,
            UiState::Search =>
                *self = UiState::Home,
        }
    }

    pub fn prev(&mut self) {
        match self {
            UiState::Home =>
                *self = UiState::Search,
            UiState::Search =>
                *self = UiState::Home,
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::Search
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
            _ => {},
        }
    }

    pub fn prev(&mut self) {
        match self {
            FocusState::Tab => *self = FocusState::SeriesTab,
            FocusState::SeriesTab => *self = FocusState::Tab,
            _ => {},
        }
    }

    pub fn enter(&mut self) {
        match self {
            FocusState::Tab => *self = FocusState::Page,
            FocusState::SeriesTab => *self = FocusState::Page,
            _ => {},
        }
    }

    pub fn escape(&mut self) {
        match self {
            FocusState::Page => *self = FocusState::Tab,
            _ => {},
        }
    }
}

impl Default for FocusState {
    fn default() -> Self {
        Self::Tab
    }
}
