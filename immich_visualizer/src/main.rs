// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};
use std::process::exit;
use std::env;
use openapi::models;
use rand::Rng;
use chrono::Utc;
use chrono::prelude::*; // Import the prelude to get the `Datelike` trait
use rand::seq::SliceRandom;
use std::sync::Arc;
use bytes::Bytes;
use image::ImageFormat;
use slint::SharedPixelBuffer;
use slint::Rgba8Pixel;

mod immich;

extern crate openapi;


// use image::Rgb;
// use slint::Image;
// use slint::Rgb8Pixel;
// use slint::SharedPixelBuffer;
slint::include_modules!();

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    main().unwrap();
}

// Function to convert Bytes to an image that Slint can display
fn bytes_to_image(bytes: Bytes) -> Result<slint::Image, image::ImageError> {
    let img = image::load_from_memory_with_format(&bytes, ImageFormat::Jpeg)?;
    let rgba_img = img.to_rgba8();

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        rgba_img.as_raw(),
        rgba_img.width(),
        rgba_img.height(),
    );

    Ok(slint::Image::from_rgba8(buffer))
}

fn display_image(url: &str) {

    // let image = image::load_from_memory(&img_bytes)?;
    let _image_data = reqwest::blocking::get(url)
        .expect("Failed to download image")
        .bytes()
        .expect("Failed to read image bytes");

    // let window = Arc::new(MainWindow::new().unwrap());
    // window.set_image(slint::Image::load_from_memory(&image_data).unwrap());
    // window.run().unwrap();
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

    let client = immich::ApiClient::new("http://192.168.50.214:2283/api".to_string(),
        api_key.to_string(),
        true);
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

    let ui = MyWindow::new()?;
    let ui_handle = ui.as_weak();

    eprintln!("Retrieving all assets...");
    let all_assets: Vec<models::AssetResponseDto> = client.get_all_assets().await.expect("F***");

    let current_year = Utc::now().year();
    let mut year = rand::thread_rng().gen_range(2012..=current_year);
    let mut switch_time = 0;
    loop {
        // beginning of the loop, get the current hour
        if Utc::now().timestamp() - switch_time > 5 {
            year = rand::thread_rng().gen_range(2012..=current_year);
            switch_time = Utc::now().timestamp();
        }
        
        // Generate a random year between 1992 and the current year

        let mut filtered_assets: Vec<&models::AssetResponseDto> = all_assets.iter().filter(|&asset| {
            // eprintln!("YEAR: {}", asset.local_date_time);
            asset.file_created_at.starts_with(&year.to_string())
        }).collect();

        eprintln!("{} assets retrieved in year {}", filtered_assets.len(), year);
        
        let mut rng = rand::thread_rng();
        filtered_assets.shuffle(&mut rng);

        for (_index, asset) in filtered_assets.iter().enumerate() {

            // let index = rand::thread_rng().gen_range(0..=filtered_assets.len()-1);
            let image = client.download_image(asset.id.clone()).await.expect("Nooo");

            let slint_image = bytes_to_image(image).expect("Invalid image conversion");
            // let img = image::load_from_memory_with_format(&image, ImageFormat::Jpeg)?;

            ui.set_image_source(slint_image);
            // println!("Image downloaded: {}", _image.display());
        }

    }
    // let _image_data = reqwest::blocking::get("url")
    // .expect("Failed to download image")
    // .bytes()
    // .expect("Failed to read image bytes");

    // MainWindow::new().unwrap().run().unwrap();


    
        // ui.on_text_is_edited(move || {
        //     let ui = ui_handle.unwrap();
        //     match QrCode::new(ui.get_thetext()) {
        //         Ok(code) => {
        //             let s = String::from(ui.get_thetext());
        //             let charcount = s.chars().count();
        //             if charcount > 500 {
        //                 ui.set_qr_size(500);
        //             } else if charcount > 150 {
        //                 ui.set_qr_size(300);
        //             }
        //             ui.set_charcount(charcount.try_into().unwrap());
        //             ui.set_errormsg("".try_into().unwrap());
        //             let image = code.render::<Rgb<u8>>().build();
        //             let pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
        //                 image.as_raw(),
        //                 image.width(),
        //                 image.height(),
        //             );
        //             ui.set_qrnote(Image::from_rgb8(pixel_buffer));
    
        //         }
        //         Err(e) => ui.set_errormsg(e.to_string().into()),
        //     }
        // });
    
        ui.run()




}

// slint::slint! {

//     component MemoryTile inherits Rectangle {
//         width: 1280px;
//         height: 720px;
//         background: #3960D5;

//         // export property <image> image_source;

//         Image {
//             source: @image-url("./resources/psyduck.png");
//             // source: root.image_source;
//             width: parent.width;
//             height: parent.height;
//         }
//     }

//     export component MainWindow inherits Window {
//         in property <image> image_source;
//         MemoryTile {
//             // image_source: root.image_source;
//         }
//     }
// }