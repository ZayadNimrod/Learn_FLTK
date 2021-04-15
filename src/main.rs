use fltk::app;
use fltk::button::*;
use fltk::enums::Color;
use fltk::text::*;
use fltk::window::*;
use fltk::group;
use fltk::frame;
use std::cell::RefCell;
use fltk::image;



fn main() {



    let mut image = image::PngImage::load("logo.png").unwrap();
    let app = app::App::default();
    app.with_scheme(app::Scheme::Base);
    let mut window = Window::new(100, 100, 400, 300, "FLTK testing");

    let mut frame = frame::Frame::new(0, 0, 400, 300, "");

    frame.draw2(move |f| {
        image.scale(f.width()/2, f.height()/2, true, true);
        image.draw(f.x(), f.y(), image.width(), image.height());
        image.draw(100, 100, image.width(), image.height());
        image.draw(200, 0, image.width(), image.height());

        image.scale(f.width(), f.height(), true, true);
        image.draw(20,0,image.width(),image.height());
    });






    window.end();




    window.show();

    app.run().unwrap();
}
