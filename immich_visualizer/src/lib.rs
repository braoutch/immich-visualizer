use std::time;
use std::time::Instant;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
slint::include_modules!();

use bytes::Bytes;
use image::ImageFormat;
use slint::Rgba8Pixel;
use slint::SharedPixelBuffer;

pub mod immich;
use openapi::models;

mod heif_utils;

// import all function from the heif_utils module
pub use crate::heif_utils::heif_utils_mod;

fn construct_ui(tx_ui_image_duration: Sender<i32>, tx_ui_enable_png: Sender<bool>) -> MyWindow {
    let ui = MyWindow::new().unwrap();
    ui.window().set_fullscreen(true);

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

    tx_ui_image_duration.send(ui.get_duration_value()).unwrap();
    tx_ui_enable_png.send(ui.get_png_value()).unwrap();
    eprintln!("Initial duration: {}", ui.get_duration_value());
    eprintln!("Initial PNG: {}", ui.get_png_value());
    ui
}

pub async fn run(api_key: String) -> Result<(), slint::PlatformError> {

    let client = immich::ApiClient::new(
        "http://192.168.50.214:2283/api".to_string(),
        api_key.to_string(),
        true,
    );
    let _res = match client.ping().await {
        Ok(_response) => {
            // println!("Ping response: {:?}", _response);
            Ok(String::from("Immich connected."))
        }
        Err(e) => {
            eprintln!("Error calling /server-info/ping: {:?}", e);
            Err(String::from("Aaaaarrrh!"))
        }
    };

    // Create a channel to communicate with the event loop
    let (tx_ui_image_duration, rx_ui_image_duration) = mpsc::channel();
    let (tx_ui_enable_png, rx_ui_enable_png) = mpsc::channel();
    
    let ui: MyWindow = construct_ui(tx_ui_image_duration, tx_ui_enable_png);
    let ui_handle = ui.as_weak();

    tokio::spawn(async move {
        // let mut count = 0;
        let mut wait_duration = rx_ui_image_duration.recv().unwrap();
        let mut enable_png = rx_ui_enable_png.recv().unwrap();
        let mut start_time: Instant = Instant::now();
        loop {
            // eprint!("Loop again...");
            let client = immich::ApiClient::new(
                "http://192.168.50.214:2283/api".to_string(),
                api_key.to_string(),
                false,
            );

            // eprintln!("Loop >> {} {:?}...", count, start_time);
            let random_asset: Vec<models::AssetResponseDto> = match client.get_random_asset(30).await
            {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("Error retrieving random asset: {:?}. Skipping.", e);
                    continue;
                }
            };

            for (_index, asset) in random_asset.iter().enumerate() {
                let mut compatibility_list = vec!["image/jpeg", "image/heic"];
                // let mut compatibility_list = vec!["image/heic"];

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
                // eprintln!("Image format: {:?}", asset.original_mime_type);

                // eprintln!("Asset: {:?}", asset.id.clone());
                let _image: (Bytes, String) = match client.download_image(asset.id.clone()).await {
                    Ok(response) => response,
                    Err(e) => {
                        eprintln!("Error downloading image: {:?}. Skipping.", e);
                        continue;
                    }
                };
                // eprintln!("DL execution time: {:?}", start_time.elapsed());

                // check the type of the image and, if it's not jpeg, continue the loop
                let image_format = match heif_utils_mod::type_str_to_image_type(&_image.1) {
                    Some(ImageFormat::Jpeg) => ImageFormat::Jpeg,
                    Some(ImageFormat::Png) => ImageFormat::Png,
                    Some(ImageFormat::OpenExr) => ImageFormat::OpenExr,
                    _ => {
                        // print the error
                        eprintln!("Unsupported format {}", &_image.1);
                        continue;
                    }
                };
                // eprintln!("Image prep 1 execution time: {:?}", start_time.elapsed());
                let pixel_buffer: SharedPixelBuffer<Rgba8Pixel> =  match heif_utils_mod::open_image_with_format(&_image.0, image_format){
                    Ok(response) => response,    
                    Err(_e) => {
                        eprintln!("Error opening the image");
                        continue;
                    }
                };

                // eprintln!("Image prep 2 execution time: {:?}", start_time.elapsed());

                // let ui = ui_handle.unwrap();
                let handle_copy = ui_handle.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    let image = slint::Image::from_rgba8_premultiplied(pixel_buffer);
                    handle_copy.unwrap().set_image_source(image);
                    eprint!("Image updated after {:?}", start_time.elapsed());
                });
                start_time = Instant::now();
                break;
            }

            // Attempt to get the latest message by draining the channel
            while let Ok(val) = rx_ui_image_duration.try_recv() {
                wait_duration = val;
            }

            // eprintln!("Sleeping for {} seconds...", wait_duration);
            std::thread::sleep(time::Duration::from_millis((wait_duration as u64) * 1000));
            // count += 1;
        }
    });

    eprintln!("Running the UI...");
    let err: Result<(), slint::PlatformError> =  ui.run();
    err
}