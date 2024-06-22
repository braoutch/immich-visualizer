use std::process::exit;
use std::env;
use std::time;
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
fn bytes_to_image(bytes: Bytes, format: ImageFormat) -> Result<slint::Image, image::ImageError> {
    let img = image::load_from_memory_with_format(&bytes, format)?;
    let rgba_img = img.to_rgba8();

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        rgba_img.as_raw(),
        rgba_img.width(),
        rgba_img.height(),
    );

    Ok(slint::Image::from_rgba8(buffer))
}

// Function to convert Bytes to an image that Slint can display
fn bytes_to_shared_image(bytes: Bytes, format: ImageFormat) -> Result<SharedPixelBuffer<Rgba8Pixel>, image::ImageError> {
    let img = image::load_from_memory_with_format(&bytes, format)?;
    let rgba_img = img.to_rgba8();

    let buffer: SharedPixelBuffer<Rgba8Pixel> = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        rgba_img.as_raw(),
        rgba_img.width(),
        rgba_img.height(),
    );

    Ok(buffer)
}

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {

    // let rt = Runtime::new().unwrap();
    // let handle = rt.handle().clone();

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
    // ui.set_image_source(@image-url("./resources/psyduck.png"))

    let ui_handle = ui.as_weak();

    
    /////////////////////////////////////////////
    // test
    ///////////////
    eprintln!("Retrieving all assets...");
    // let all_assets: Vec<models::AssetResponseDto> = client.get_all_assets().expect("F***");
    let random_asset: Vec<models::AssetResponseDto> = client.get_random_asset().await.expect("F***");
    // print the number of items in random_asset
    eprintln!("{} assets retrieved", random_asset.len());

    for (_index, asset) in random_asset.iter().enumerate() {
        if asset.original_mime_type != "image/jpeg" {
            eprintln!("Skip image format: {:?}", asset.original_mime_type);
            continue;
        }
        eprintln!("Image format: {:?}", asset.original_mime_type);


        eprintln!("Asset: {:?}", asset.id.clone());
        let _image: (Bytes, String) = client.download_image(asset.id.clone()).await.expect("Nooo");
        // check the type of the image and, if it's not jpeg, continue the loop
        let image_format = type_str_to_image_type(&_image.1);
        if image_format != Some(ImageFormat::Jpeg) {
            eprintln!("Skip image format: {:?}", image_format);
            continue;
        }

        let _slint_image = bytes_to_image(_image.0, image_format.unwrap()).expect("Invalid image conversion");
        ui.set_image_source(_slint_image);
        break;
    }   
    /////////////////////////////////////////////
    // END TEST /////////////////////////////////////////////
    /////////////////////////////////////////////

    tokio::spawn(async move {
        let mut count = 0;
        loop {
            
            let client = immich::ApiClient::new("http://192.168.50.214:2283/api".to_string(), api_key.to_string(), true);
            eprintln!("PING {}...", count);
            let start_time = Instant::now();
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
                if asset.original_mime_type != "image/jpeg" {
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
                let elapsed_time = start_time.elapsed();
                eprintln!("Download execution time: {:?}", elapsed_time);

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

                let pixel_buffer = match bytes_to_shared_image(_image.0, image_format.unwrap()) {
                    Ok(response) => response,
                    Err(e) => {
                        eprintln!("Error converting image: {:?}. Skipping.", e);
                        continue;
                    },
                };

                // let ui = ui_handle.unwrap();
                let handle_copy = ui_handle.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    let image = slint::Image::from_rgba8_premultiplied(pixel_buffer);
                    handle_copy.unwrap().set_image_source(image);
                });
                // let handle_copy = ui_handle.clone();
                // let _ = slint::invoke_from_event_loop(move || handle_copy.unwrap().set_image_text(String::from("Hello, world!").into()));
                break;
            }   
        
            std::thread::sleep(time::Duration::from_millis(2000));
            count += 1;
        }
     });

     eprintln!("Running the UI...");

    ui.run()
}