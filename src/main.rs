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

    let mut clock_display = Frame::default()
        .with_size(width as i32, (height * 0.35) as i32)
        .center_of(&wind)
        .with_label("11:11:11");
    clock_display.set_label_size((height * 0.35) as i32);
    clock_display.set_label_color(Color::Red);

    let mut date_display = Frame::default()
        .with_size(width as i32, (height * 0.1) as i32)
        .right_of(&clock_display, 10)
        .with_label("01.01.1970");

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
            clock_display.set_label(&s);
            clock_display.redraw();
        }
    }
}
