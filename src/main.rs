use std::{thread, time::Duration};

use chrono::Local;
use fltk::{app, enums::Color, frame::Frame, window::Window, GroupExt, WidgetExt, WindowExt};

fn main() {
    let (width, height) = app::screen_size();
    let width = width * 0.96;
    let height = height * 0.96;

    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(width as i32, height as i32)
        .center_screen()
        .with_label("Counter");
    wind.set_color(Color::from_u32(0x2d1301));
    wind.fullscreen(true);
    let mut frame = Frame::default()
        .with_size(width as i32, height as i32)
        .center_of(&wind)
        .with_label("11:11:11");
    frame.set_label_size(200);
    frame.set_label_color(Color::Red);

    wind.end();
    wind.show();

    let (tx, rx) = app::channel();

    thread::spawn(move || loop {
        let time = Local::now();
        tx.send(format!("{}", time.format("%H:%M:%S")));
        thread::sleep(Duration::from_secs(1));
    });

    while app.wait() {
        if let Some(s) = rx.recv() {
            frame.set_label(&s);
            frame.redraw();
        }
    }
}
