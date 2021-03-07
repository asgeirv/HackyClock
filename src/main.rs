use std::{thread, time::Duration};

use chrono::{Local, Timelike};
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
        .with_label("11:11");
    clock_display.set_label_size((height * 0.35) as i32);
    clock_display.set_label_color(Color::Red);
    let (clock_width, clock_height) = clock_display.measure_label();
    println!("{}, {}, {}", clock_display.y(), clock_width, clock_height);

    let secs_x = calculate_seconds_x(width);
    let secs_y = calculate_seconds_y(clock_display.y(), clock_height);
    let mut seconds_display = Frame::default()
        .with_size((width * 0.07) as i32, (height * 0.07) as i32)
        .with_pos(secs_x, secs_y)
        .with_label("11");
    seconds_display.set_label_size((height * 0.12) as i32);
    seconds_display.set_label_color(Color::Red);

    let mut date_display = Frame::default()
        .with_size(width as i32, (height * 0.07) as i32)
        .below_of(&clock_display, (height * 0.2) as i32)
        .with_label("01.01.1970");
    date_display.set_label_color(Color::Red);
    date_display.set_label_size((height * 0.07) as i32);

    wind.end();
    wind.show();

    let (tx, rx) = app::channel();

    thread::spawn(move || loop {
        tx.send(Local::now());
        thread::sleep(Duration::from_secs(1));
    });

    while app.wait() {
        if let Some(time) = rx.recv() {
            clock_display.set_label(&format!("{}", time.format("%H:%M")));
            seconds_display.set_label(&format!("{}", time.format("%S")));
            date_display.set_label(&format!("{}", time.format("%-d.%-m.%Y")));
        }
    }

    std::process::exit(0);
}

fn calculate_seconds_x(width: f64) -> i32 {
    (width * 0.82) as i32
}

fn calculate_seconds_y(y: i32, height: i32) -> i32 {
    y + (height as f64 / 1.65) as i32
}
