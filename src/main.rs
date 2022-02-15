use std::env;
use std::io::Cursor;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let link = match args.get(1){
        Some(value) => value,
        None => {println!("Please provide a link"); return;}
    };
    let filename = match args.get(2){
        Some(value) => value,
        None => {println!("Please provide a file name"); return;}
    };
    if link != "__None" && filename != "__None" {
    println!("Link: {}", link);
    println!("File name: {}", filename);
    match fetch_url(link.to_string(), filename.to_string()).await {
        Ok(_) => println!("Downloaded file"),
        Err(e) => println!("Error: {}", e)
    }
}}
