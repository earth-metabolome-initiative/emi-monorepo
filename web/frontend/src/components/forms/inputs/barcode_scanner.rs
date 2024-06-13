use gloo::timers::callback::Interval;
use gloo::utils::errors::JsError;
use gloo::utils::window;
use image::codecs::jpeg::JpegEncoder;
use image::Luma;
use rxing::qrcode::encoder::QRCode;
use rxing::{self, BarcodeFormat, Exceptions};
use std::collections::hash_set::HashSet;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_common::traits::GuessImageFormat;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement, MediaStream,
    MediaStreamConstraints, MediaStreamTrack, MediaTrackConstraints, VideoFacingModeEnum,
};
use yew::prelude::*;

pub struct Scanner {
    video_ref: NodeRef,
    canvas_ref: NodeRef,
    canvas_to_display: NodeRef,
    stream: Option<MediaStream>,
    is_scanning: bool,
    is_flashlight_on: bool,
    interval: Option<Interval>,
    image: Option<Vec<u8>>,
}

pub enum ScannerMessage {
    ReceivedStream(MediaStream),
    CapturedImage,
    Error(JsError),
    ToggleScanner,
    CloseScanner,
    ToggleFlashlight,
    VideoTimeUpdate,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ScannerProps {
    #[prop_or_default]
    pub onscan: Callback<rxing::RXingResult>,
    #[prop_or_default]
    pub onerror: Callback<JsError>,
    #[prop_or_default]
    pub onclose: Callback<()>,
    #[prop_or(100)]
    pub refresh_milliseconds: u32,
    #[prop_or(0.3)]
    crop_percentage: f64,
    #[prop_or(300)]
    crop_dimension: u32,
}

impl Scanner {
    fn get_resolution(&self) -> (u32, u32) {
        let video = match self.video_ref.cast::<HtmlVideoElement>() {
            Some(video) => video,
            None => return (300, 300),
        };
        let mut video_height = video.video_height();
        let mut video_width = video.video_width();

        let max_resolution = 800;

        if video_height > max_resolution || video_width > max_resolution {
            let ratio = video_width as f64 / video_height as f64;
            if video_height > video_width {
                video_height = max_resolution;
                video_width = (max_resolution as f64 * ratio) as u32;
            } else {
                video_width = max_resolution;
                video_height = (max_resolution as f64 / ratio) as u32;
            }
        }
        (video_width, video_height)
    }
}

impl Component for Scanner {
    type Message = ScannerMessage;
    type Properties = ScannerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            video_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            canvas_to_display: NodeRef::default(),
            stream: None,
            is_scanning: false,
            is_flashlight_on: false,
            interval: None,
            image: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let time_update = ctx.link().callback(|_| ScannerMessage::VideoTimeUpdate);
        let toggle_scanner = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::ToggleScanner
        });
        let close_scanner = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::CloseScanner
        });
        let toggle_flashlight = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::ToggleFlashlight
        });
        let (video_width, video_height) = self.get_resolution();

        let image_url = self
            .image
            .as_ref()
            .and_then(|image| image.guess_image_url());

        html! {
            <>
                // Button to start or stop the scanner
            if !self.is_scanning {
                <button onclick={toggle_scanner} title="Start Scanner" class="start-scanner">
                    <i class="fas fa-qrcode"></i>
                </button>
            }
            // Modal for the scanner
            if self.is_scanning {
                <div class="active-scanner-ui">
                    <div class="active-scanner-ui-content">
                        <button class="toggle-flashlight" onclick={&toggle_flashlight} title="Turn on/off flashlight">
                            <i class="fas fa-lightbulb"></i>
                        </button> // Add this line
                        <button class="close" onclick={&close_scanner}>{ "×" }</button>
                        <video ref={&self.video_ref} autoPlay="true" ontimeupdate={time_update} style="display: none;"/>
                        if let Some(image_url) = image_url {
                            <img src={image_url} alt="Captured Image" class="captured-image"/>
                        }
                        <canvas ref={&self.canvas_ref} width={video_width.to_string()} height={video_height.to_string()} style="display: none;"></canvas>
                        <canvas ref={&self.canvas_to_display} width={video_width.to_string()} height={video_height.to_string()} style="display: block;"></canvas>
                    </div>
                </div>
                }
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ScannerMessage::VideoTimeUpdate => {
                if self.interval.is_some() {
                    return false;
                }
                let link = ctx.link().clone();
                self.interval = Some(Interval::new(ctx.props().refresh_milliseconds, move || {
                    link.send_message(ScannerMessage::CapturedImage);
                }));
                false
            }
            ScannerMessage::ReceivedStream(stream) => {
                self.stream = Some(stream);
                let video = self
                    .video_ref
                    .cast::<HtmlVideoElement>()
                    .expect("video should be an HtmlVideoElement");

                video.set_src_object(self.stream.as_ref().clone());
                true
            }

            ScannerMessage::CapturedImage => {
                if !self.is_scanning {
                    return false;
                }

                let (video_width, video_height) = self.get_resolution();

                let canvas = self
                    .canvas_ref
                    .cast::<HtmlCanvasElement>()
                    .expect("canvas should be an HtmlCanvasElement");

                let context = canvas
                    .get_context("2d")
                    .expect("context should be available")
                    .unwrap()
                    .unchecked_into::<CanvasRenderingContext2d>();

                let video = self
                    .video_ref
                    .cast::<HtmlVideoElement>()
                    .expect("video should be an HtmlVideoElement");
                match context.draw_image_with_html_video_element(&video, 0.0, 0.0) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{:?}", e);
                        return true;
                    }
                }

                let canvas_to_display = self
                    .canvas_to_display
                    .cast::<HtmlCanvasElement>()
                    .expect("canvas should be an HtmlCanvasElement");

                let context_to_display = canvas_to_display
                    .get_context("2d")
                    .expect("context should be available")
                    .unwrap()
                    .unchecked_into::<CanvasRenderingContext2d>();
                match context_to_display.draw_image_with_html_video_element(&video, 0.0, 0.0) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{:?}", e);
                        return true;
                    }
                }
                let image_data =
                    match context.get_image_data(0.0, 0.0, video_width as f64, video_height as f64)
                    {
                        Ok(image_data) => image_data,
                        Err(error) => {
                            log::error!("{:?}", error);
                            return true;
                        }
                    };

                let image_data = convert_js_image_to_luma(&image_data.data());

                // We crop the image down to the central 80% square of the image.
                let square_side = (video_width as f64 * ctx.props().crop_percentage)
                    .min(video_height as f64 * ctx.props().crop_percentage);
                let x = (video_width as f64 - square_side) / 2.0;
                let y = (video_height as f64 - square_side) / 2.0;
                let image_data: Vec<u8> = image_data
                    .chunks_exact(video_width as usize)
                    .skip(y as usize)
                    .take(square_side as usize)
                    .map(|row| {
                        row.iter()
                            .skip(x as usize)
                            .take(square_side as usize)
                            .cloned()
                    })
                    .flatten()
                    .collect();

                // Next, we resize the image to 300x300 pixels.
                let resized_image: image::ImageBuffer<image::Luma<u8>, Vec<u8>> =
                    image::imageops::resize(
                        &image::ImageBuffer::from_raw(
                            square_side as u32,
                            square_side as u32,
                            image_data,
                        )
                        .unwrap(),
                        ctx.props().crop_dimension,
                        ctx.props().crop_dimension,
                        image::imageops::FilterType::Nearest,
                    );

                // we convert the image into a jpeg and serialize it into a Vec<u8>
                let mut jpeg_image = Vec::new();
                let mut encoder = JpegEncoder::new_with_quality(&mut jpeg_image, 100);
                encoder
                    .encode(
                        resized_image.as_raw(),
                        resized_image.width(),
                        resized_image.height(),
                        image::ExtendedColorType::L8,
                    )
                    .unwrap();
                self.image = Some(jpeg_image);

                let decode_result = decode_barcode(
                    resized_image.into_vec(),
                    ctx.props().crop_dimension,
                    ctx.props().crop_dimension,
                    Some(false),
                    Some(false),
                );
                match decode_result {
                    Ok(s) => {
                        ctx.props().onscan.emit(s);
                        ctx.link().send_message(ScannerMessage::CloseScanner);
                    }
                    Err(e) => {
                        ctx.link().send_message(ScannerMessage::Error(JsError::from(
                            js_sys::Error::new(e.to_string().as_str()),
                        )));
                    }
                }
                true
            }
            ScannerMessage::Error(e) => {
                ctx.props().onerror.emit(e);
                true
            }
            ScannerMessage::ToggleScanner => {
                ctx.link().send_future(async {
                    let mut constraints = MediaStreamConstraints::new();
                    let mut video_constraints = MediaTrackConstraints::new();

                    let advanced_constraints = js_sys::Array::new();
                    let torch_constraint = js_sys::Object::new();
                    js_sys::Reflect::set(
                        &torch_constraint,
                        &JsValue::from_str("torch"),
                        &JsValue::from_bool(false),
                    )
                    .unwrap();
                    advanced_constraints.push(&torch_constraint);
                    video_constraints.advanced(&advanced_constraints);

                    video_constraints
                        .facing_mode(&VideoFacingModeEnum::Environment.into())
                        .frame_rate(&20.into());

                    constraints.video(&video_constraints);
                    match window().navigator().media_devices() {
                        Ok(devs) => match devs.get_user_media_with_constraints(&constraints) {
                            Ok(promise) => {
                                match wasm_bindgen_futures::JsFuture::from(promise).await {
                                    Ok(stream) => {
                                        ScannerMessage::ReceivedStream(stream.unchecked_into())
                                    }
                                    Err(e) => ScannerMessage::Error(JsError::try_from(e).unwrap()),
                                }
                            }
                            Err(e) => ScannerMessage::Error(JsError::try_from(e).unwrap()),
                        },
                        Err(e) => ScannerMessage::Error(JsError::try_from(e).unwrap()),
                    }
                });
                self.is_scanning = !self.is_scanning;
                true
            }
            ScannerMessage::CloseScanner => {
                // close event
                if let Some(stream) = self.stream.as_ref() {
                    for track in stream.get_tracks().iter() {
                        if let Ok(track) = track.dyn_into::<MediaStreamTrack>() {
                            track.stop();
                        }
                    }
                }

                self.is_scanning = false;
                self.stream = None;
                self.is_flashlight_on = false;
                if let Some(interval) = self.interval.take() {
                    interval.cancel();
                }
                ctx.props().onclose.emit(());
                true
            }
            ScannerMessage::ToggleFlashlight => {
                if let Some(stream) = &self.stream {
                    let track = stream
                        .get_video_tracks()
                        .get(0)
                        .dyn_into::<MediaStreamTrack>();
                    let constraints = js_sys::Object::new();
                    js_sys::Reflect::set(
                        &constraints,
                        &JsValue::from_str("torch"),
                        &JsValue::from_bool(!self.is_flashlight_on),
                    )
                    .unwrap();
                    let advanced_constraints = js_sys::Array::new();
                    advanced_constraints.push(&constraints);
                    let mut video_constraints = MediaTrackConstraints::new();
                    video_constraints
                        .advanced(&advanced_constraints)
                        .facing_mode(&VideoFacingModeEnum::Environment.into())
                        .frame_rate(&20.into());
                    let _ = track
                        .expect("Cannot apply constrait")
                        .apply_constraints_with_constraints(&video_constraints)
                        .unwrap();
                    self.is_flashlight_on = !self.is_flashlight_on;
                }
                true
            }
        }
    }
}

/// Decode a barcode from an array of 8bit luma data
pub(crate) fn decode_barcode(
    data: Vec<u8>,
    width: u32,
    height: u32,
    try_harder: Option<bool>,
    filter_image: Option<bool>,
) -> Result<rxing::RXingResult, Exceptions> {
    let mut hints: rxing::DecodingHintDictionary = HashMap::new();
    hints.insert(
        rxing::DecodeHintType::POSSIBLE_FORMATS,
        rxing::DecodeHintValue::PossibleFormats([BarcodeFormat::QR_CODE].iter().cloned().collect()),
    );
    if let Some(true) = try_harder {
        hints.insert(
            rxing::DecodeHintType::TRY_HARDER,
            rxing::DecodeHintValue::TryHarder(true),
        );
    }

    let detection_function = if matches!(filter_image, Some(true)) {
        rxing::helpers::detect_in_luma_filtered_with_hints
    } else {
        rxing::helpers::detect_in_luma_with_hints
    };

    detection_function(
        data,
        width,
        height,
        Some(BarcodeFormat::QR_CODE),
        &mut hints,
    )
}

/// Convert a javascript image context's data into luma 8.
///
/// Data for this function can be found from any canvas object
/// using the `data` property of an `ImageData` object.
/// Such an object could be obtained using the `getImageData`
/// method of a `CanvasRenderingContext2D` object.
pub(crate) fn convert_js_image_to_luma(data: &[u8]) -> Vec<u8> {
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
