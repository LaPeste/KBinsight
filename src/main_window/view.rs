use relm::{Relm, Update, Widget};
use gtk::prelude::*;
use gtk::*;
use gtk::{Window, Inhibit, WindowType};
use relm_derive::Msg;
use relm_derive::widget;

use super::model::Model;

#[derive(Msg)]
pub enum Msg {
    Flush,
    KeyDown,
    Quit,
}

#[widget]
impl Widget for Win {

    fn model() -> Model {
        Model::new()
    }

    // gtk::prelude::add_events(GDK_KEY_PRESS_MASK);
    // a keyboard is one of the many slave devices
    // gtk::WidgetExt::add_device_events(gdk::EventKey);//Widget:://add_events(gdk::EventMask::EventKey);

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Flush => {
                self.model.flush_cps();
                self.cps.set_text(&self.model.chars_per_sec().to_string());
            }
            Msg::KeyDown => self.model.key_pressed(),
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Orientation::Vertical,
                #[name="hbox"]
                gtk::Box {
                    orientation: Orientation::Horizontal,
                },
                #[name="add_button"]
                gtk::Button {
                    label: "Add",
                    clicked => Msg::KeyDown,
                },
                #[name="flush_button"]
                gtk::Button {
                    label: "Flush",
                    clicked => Msg::Flush,
                },
                #[name="cps"]
                gtk::Label {
                    text: &self.model.chars_per_sec().to_string(),
                },
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}