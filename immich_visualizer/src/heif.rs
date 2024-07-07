
// use libheif_rs::HeifError;
use libheif_rs::{ColorSpace, HeifContext, ItemId, LibHeif, Result, RgbChroma};
use slint::Rgba8Pixel;
use slint::SharedPixelBuffer;
// use image::ImageFormat;
// use slint::Rgba8Pixel;
// use slint::SharedPixelBuffer;

pub fn safe_read_and_decode_heic_memory(
    bytes: &[u8],
) -> Result<SharedPixelBuffer<Rgba8Pixel>> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_bytes(bytes)?;
    let handle = ctx.primary_image_handle().unwrap();
    // assert_eq!(handle.width(), 1652);
    // assert_eq!(handle.height(), 1791);

    // Get Exif
    let mut meta_ids: Vec<ItemId> = vec![0; 1];
    let count = handle.metadata_block_ids(&mut meta_ids, b"Exif");
    assert_eq!(count, 1);
    let _: Vec<u8> = handle.metadata(meta_ids[0])?;

    // Decode the image
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgba), None)?;
    assert_eq!(image.color_space(), Some(ColorSpace::Rgb(RgbChroma::Rgba)),);
    // assert_eq!(image.width(), 1652);
    // assert_eq!(image.height(), 1791);

    // Scale the image
    // let small_img = image.scale(1280, 720, None)?;
    // assert_eq!(small_img.width(), 1280);
    // assert_eq!(small_img.height(), 720);

    // Get "pixels"
    let planes: libheif_rs::Planes<&[u8]> = image.planes();
    let interleaved_plane: libheif_rs::Plane<&[u8]> = planes.interleaved.unwrap();
    assert_ne!(interleaved_plane.width, 0);
    assert_ne!(interleaved_plane.height, 0);
    assert!(!interleaved_plane.data.is_empty());
    assert!(interleaved_plane.stride > 0);
    eprint!("Loaded HEIC image whose size is {}x{}", interleaved_plane.width, interleaved_plane.height);

    // convert to pixel buffer to Bytes
    // Ok(interleaved_plane.data)
    // let pixel_buffer = interleaved_plane.data;
    // let bytes = Bytes::copy_from_slice(pixel_buffer);

    // let img_buffer =  image::ImageBuffer::from_raw(1280, 720, interleaved_plane.data.to_vec()).unwrap();
    
    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        interleaved_plane.data,
        interleaved_plane.width,
        interleaved_plane.height,
    );

    assert_ne!(buffer.width(), 0);
    assert_ne!(buffer.height(), 0);
    eprint!("Done creating slint image buffer, dimensions are {}x{}", buffer.width(), buffer.height());
    
    eprint!("Done decoding HEIC image");
    Ok(buffer)
    // Ok(bytes)
    // Err(HeifError { code: libheif_rs::HeifErrorCode::ColorProfileDoesNotExist, sub_code: libheif_rs::HeifErrorSubCode::InvalidProperty, message: "".to_string() })
    }
        

    // Ok(())
