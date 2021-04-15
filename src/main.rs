use fltk::app;
use fltk::button::*;
use fltk::enums::Color;
use fltk::text::*;
use fltk::window::*;
use fltk::group;
use fltk::frame;
use std::cell::RefCell;

fn main() {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 100, 300, "FLTK testing");
    let mut count:RefCell<u32> = RefCell::new(0);

    let mut packing = group::Pack::default().with_size(100,300).center_of(&window);
    packing.set_type(group::PackType::Vertical);

    let mut button_inc = Button::default()
        .with_label("Increment!");





    let mut display = frame::Frame::default()
        //.center_of(&window)
        .with_label("0");


    let mut button_dec = Button::default()//.with_size(100,50)
        .with_label("decrement!");

/*
    button_inc.set_callback(|| {
        *count.borrow_mut()+=1;
        display.set_label(&(*count.borrow()).to_string());
    });


    button_dec.set_callback(|| {
        *count.borrow_mut()-=1;
        display.set_label(&(*count.borrow()).to_string());
    });

    button_dec.handle2(|b,event| match event{
        Event::Enter => {b.set_label("Decrement"); true},
        Event::Leave => {b.set_label("decrement"); true},
        _ => false,
    });
*/
    packing.auto_layout();
    packing.end();






    window.end();




    window.show();

    app.run().unwrap();
}
