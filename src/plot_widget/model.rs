use relm::{Relm, Widget, connect, DrawHandler};
use gtk::{
    BoxExt,
    DrawingArea,
    Inhibit,
    OrientableExt,
    WidgetExt,
    prelude::WidgetExtManual,
};
use gdk::{EventMask, RGBA};

pub struct Model {
    pub draw_handler: DrawHandler<DrawingArea>,
    points_to_draw: Vec<(f32, f32)>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            draw_handler: DrawHandler::new().expect("draw handler"),
            points_to_draw: Vec::new(),
        }
    }

    // pub fn add_point_to_draw(&mut self, p: (f32, f32)) {
    //     self.points_to_draw.push(p);
    // }

    pub fn get_points_to_draw(&self) -> &Vec<(f32, f32)> {
        &(self.points_to_draw)
    }
}