use std::env;
use std::io::Cursor;
use std::panic;

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
    panic::set_hook(Box::new(|_info| {
        println!("Invalid input");
    }));
    let args: Vec<String> = env::args().collect();
    let link = &args[1];
    let file_name = &args[2];
    if !link.is_empty() && !file_name.is_empty() {
        println!("Link: {}", link);
        println!("File name: {}", file_name);
        fetch_url(link.to_string(), file_name.to_string())
            .await
            .unwrap();
    }
}