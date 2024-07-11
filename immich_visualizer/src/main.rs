use std::env;
use std::process::exit;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    main().unwrap();
}

#[tokio::main]
pub async fn main() -> Result<(), slint::PlatformError> {

    // Handle the Option
    let api_key = match env::var("IMMICH_API_KEY") {
        Ok(value) => {
            println!("Environment variable value: {}", value);
            value
        }
        Err(e) => {
            eprintln!("Error reading environment variable: {}", e);
            exit(1)
        }
    };

    immich_visualizer::run(api_key).await
}
