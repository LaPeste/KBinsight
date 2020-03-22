use relm::{Relm, Update, Widget};
use gtk::prelude::*;
use gtk::*;
use gtk::{Window, Inhibit, WindowType};
use relm_derive::Msg;

use super::model::Model;

pub struct Win {
    model: Model,
    window: Window,
}

#[derive(Msg)]
pub enum Msg {
    KeyDown,
    Quit,
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);
        
        // Connect the signal `delete_event` to send the `Quit` message.
        // connect!(relm,
        //         window,
        //         connect_delete_event(_, _),
        //         return (Some(Msg::Quit), Inhibit(false)));
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback

        window.show_all();

        Win {
            model,
            window,
        }
    }
}

impl Update for Win {
    // when receive the keyboard events update the model

    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model::new()
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    fn update(&mut self, event: Msg) {
        match event {
            Msg::KeyDown => self.model.on_key_pressed(),
            Msg::Quit => gtk::main_quit(),
        }
    }
}
