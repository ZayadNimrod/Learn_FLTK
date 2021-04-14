use fltk::app;
use fltk::button::*;
use fltk::enums::Color;
use fltk::text::*;
use fltk::window::*;

fn main() {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 400, 300, "FLTK testing");
    //window.set_border(false);
    let mut my_window2 = Window::new(10, 10, 190, 280, "");
    my_window2.set_color(Color::Red);
    my_window2.end();

    let mut my_window3 = Window::new(200, 10, 190, 280, "");
    my_window3.set_color(Color::Green);

    let mut my_window4 = Window::new(0, 0, 190, 140, "");
    my_window4.set_color(Color::Black);
    my_window4.end();

    my_window3.end();
    window.end();

    let mut second_window = Window::new(110, 110, 160, 80, "");
    second_window.end();

    let mut button = Button::default()
        .with_label("Hello there!")
        .with_align(fltk::enums::Align::Center)
        .with_size(120, 30)
        .center_of(&second_window);
    button.set_callback(|| {
        fltk::dialog::alert(100, 100, "General Kenobi!");
    });
    second_window.add(&button);

    window.show();
    second_window.show();

    app.run().unwrap();
}
