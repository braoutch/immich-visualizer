use std::process::exit;
use std::env;
use std::time;
use std::thread;
use openapi::models;
use rand::Rng;
use chrono::Utc;
use chrono::prelude::*;
use rand::seq::SliceRandom;
use bytes::Bytes;
use image::ImageFormat;
use slint::SharedPixelBuffer;
use slint::Rgba8Pixel;

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

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
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
    let _res = match client.ping() {
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

    eprintln!("Retrieving all assets...");
    let all_assets: Vec<models::AssetResponseDto> = client.get_all_assets().expect("F***");

    /////////////////////////////////////////////
    // test
    ///////////////
    let current_year = Utc::now().year();
    let year = rand::thread_rng().gen_range(2012..=current_year);
    // Generate a random year between 1992 and the current year

    let mut filtered_assets: Vec<&models::AssetResponseDto> = all_assets.iter().filter(|&asset| {
        // eprintln!("YEAR: {}", asset.local_date_time);
        asset.file_created_at.starts_with(&year.to_string())
    }).collect();

    let mut rng = rand::thread_rng();
    filtered_assets.shuffle(&mut rng);

    for (_index, asset) in filtered_assets.iter().enumerate() {
        eprintln!("Asset: {:?}", asset.id.clone());
        let _image: (Bytes, String) = client.download_image(asset.id.clone()).expect("Nooo");
        let image_format = type_str_to_image_type(&_image.1).expect("Invalid image format");
        let _slint_image = bytes_to_image(_image.0, image_format).expect("Invalid image conversion");
        ui.set_image_source(_slint_image);
        break;
    }   
    /////////////////////////////////////////////
    // END TEST /////////////////////////////////////////////
    /////////////////////////////////////////////

    thread::spawn(move || {
        let current_year = Utc::now().year();
        let mut year;
        let mut switch_time = Utc::now().timestamp();
        let mut count = 0;
        loop {
            // beginning of the loop, get the current hour
            year = rand::thread_rng().gen_range(2012..=current_year);
            // Generate a random year between 1992 and the current year

            let mut filtered_assets: Vec<&models::AssetResponseDto> = all_assets.iter().filter(|&asset| {
                // eprintln!("YEAR: {}", asset.local_date_time);
                asset.file_created_at.starts_with(&year.to_string())
            }).collect();

            eprintln!("{} assets retrieved in year {}", filtered_assets.len(), year);

            let mut rng = rand::thread_rng();
            filtered_assets.shuffle(&mut rng);

            for (_index, asset) in filtered_assets.iter().enumerate() {
                eprintln!("Asset: {:?}", asset.id.clone());
                let image = client.download_image(asset.id.clone()).expect("Nooo");
                // let slint_image = bytes_to_image(image).expect("Invalid image conversion");
                
                // let ui = ui_handle.unwrap();
                // ui.set_image_source(slint_image);
                break;
            }            

            std::thread::sleep(time::Duration::from_millis(500));
            count += 1;
            if count > 3 {
                break;
            }

            if Utc::now().timestamp() - switch_time > 5 {
                switch_time = Utc::now().timestamp();
                break;
            }
        }
     });

     eprintln!("Running the UI...");

    ui.run()
}