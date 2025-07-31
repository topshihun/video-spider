
#[derive(Debug, Clone)]
pub enum State {
    Home,
    Search,
}

pub type StateChangedMessage = State;

impl State {
    pub fn default() -> Self {
        Self::Search
    }
}
