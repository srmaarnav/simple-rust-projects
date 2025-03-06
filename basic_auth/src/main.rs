use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user = "test_user".to_string();
    let passwd: Option<String> = None;

    let response = client.get("https://httpbin.org/").basic_auth(user, passwd).send();

    println!("{:?}", response);

    Ok(())
}