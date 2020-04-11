use relm::{Relm, Update, Widget};
use gtk::prelude::*;
use gtk::*;
use gtk::{Window, WidgetExt, Inhibit, WindowType};
use relm_derive::Msg;
use gdk::{EventKey, keyval_to_unicode, enums::key};
use relm_derive::{widget};

use super::model::Model;

#[derive(Msg)]
pub enum Msg {
    Flush,
    KeyPress(EventKey),
    KeyDown,
    Quit,
}

#[widget]
impl Widget for Win {

    fn model() -> Model {
        Model::new()
    }

    fn update(&mut self, event: Msg) {
        let chars_to_skip = vec![ key::Tab, key::Caps_Lock,
                            key::Shift_L, key::Shift_R,
                            key::Control_L, key::Control_R,  
                            key::Super_L, key::Super_R, key::Alt_L, key::Alt_R,
                            key::Num_Lock, key::Return, key::BackSpace ];
        match event {
            Msg::Flush => {
                self.model.flush_cps();
            }
            Msg::KeyPress(event) => {
                let mut to_skip = false;
                for curr_char in chars_to_skip {
                    if event.get_keyval() == curr_char {

                        #[cfg(debug_assertions)]
                        {
                            println!("Skipped! ;)");
                        }
                        to_skip = true;
                        break;
                    }
                }
                if !to_skip {
                    self.model.key_pressed();
                }
            },
            Msg::KeyDown => self.model.key_pressed(),
            Msg::Quit => gtk::main_quit(),
        }
        self.cps.set_text(&self.model.chars_per_sec().to_string());
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
            key_press_event(_, event) => (Msg::KeyPress(event.clone()), Inhibit(false)),
        }
    }
}