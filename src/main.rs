use relm::Widget;

mod main_window;
use main_window::view::Win;

fn main() {
    Win::run(()).unwrap();
}