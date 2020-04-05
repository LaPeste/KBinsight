mod main_window;

use relm::Widget;
use main_window::view::Win;

fn main() {
    Win::run(()).unwrap();
}