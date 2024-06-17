extern crate openapi;
// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};

use openapi::apis::server_info_api::ping_server; // Replace with the actual module paths
use openapi::apis::assets_api::{download_asset, get_all_assets}; // Replace with the actual module paths
use openapi::apis::configuration::{ApiKey, Configuration};   // Replace with the actual module paths
use openapi::models;
use bytes::Bytes;
use futures::executor; 
use std::fs::File;
use std::io::Write;

pub struct ApiClient {
    config: Configuration,
    verbose:bool
}

impl ApiClient {
    // Constructor method to create a new ApiClient
    pub fn new(base_url: String, api_key: String, verbose: bool) -> Self {
        // Create a configuration instance for the API client
        let mut config = Configuration::new();
        config.base_path = base_url; // Replace with the actual base path of your API
        let apik = ApiKey {prefix: None, key: api_key };
        config.api_key = Some(apik);
        ApiClient { config, verbose }
    }

    // Method to set the base URL
    // pub fn set_base_url(&mut self, base_url: String) {
    //     self.config.base_path = base_url;
    // }

    // // Method to get the base URL
    // pub fn get_base_url(&self) -> &str {
    //     &self.config.base_path
    // }

    pub fn download_image(&self, id: String) -> Result<Bytes, String> {
        // Simulate an API call
        if self.config.base_path.is_empty() {
            return Err("Base URL is empty".to_string())
        }

        let message: Result<Bytes, String> = match executor::block_on(download_asset(&self.config, &id, None)) {
            Ok(response) => {
                if self.verbose {
                    let file_path = "./debug.jpg";

                    // Create a new file at the specified path
                    let mut file = File::create(file_path).expect("Unable to create file");
            
                    // Write the image bytes to the file
                    file.write_all(&response).expect("Unable to write data");
            
                    // Close the file
                    file.flush().expect("Unable to flush data");
                }
                Ok(response)
            },
            Err(e) => {
                if self.verbose {
                    eprintln!("Download response: {:?}", e);
                }
                Err(String::from("Aaaaarrrh!"))
            },
        };

        message
    }

    pub fn get_all_assets(&self) -> Result<Vec<models::AssetResponseDto>, String> {
        // Simulate an API call
        if self.config.base_path.is_empty() {
            return Err("Base URL is empty".to_string())
        }

        let message: Result<Vec<models::AssetResponseDto>, String> = match executor::block_on(get_all_assets(&self.config)) {
            Ok(response) => {
                if self.verbose {
                    // println!("Ping response: {:?}", response);
                }
                Ok(response)
            },
            Err(e) => {
                if self.verbose {
                    eprintln!("Ping response: {:?}", e);
                }
                Err(String::from("Aaaaarrrh!"))
            },
        };

        message
    }

    // Example method to simulate an API call
    pub fn ping(&self) -> Result<String, String> {
        // Simulate an API call
        if self.config.base_path.is_empty() {
            return Err("Base URL is empty".to_string())
        }

        let message = match executor::block_on(ping_server(&self.config)) {
            Ok(response) => {
                if self.verbose {
                    println!("Ping response: {:?}", response);
                }
                Ok(String::from("Immich connected."))
            },
            Err(e) => {
                if self.verbose {
                    eprintln!("Ping response: {:?}", e);
                }
                Err(String::from("Aaaaarrrh!"))
            },
        };

        message
    }
}
