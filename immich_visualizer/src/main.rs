// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};
use std::process::exit;
use std::env;
use openapi::models;
use rand::Rng;
use chrono::Utc;
use chrono::prelude::*; // Import the prelude to get the `Datelike` trait

mod immich;

extern crate openapi;
#[tokio::main]
async fn main() {

    let env_api_key = env::var("IMMICH_API_KEY");

    // Handle the Option
    let api_key = match env_api_key {
        Ok(value) => {
            println!("Environment variable value: {}", value);
            value
    },
        Err(e) => {
            eprintln!("Error reading environment variable: {}", e);
            exit(1)
        },
    };

    let client = immich::ApiClient::new("http://192.168.50.214:2283/api".to_string(),
        api_key.to_string(),
        false);
    let _res = match client.ping().await{
        Ok(response) => {
            println!("Ping response: {:?}", response);
            Ok(String::from("Immich connected."))
        },
        Err(e) => {
            eprintln!("Error calling /server-info/ping: {:?}", e);
            Err(String::from("Aaaaarrrh!"))
        },
    };

    eprintln!("Retrieving all assets...");
    let all_assets: Vec<models::AssetResponseDto> = client.get_all_assets().await.expect("F***");

    // Generate a random year between 1992 and the current year
    let current_year = Utc::now().year();
    let year = rand::thread_rng().gen_range(2012..=current_year);

    let filtered_assets: Vec<&models::AssetResponseDto> = all_assets.iter().filter(|&asset| {
        // eprintln!("YEAR: {}", asset.local_date_time);
        asset.file_created_at.starts_with(&year.to_string())
    }).collect();

    eprintln!("{} assets retrieved in year {}", filtered_assets.len(), year);


    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {

    component MemoryTile inherits Rectangle {
        width: 1280px;
        height: 720px;
        background: #3960D5;

        Image {
            source: @image-url("./resources/psyduck.png");
            width: parent.width;
            height: parent.height;
        }
    }

    export component MainWindow inherits Window {
        MemoryTile {}
    }
}