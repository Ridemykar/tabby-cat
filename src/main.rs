fn main() {
    loop {
        if let Err(e) = open_tab() {
            println!("{:#?}", e);
            panic!("Failed to execute program!");
        }
    }
}


fn open_tab() -> Result<(), std::io::Error> {
    let tab_url: String = String::from("https://pornhub.com/gay");
    
    webbrowser::open(&tab_url)
}