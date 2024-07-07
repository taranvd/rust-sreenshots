#![warn(clippy::all, clippy::pedantic)]

use chrono::{Utc};  // Importing Utc from chrono for date/time operations
use rdev::{grab, Event, EventType, Key};  // Importing grab, Event, EventType, and Key from rdev
use screenshots::Screen;  // Importing Screen from screenshots crate
use std::env;  // Importing env module from standard library
use std::fs;   // Importing fs module from standard library for file system operations

const TARGET_DIR: &str = "screens";  // Constant for default target directory name

fn main() -> std::io::Result<()> {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Determine the directory where screenshots will be saved
    let screens_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();

    // Get the current directory path
    let mut path = env::current_dir()?;

    // Append the target directory name to the current directory path
    path.push(&screens_dir);

    // Create the directory and all its parents if it doesn't exist
    fs::create_dir_all(path)?;

    // Start grabbing events; handle errors if any occur
    if let Err(error) = grab(move |e| callback(e, &screens_dir)) {
        println!("Error: {error:?}");
    }

    Ok(())
}

// Callback function that handles events
fn callback(event: Event, screens_dir: &String) -> Option<Event> {
    // Match on the event type
    match event.event_type {
        // If the event is a KeyPress and the key is F10
        EventType::KeyPress(Key::F10) => {
            // Call function to capture screens and save them
            make_screen(screens_dir);
            None  // Return None to consume the event
        }
        _ => Some(event),  // Return Some(event) to propagate the event
    }
}

// Function to capture screens and save them as images
fn make_screen(screens_dir: &String) {
    // Get all screens available for capture
    let screens = Screen::all().unwrap();

    // Iterate through each screen
    for screen in screens {
        // Capture the screen as an image
        let image = screen.capture().unwrap();

        // Get the current date and time in UTC
        let now = Utc::now();

        // Save the captured image with a formatted filename
        image
            .save(format!(
                "{}/{}.png",
                screens_dir,
                now.format("%d-%m-%Y_%H_%M_%S")
            ))
            .unwrap()  // Unwrap the result; panic if saving fails
    }
}
