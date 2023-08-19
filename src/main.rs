use reqwest;
use std::fs::File;
use std::io::{self, Read};

fn fetch_token_from_file() -> Result<String, io::Error> {
    let mut file = File::open("access_token.txt")?;
    let mut token = String::new();
    file.read_to_string(&mut token)?;
    Ok(token.trim().to_owned())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token=  match fetch_token_from_file() {
        Ok(token) => token,
        Err(err) => {
            panic!("Error reading 'access_token.txt' file: {}", err);
        }
    };
    
    let api_url = "https://slack.com/api/users.list";

    let client = reqwest::Client::new();
    let response = client
        .get(api_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    // process the response
    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());
    println!("Body:\n{}", response.text().await?);

    Ok(())
}