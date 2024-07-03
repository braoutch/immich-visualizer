use std::process::exit;
use std::env;
use std::time;
use std::sync::mpsc;
// use std::thread;
// use tokio::runtime::Runtime;

use openapi::models;
// use rand::Rng;
// use chrono::Utc;
// use chrono::prelude::*;
// use rand::seq::SliceRandom;
use bytes::Bytes;
use image::ImageFormat;
use slint::SharedPixelBuffer;
use slint::Rgba8Pixel;
use std::time::Instant;

mod immich;

extern crate openapi;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32",
wasm_bindgen::prelude::wasm_bindgen(start))]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    main().unwrap();
}

fn type_str_to_image_type(type_str: &str) -> Option<ImageFormat> {
    match type_str {
        "image/jpeg" => Some(ImageFormat::Jpeg),
        "image/png" => Some(ImageFormat::Png),
        "image/gif" => Some(ImageFormat::Gif),
        "image/bmp" => Some(ImageFormat::Bmp),
        "image/tiff" => Some(ImageFormat::Tiff),
        "image/x-icon" => Some(ImageFormat::Ico),
        _ => None,
    }
}

// Function to convert Bytes to an image that Slint can display
fn bytes_to_shared_image(bytes: &Bytes, format: ImageFormat) -> Result<SharedPixelBuffer<Rgba8Pixel>, image::ImageError> {
    // let start_time = Instant::now();
    let img = image::load_from_memory_with_format(&bytes, format)?;
    // eprintln!("1 execution time: {:?}", start_time.elapsed());
    let rgba_img = img.to_rgba8();
    // eprintln!("2 execution time: {:?}", start_time.elapsed());

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        rgba_img.as_raw(),
        rgba_img.width(),
        rgba_img.height(),
    );
    Ok(buffer)
}

#[tokio::main]
pub async fn main() -> Result<(), slint::PlatformError> {

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

    let client = immich::ApiClient::new("http://192.168.50.214:2283/api".to_string(), api_key.to_string(), true);
    let _res = match client.ping().await {
        Ok(response) => {
            println!("Ping response: {:?}", response);
            Ok(String::from("Immich connected."))
        },
        Err(e) => {
            eprintln!("Error calling /server-info/ping: {:?}", e);
            Err(String::from("Aaaaarrrh!"))
        },
    };

    let ui = MyWindow::new()?;
    ui.window().set_fullscreen(true);

    // Create a channel to communicate with the event loop
    let (tx_ui_image_duration, rx_ui_image_duration) = mpsc::channel();
    let (tx_ui_enable_png, rx_ui_enable_png) = mpsc::channel();
    let tx1_ui_image_duration = tx_ui_image_duration.clone();
    let tx1_ui_enable_png = tx_ui_enable_png.clone();

    ui.on_settings_clicked({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            // ui.set_counter(ui.get_counter() + 1);
            eprintln!("Settings clicked: {}", ui.get_state());
        }
    });

    ui.on_image_duration_value_changed({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            eprintln!("Duration value changed: {}", ui.get_duration_value());
            tx1_ui_image_duration.send(ui.get_duration_value()).unwrap();
        }
    });

    ui.on_png_settings_changed({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            eprintln!("PNG enabled: {}", ui.get_png_value());
            tx1_ui_enable_png.send(ui.get_png_value()).unwrap();
        }
    });

    let ui_handle = ui.as_weak();
    tx_ui_image_duration.send(ui.get_duration_value()).unwrap();
    tx_ui_enable_png.send(ui.get_png_value()).unwrap();

    tokio::spawn(async move {
        let mut count = 0;
        let mut wait_duration = rx_ui_image_duration.recv().unwrap();
        let mut enable_png = rx_ui_enable_png.recv().unwrap();
        loop {
            let start_time = Instant::now();
            let client = immich::ApiClient::new("http://192.168.50.214:2283/api".to_string(), api_key.to_string(), true);
            
            eprintln!("Loop >> {} {:?}...", count, start_time);
            let random_asset: Vec<models::AssetResponseDto> = match client.get_random_asset().await {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("Error retrieving random asset: {:?}. Skipping.", e);
                    continue;
                },
            };

            // print the number of items in random_asset
            eprintln!("{} assets retrieved", random_asset.len());

            for (_index, asset) in random_asset.iter().enumerate() {
                // let mut compatibility_list = vec!["image/jpeg", "image/heic"];
                let mut compatibility_list = vec!["image/jpeg"];

                // should we skip pngs? Can be good to skip pngs becauses these are usually shitty screenshots of phones
                while let Ok(val) = rx_ui_enable_png.try_recv() {
                    enable_png = val;
                }

                if enable_png {
                    compatibility_list.push("image/png");
                }

                if !compatibility_list.contains(&asset.original_mime_type.as_str()) {
                    eprintln!("Skip image format: {:?}", asset.original_mime_type);
                    continue;
                }
                eprintln!("Image format: {:?}", asset.original_mime_type);

                eprintln!("Asset: {:?}", asset.id.clone());
                let _image: (Bytes, String) = match client.download_image(asset.id.clone()).await {
                    Ok(response) => response,
                    Err(e) => {
                        eprintln!("Error downloading image: {:?}. Skipping.", e);
                        continue;
                    },
                };
                // eprintln!("DL execution time: {:?}", start_time.elapsed());

                // check the type of the image and, if it's not jpeg, continue the loop
                let image_format = match type_str_to_image_type(&_image.1) {
                    Some(ImageFormat::Jpeg) => Some(ImageFormat::Jpeg),
                    Some(ImageFormat::Png) => Some(ImageFormat::Png),
                    _ => { 
                        // print the error
                        eprintln!("Unsupported format {}", &_image.1);
                        continue 
                    },                 
                };
                // eprintln!("Image prep 1 execution time: {:?}", start_time.elapsed());

                let pixel_buffer = match bytes_to_shared_image(&_image.0, image_format.unwrap()) {
                    Ok(response) => response,
                    Err(e) => {
                        eprintln!("Error converting image: {:?}. Skipping.", e);
                        continue;
                    },
                };
                // eprintln!("Image prep 2 execution time: {:?}", start_time.elapsed());

                // let ui = ui_handle.unwrap();
                let handle_copy = ui_handle.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    let image = slint::Image::from_rgba8_premultiplied(pixel_buffer);
                    handle_copy.unwrap().set_image_source(image);
                });
                break;
            }   

            // Attempt to get the latest message by draining the channel
            while let Ok(val) = rx_ui_image_duration.try_recv() {
                wait_duration = val;
            }
            
            eprintln!("Sleeping for {} seconds...", wait_duration);
            std::thread::sleep(time::Duration::from_millis((wait_duration as u64) * 1000));
            count += 1;
        }
     });

     eprintln!("Running the UI...");

    ui.run()
}