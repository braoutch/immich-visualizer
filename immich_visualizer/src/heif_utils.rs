
// use libheif_rs::HeifError;
use libheif_rs::{ColorSpace, HeifContext, ItemId, LibHeif, RgbChroma};
use slint::Rgba8Pixel;
use slint::SharedPixelBuffer;
use bytes::Bytes;
use image::ImageFormat;

// Function to convert Bytes to an image that Slint can display
pub fn bytes_to_shared_image(
    bytes: &Bytes,
    format: ImageFormat,
) -> Result<SharedPixelBuffer<Rgba8Pixel>, image::ImageError> {
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

pub fn safe_read_and_decode_heic_memory(
    bytes: &[u8],
) -> Result<SharedPixelBuffer<Rgba8Pixel>, String> {
    let lib_heif = LibHeif::new();
    let ctx = match HeifContext::read_from_bytes(bytes) {
        Ok(ctx) => ctx,
        Err(e) => return Err(format!("Error reading HEIC: {:?}", e)),
    };
    let handle = match ctx.primary_image_handle() {
        Ok(handle) => handle,
        Err(e) => return Err(format!("No primary image found:: {:?}", e)),
    };
    // assert_eq!(handle.width(), 1652);
    // assert_eq!(handle.height(), 1791);

    // Get Exif
    let mut meta_ids: Vec<ItemId> = vec![0; 1];
    let count = handle.metadata_block_ids(&mut meta_ids, b"Exif");
    assert_eq!(count, 1);
    let _: Vec<u8> = match handle.metadata(meta_ids[0]) {
        Ok(meta) => meta,
        Err(e) => return Err(format!("Error reading metadata: {:?}", e)),
    };

    // Decode the image
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgba), None).unwrap();
    assert_eq!(image.color_space(), Some(ColorSpace::Rgb(RgbChroma::Rgba)),);
    // assert_eq!(image.width(), 1652);
    // assert_eq!(image.height(), 1791);

    // Scale the image
    // let small_img = image.scale(1280, 720, None)?;
    // assert_eq!(small_img.width(), 1280);
    // assert_eq!(small_img.height(), 720);

    // Get "pixels"
    let planes: libheif_rs::Planes<&[u8]> = image.planes();
    let interleaved_plane: libheif_rs::Plane<&[u8]> = match planes.interleaved {
        Some(plane) => plane,
        None => return Err(format!("Error reading interleaved plane.")),
    };
    
    assert_ne!(interleaved_plane.width, 0);
    assert_ne!(interleaved_plane.height, 0);
    assert!(!interleaved_plane.data.is_empty());
    assert!(interleaved_plane.stride > 0);
    // eprint!("Loaded HEIC image whose size is {}x{}", interleaved_plane.width, interleaved_plane.height);

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        interleaved_plane.data,
        interleaved_plane.width,
        interleaved_plane.height,
    );

    assert_ne!(buffer.width(), 0);
    assert_ne!(buffer.height(), 0);
    // eprintln!("Done creating slint image buffer, dimensions are {}x{}", buffer.width(), buffer.height());
    Ok(buffer)
    // Ok(bytes)
    // Err(HeifError { code: libheif_rs::HeifErrorCode::ColorProfileDoesNotExist, sub_code: libheif_rs::HeifErrorSubCode::InvalidProperty, message: "".to_string() })
    }
        
    pub fn type_str_to_image_type(type_str: &str) -> Option<ImageFormat> {
        match type_str {
            "image/jpeg" => Some(ImageFormat::Jpeg),
            "image/png" => Some(ImageFormat::Png),
            "image/gif" => Some(ImageFormat::Gif),
            "image/bmp" => Some(ImageFormat::Bmp),
            "image/tiff" => Some(ImageFormat::Tiff),
            "image/x-icon" => Some(ImageFormat::Ico),
            "image/heic" => Some(ImageFormat::OpenExr),
            _ => None,
        }
    }