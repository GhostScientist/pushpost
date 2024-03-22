use std::io::{self};
use serde::{Deserialize, Serialize};
use reqest;
use dirs;

#[derive(Serialize, Deserialize)]
struct Secrets {
    name: String,
    favorite_color: String,
    birthday: String,
}

fn main() {
    loop {
        println!("Select an option:");
        println!("1. Repeat after me");
        println!("2. Installation wizard");
        println!("3. Make API request");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = choice.trim().parse().expect("Invalid input");

        match choice {
            1 => repeat_after_me(),
            2 => installation_wizard(),
            3 => make_api_request(),
            4 => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn repeat_after_me() {
    println!("Enter a message (press Enter to finish):");
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read input");
    println!("You said: {}", message.trim());
}


fn installation_wizard() {
    let secrets_path = dirs::home_dir().unwrap().join(".cli_tool_secrets.json");
    
    if secrets_path.exists() {
        println!("Hey, we already got your secrets - wanna update them? (y/n)");
        let mut update_choice = String::new();
        io::stdin().read_line(&mut update_choice).expect("Failed to read input");
        
        if update_choice.trim().to_lowercase() != "y" {
            return;
        }
    }
    
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    
    println!("Enter your favorite color:");
    let mut favorite_color = String::new();
    io::stdin().read_line(&mut favorite_color).expect("Failed to read input");
    
    println!("Enter your birthday (YYYY-MM-DD):");
    let mut birthday = String::new();
    io::stdin().read_line(&mut birthday).expect("Failed to read input");
    
    let secrets = Secrets {
        name: name.trim().to_string(),
        favorite_color: favorite_color.trim().to_string(),
        birthday: birthday.trim().to_string(),
    };
    
    let serialized_secrets = serde_json::to_string(&secrets).expect("Failed to serialize secrets");
    std::fs::write(secrets_path, serialized_secrets).expect("Failed to write secrets to file");
    
    println!("Secrets stored successfully!");
}


fn make_api_request() {
    println!("Enter the URL to make a GET request:");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read input");
    
    let response = reqwest::blocking::get(url.trim()).expect("Failed to make API request");
    let status = response.status();
    let body = response.text().expect("Failed to read response body");
    
    println!("Response status: {}", status);
    println!("Response body: {}", body);
}