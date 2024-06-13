/// Convert a javascript image context's data into luma 8.
///
/// Data for this function can be found from any canvas object
/// using the `data` property of an `ImageData` object.
/// Such an object could be obtained using the `getImageData`
/// method of a `CanvasRenderingContext2D` object.
fn convert_js_image_to_luma(data: &[u8]) -> Vec<u8> {
    let mut luma_data = Vec::with_capacity(data.len() / 4);
    for src_pixel in data.chunks_exact(4) {
        let [red, green, blue, alpha] = src_pixel else {
            continue;
        };
        let pixel = if *alpha == 0 {
            // white, so we know its luminance is 255
            0xFF
        } else {
            // .299R + 0.587G + 0.114B (YUV/YIQ for PAL and NTSC),
            // (306*R) >> 10 is approximately equal to R*0.299, and so on.
            // 0x200 >> 10 is 0.5, it implements rounding.

            ((306 * (*red as u64) + 601 * (*green as u64) + 117 * (*blue as u64) + 0x200) >> 10)
                as u8
        };
        luma_data.push(pixel);
    }

    luma_data
}

fn crop_square_from_center(
    data: &[u8],
    crop_percentage: f64,
    width: u32,
    height: u32,
) -> (Vec<u8>, usize) {
    // We crop the image down to the central 80% square of the image.
    let square_side = (width as f64 * crop_percentage).min(height as f64 * crop_percentage);
    let x = (width as f64 - square_side) / 2.0;
    let y = (height as f64 - square_side) / 2.0;
    (
        data.chunks_exact(width as usize)
            .skip(y as usize)
            .take(square_side as usize)
            .map(|row| {
                row.iter()
                    .skip(x as usize)
                    .take(square_side as usize)
                    .cloned()
            })
            .flatten()
            .collect(),
        square_side as usize,
    )
}

fn downscale_image(
    data: &[u8],
    square_side: u32,
    crop_dimension: u32,
) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    image::imageops::resize(
        &image::ImageBuffer::from_raw(square_side, square_side, data).unwrap(),
        crop_dimension,
        crop_dimension,
        image::imageops::FilterType::CatmullRom,
    )
}

/// Preprocess image data for barcode detection.
///
///
pub(super) fn preprocess_image_data(
    image_data: web_sys::ImageData,
    crop_percentage: f64,
    crop_dimension: u32,
) -> (Vec<u8>, usize) {
    let data = convert_js_image_to_luma(&image_data.data());
    let (cropped_data, square_side) = crop_square_from_center(
        &data,
        crop_percentage,
        image_data.width(),
        image_data.height(),
    );
    if square_side > crop_dimension as usize {
        let downsampled_data = downscale_image(&cropped_data, square_side as u32, crop_dimension);
        (downsampled_data.into_raw(), crop_dimension as usize)
    } else {
        (cropped_data, square_side)
    }
}
