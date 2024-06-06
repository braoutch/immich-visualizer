use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use std::process::exit;
use std::env;
mod immich;

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

    let mut client = immich::ApiClient::new("http://192.168.50.214:2283/api".to_string(), api_key.to_string(), true);
    let res = match client.ping().await{
        Ok(response) => {
            println!("Ping response: {:?}", response);
            Ok(String::from("Immich connected."))
        },
        Err(e) => {
            eprintln!("Error calling /server-info/ping: {:?}", e);
            Err(String::from("Aaaaarrrh!"))
        },
    };

    let res = match client.get_all_assets().await{
        Ok(response) => {
            println!("Ping response: {:?}", response);
            Ok(String::from("Immich connected."))
        },
        Err(e) => {
            eprintln!("Error calling /server-info/ping: {:?}", e);
            Err(String::from("Aaaaarrrh!"))
        },
    };

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