use std::fs::File;
use std::io::BufReader;
use std::{thread, time::Duration};

use chrono::{DateTime, Local, Timelike};
use fltk::{app, enums::Color, frame::Frame, window::Window, GroupExt, WidgetExt, WindowExt};
use rodio::Source;

use crate::config::{AlarmTime, Config};

mod config;

fn main() -> anyhow::Result<()> {
    let Config {
        alarm_time,
        audio_path,
    } = config::read()?;

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

    let (_clock_width, clock_height) = clock_display.measure_label();
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

    let mut previous_time = Local::now();
    let mut playing_alarm = None;
    while app.wait() {
        if let Some(current_time) = rx.recv() {
            clock_display.set_label(&format!("{}", current_time.format("%H:%M")));
            seconds_display.set_label(&format!("{}", current_time.format("%S")));
            date_display.set_label(&format!("{}", current_time.format("%-d.%-m.%Y")));

            if previous_time.minute() != current_time.minute() {
                if check_alarm(&current_time, &alarm_time) {
                    println!("Playing alarm");
                    playing_alarm = Some(play_alarm(&audio_path));
                }
            }

            previous_time = current_time;
        }
    }

    Ok(())
}

fn calculate_seconds_x(width: f64) -> i32 {
    (width * 0.84) as i32
}

fn calculate_seconds_y(y: i32, height: i32) -> i32 {
    y + (height as f64 / 1.8) as i32
}

fn check_alarm(current_time: &DateTime<Local>, alarm_time: &AlarmTime) -> bool {
    let current_hour = current_time.hour();
    let current_minute = current_time.minute();

    current_hour == alarm_time.hour as u32 && current_minute == alarm_time.minute as u32
}

fn play_alarm(alarm_path: &str) -> (rodio::OutputStream, rodio::OutputStreamHandle) {
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // TODO load all files in directory
    let file = File::open(alarm_path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    stream_handle.play_raw(source.convert_samples());

    (stream, stream_handle)
}
