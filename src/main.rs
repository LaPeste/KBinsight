mod main_window;
mod plot_widget;

use relm::Widget;
use main_window::view::Win;


fn main() {
    Win::run(()).unwrap();
}