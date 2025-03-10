use error_chain::error_chain;
use std::io::{copy, Cursor};
use std::fs::File;
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?; 
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

    let response = reqwest::get(target).await?;
    
    let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");
    
    println!("File to download: '{}'", fname);
    let fname = tmp_dir.path().join(fname);

    println!("File is to be located at: '{:?}'", fname);
    let mut file = File::create(fname)?; // Fix: Assign `File` to `file`

    let content = response.bytes().await?; // Get bytes instead of text
    let mut cursor = Cursor::new(content); // Convert bytes to `Read`
    copy(&mut cursor, &mut file)?; // Copy bytes to the file

    Ok(())
}
