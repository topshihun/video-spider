mod page;
mod main_win;
mod left_column;
mod state;

use main_win::MainWin;

fn main() -> iced::Result {
    iced::application(MainWin::title, MainWin::update, MainWin::view).run()
}
