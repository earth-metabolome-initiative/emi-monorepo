use super::barcode_preprocessing::preprocess_image_data;
use std::collections::HashMap;

#[inline]
fn make_hints() -> rxing::DecodingHintDictionary {
    let mut hints: rxing::DecodingHintDictionary = HashMap::new();
    hints.insert(
        rxing::DecodeHintType::POSSIBLE_FORMATS,
        rxing::DecodeHintValue::PossibleFormats(
            [rxing::BarcodeFormat::QR_CODE].iter().cloned().collect(),
        ),
    );
    hints.insert(
        rxing::DecodeHintType::TRY_HARDER,
        rxing::DecodeHintValue::TryHarder(true),
    );
    hints
}

/// Decode a barcode from an array of 8bit luma data
pub(super) fn decode_barcode(
    image_data: web_sys::ImageData,
    crop_percentage: f64,
    crop_dimension: u32,
) -> Result<rxing::RXingResult, rxing::Exceptions> {
    let (luma_data, square_side) = preprocess_image_data(image_data, crop_percentage, crop_dimension);
    rxing::helpers::detect_in_luma_with_hints(
        luma_data,
        square_side as u32,
        square_side as u32,
        Some(rxing::BarcodeFormat::QR_CODE),
        &mut make_hints(),
    )
}
