use relm::{Relm, Update, Widget, connect};
use gtk::prelude::*;
use gtk::*;
use gtk::{Window, WidgetExt, Inhibit, WindowType};
use relm_derive::Msg;
use gdk::{EventKey, keyval_to_unicode, enums::key};
use relm_derive::{widget};

use std::time::{Duration, Instant};
use std::convert::TryFrom;

// use super::model::Model;
use super::super::plot_widget::PlotWidget; // or crate::plot
use super::super::plot_widget::Msg as plot_msg;

#[derive(Msg)]
pub enum Msg {
    Flush,
    KeyPress(EventKey),
    KeyDown,
    Quit,
}

/**
 * CPM : chars per minute
 * WPM : word per minute
 */
pub struct Model {
    relm: Relm<Win>,

    // algorithm to count WPM
    cps: f32,
    wpm: i32,

    // assuming average lenght of word x, count how many chars have been
    // written since last time
    count_char_for_word: i32,
    last_input_char: Instant,
    start_typ_time: Instant,
    in_typing: bool,
    chars_count: i32,
    keys_per_sec: Vec<(f32, f32)>,
}



impl Model {
    // pub fn new(relm: &relm::Relm<Self>) -> Model {
    //     Model {
    //         relm: relm.clone(),
    //         cps: 0.0,
    //         wpm: 0,
    //         count_char_for_word: 0,
    //         start_typ_time: Instant::now(),
    //         last_input_char: Instant::now(),
    //         in_typing: false,
    //         chars_count: 0,
    //         keys_per_sec: Vec::new(),
    //     }
    // }

    pub fn chars_per_sec(&self) -> f32 {
        self.cps
    }

    /* Returns true if flushed, false otherwise */
    pub fn key_pressed(&mut self) {
        let elaps_sec_from_last_typ = elapsed_sec(self.last_input_char, Instant::now());
        self.last_input_char = Instant::now();

        if !self.in_typing {
            self.start_typ_time = Instant::now();
            self.chars_count = 0;
            self.in_typing = true;
        }
        else {
            if elaps_sec_from_last_typ > 1.0 {
                self.flush_cps();
            }
        }

        self.chars_count += 1;
    }

    pub fn flush_cps(&mut self) {
        if self.in_typing {
            self.cps = cps_alg(self.chars_count, self.start_typ_time, self.last_input_char);
            self.in_typing = false;
        }
    }

    pub fn add_keys_per_sec(&mut self, value: (f32, f32)) {
        self.keys_per_sec.push(value);
    }

    pub fn get_keys_per_sec(&self) -> &Vec<(f32, f32)> {
        &self.keys_per_sec
    }
}

fn cps_alg(chars_count: i32, start: Instant, end: Instant) -> f32 {
    let elapsed_sec = elapsed_sec(start, end);
    chars_count as f32 / elapsed_sec as f32
}

fn elapsed_sec(start: Instant, end: Instant) -> f64{
    let elapsed_time = end - start;
    elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 * 1e-9
}

#[widget]
impl Widget for Win {

    fn init_view(&mut self) {
        
    }

    fn model(relm_in: &Relm<Self>, _: ()) -> Model {
        //Model::new(relm)
        Model {
            relm: relm_in.clone(),
            cps: 0.0,
            wpm: 0,
            count_char_for_word: 0,
            start_typ_time: Instant::now(),
            last_input_char: Instant::now(),
            in_typing: false,
            chars_count: 0,
            keys_per_sec: Vec::new(),
        }
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
        
        //relm::connect!(self.model.relm@Msg::Flush, cps_plot, plot_msg::Draw);

        self.cps.set_text(&self.model.chars_per_sec().to_string());
    }

    view! {
        #[name="main_window"]
        gtk::Window {
            property_default_width: 1000,
            property_default_height: 600,
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
                #[name="cps_plot"]
                PlotWidget {
                    //test: "test",
                },
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
            key_press_event(_, event) => (Msg::KeyPress(event.clone()), Inhibit(false)),
        }
    }
}