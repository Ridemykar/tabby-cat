#[tokio::main]
async fn main() {
    loop {
        if let Err(e) = open_tab().await {
            println!("{:#?}", e);
            panic!("Failed to execute program!");
        }
    }
}


async fn open_tab() -> Result<(), std::io::Error> {
    let tab_url: String = String::from("https://pornhub.com/gay");
    
    webbrowser::open(&tab_url)
}