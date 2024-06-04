use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
extern crate openapi;

use openapi::apis::server_info_api::ping_server; // Replace with the actual module paths
use openapi::apis::configuration::Configuration;   // Replace with the actual module paths

async fn ping() {
    let stdout = stdout();

    // ping immich
    // Create a configuration instance for the API client
    let mut config = Configuration::new();
    config.base_path = "http://192.168.50.214:2283/api".to_string(); // Replace with the actual base path of your API
    
    let message = match ping_server(&config).await {
        Ok(response) => {
            println!("Ping response: {:?}", response);
            String::from("Immich connected.")
        },
        Err(e) => {
            eprintln!("Error calling /server-info/ping: {:?}", e);
            String::from("Aaaaarrrh!")
        },
    };

    // let message = String::from("Hello, crustacean!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

}

#[tokio::main]
async fn main() {
    ping().await;
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