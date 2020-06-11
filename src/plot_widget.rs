use std::rc::Rc;
use std::env::args;
use relm::{Relm, Update, Widget};
use relm_derive::Msg;
use relm_derive::{widget};

// use gio::prelude::*;
use gtk::prelude::*;
use gtk::DrawingArea;

use cairo::Context;
use plotters::prelude::*;

mod model;
use model::Model;

//  pub struct PlotWidget {

//  }

#[derive(Msg)]
pub enum Msg {
    Draw,
}

#[widget]
impl Widget for PlotWidget {
    fn init_view(&mut self) {
        self.model.draw_handler.init(&self.drawing_area);
    }

    fn model() -> Model {
        Model::new()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Draw => {
                // let context = create_context(&self.drawing_area);
                let context = self.model.draw_handler.get_context();

                // this depends on cairo-rs to ^0.8.1
                let root = CairoBackend::new(&context, (500, 500)).unwrap().into_drawing_area();

                root.fill(&WHITE).unwrap();

                let mut chart = ChartBuilder::on(&root)
                    .caption("This is a test", ("sans-serif", 20))
                    .margin(5)
                    .set_all_label_area_size(50)
                    // .x_label_area_size(40)
                    // .y_label_area_size(40)
                    .build_ranged(0f32..15f32, 0f32..15f32)
                    .unwrap();

                // let points = vec![(1.0,4.0),(2.0,2.0),(3.0,5.0),(4.0,10.0),(5.0,6.0),(6.0,7.0)];
                // chart.draw_series( PointSeries::of_element(
                //     points,
                //     5,
                //     ShapeStyle::from(&BLUE).filled(),
                //     &|coord, size, style| {
                //         return EmptyElement::at(coord)    // We want to construct a composed element on-the-fly
                //         + Circle::new((0,0),size,style.filled()) // At this point, the new pixel coordinate is established
                //         + Text::new(format!("{:?}", coord), (10, 0), ("sans-serif", 10).into_font());
                //     }
                // )).expect("noooooooo");

                let points = self.model.get_points_to_draw().iter().copied();
                chart.draw_series( LineSeries::new(
                    points,
                    ShapeStyle::from(&BLUE).filled(),
                )).expect("Failed to draw the points in the chart.");


                chart.configure_mesh()
                    .draw()
                    .unwrap();

                Inhibit(false);
            },

        }
    }
    
    view! {
        #[name="drawing_area"]
        gtk::DrawingArea {
            child: {
                expand: true,
            },
            draw(_, _) => (Msg::Draw, gtk::Inhibit(false)),
        }
    }
}

// fn create_context(widget: &gtk::DrawingArea) -> Context {
//     let mut draw_handler = relm::DrawHandler::new().expect("draw handler");
//     draw_handler.init(widget);
//     draw_handler.get_context().clone()
// }