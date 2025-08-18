use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use videospider::Series;

#[derive(Debug)]
pub struct State {
    pub exit: bool,
    pub tab_state: Rc<RwLock<TabState>>,
    pub series_tab_state: Rc<RwLock<SeriesTabState>>,
    pub page_state: PageState,
    pub focus_state: FocusState,
}

impl State {
    pub fn new() -> Self {
        let tab_state = Rc::new(RwLock::new(TabState::default()));
        let series_tab_state = Rc::new(RwLock::new(SeriesTabState::default()));
        Self {
            exit: false,
            tab_state: Rc::clone(&tab_state),
            series_tab_state: Rc::clone(&series_tab_state),
            page_state: PageState::Tab(Rc::clone(&tab_state)),
            focus_state: FocusState::default(),
        }
    }

    pub fn update_page_state(&mut self) {
        match self.focus_state {
            FocusState::Tab => {
                self.page_state = PageState::Tab(Rc::clone(&self.tab_state));
            }
            FocusState::SeriesTab => {
                self.page_state = PageState::Series(Rc::clone(&self.series_tab_state));
            }
            _ => {}
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
    pub series_list: Vec<Arc<Series>>,
    pub index: Option<usize>,
}

impl SeriesTabState {
    pub fn push_series(&mut self, series: &Arc<Series>) {
        self.series_list.push(Arc::clone(series));
        self.index = Some(self.series_list.len().saturating_sub(1));
    }

    pub fn get(&self) -> Option<&Series> {
        if let Some(index) = self.index {
            Some(
                self.series_list
                    .get(index)
                    .unwrap_or_else(|| panic!("index: {index} error")),
            )
        } else {
            None
        }
    }

    pub fn next(&mut self) {
        if let Some(index) = self.index {
            if index == self.series_list.len() - 1 {
                self.index = Some(0);
            } else {
                self.index = Some(index.saturating_add(1));
            }
        }
    }

    pub fn prev(&mut self) {
        if let Some(index) = self.index {
            if index == 0 {
                self.index = Some(self.series_list.len().saturating_sub(1));
            } else {
                self.index = Some(index.saturating_sub(1));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PageState {
    Tab(Rc<RwLock<TabState>>),
    Series(Rc<RwLock<SeriesTabState>>),
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
        *self = FocusState::Page;
    }

    pub fn escape(&mut self, page_state: &PageState) {
        if let FocusState::Page = self {
            match page_state {
                PageState::Tab(_) => *self = FocusState::Tab,
                PageState::Series(_) => *self = FocusState::SeriesTab,
            }
        }
    }
}

impl Default for FocusState {
    fn default() -> Self {
        Self::Tab
    }
}
