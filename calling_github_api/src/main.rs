use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers", owner = "rust-lang", repo = "rust");
    println!("Request URL: {}", request_url);

    let client = reqwest::Client::new();
    let response = client.get(&request_url).header(USER_AGENT, "reqwest").send().await?;

    let users: Vec<User> = response.json().await?;

    // Print the list of users who starred the Rust repository
    println!("Users who starred the Rust repository:");
    for user in &users{
        print!("{}: {}\n", user.id, user.login);
    }

    // Print the same but in debug mode
    println!("Users who starred the Rust repository: \n{:?}", users);
    Ok(())
}