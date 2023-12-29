#![warn(clippy::all, clippy::pedantic)]

use std::{env, fs};
use rdev::{grab, Event, EventType, Key};
use screenshots::Screen;
use chrono::Utc;

const TARGET_DIR: &str = "screens";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let screen_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    let mut path = env::current_dir()?;
    path.push(&screen_dir);

    fs::create_dir_all(path)?;

    if let Err(_err) = grab(move |e| callback(e, &screen_dir)) {
        println!("something went wrong");
    }

    Ok(())
}

fn callback(event: Event, screen_dir: &String) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::PrintScreen) => {
            make_screenshot(screen_dir);
            None
        }
        _ => Some(event),
    }
}

fn make_screenshot(screen_dir: &String) {
    let screens = Screen::all().unwrap();

    for screen in screens {
        let image = screen.capture().unwrap();
        let now = Utc::now();
        image.save(format!("{screen_dir}/{}.png", now.format("%Y-%m-%d %H:%M:%S"))).unwrap();
    }
}