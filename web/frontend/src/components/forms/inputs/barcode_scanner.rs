use crate::utils::is_mobile_device;
use gloo::timers::callback::Interval;
use gloo::timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_common::api::ApiError;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement, MediaStream, MediaStreamTrack,
};
use yew::prelude::*;
mod barcode_preprocessing;
mod decode_barcode;
use decode_barcode::decode_barcode;
mod camera_metadata;
use camera_metadata::{apply_stream_filter, get_available_cameras, CameraInfo};

pub struct Scanner {
    video_ref: NodeRef,
    canvas_ref: NodeRef,
    stream: Option<MediaStream>,
    is_scanning: bool,
    is_flashlight_on: bool,
    has_loaded: bool,
    mirrored: bool,
    current_camera: Option<(usize, CameraInfo)>,
    number_of_identical_frames: u32,
    cameras: Vec<CameraInfo>,
    closing: Option<Timeout>,
    interval: Option<Interval>,
    image: Option<Vec<u8>>,
}

impl Scanner {
    /// Returns whether the scanner has cameras.
    pub fn has_cameras(&self) -> bool {
        !self.cameras.is_empty()
    }

    /// Returns the number of cameras currently detected.
    pub fn number_of_cameras(&self) -> usize {
        self.cameras.len()
    }

    /// Get the next camera label in the list of available cameras.
    fn get_next_camera(&self) -> Option<(usize, CameraInfo)> {
        if self.number_of_cameras() < 2 {
            return None;
        }
        if let Some((index, _)) = self.current_camera {
            let next_index = (index + 1) % self.cameras.len();
            Some((next_index, self.cameras[next_index].clone()))
        } else {
            None
        }
    }
}

pub enum ScannerMessage {
    ReceivedStream(MediaStream),
    CapturedImage,
    Error(ApiError),
    Start,
    Close,
    ToggleFlashlight,
    VideoTimeUpdate,
    Loaded,
    EffectivelyClose,
    SwitchCamera,
    FindCameras,
    Mirrored,
    RequireUserMedia,
    Cameras(Vec<CameraInfo>),
}

#[derive(Properties, PartialEq, Clone)]
pub struct ScannerProps {
    #[prop_or_default]
    pub onscan: Callback<rxing::RXingResult>,
    #[prop_or_default]
    pub onerror: Callback<ApiError>,
    #[prop_or_default]
    pub onclose: Callback<()>,
    #[prop_or(100)]
    pub refresh_milliseconds: u32,
    #[prop_or(0.4)]
    crop_percentage: f64,
    #[prop_or(256)]
    crop_dimension: u32,
}

impl Component for Scanner {
    type Message = ScannerMessage;
    type Properties = ScannerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            video_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            stream: None,
            is_scanning: false,
            is_flashlight_on: false,
            has_loaded: false,
            current_camera: None,
            cameras: Vec::new(),
            mirrored: !is_mobile_device(),
            interval: None,
            closing: None,
            image: None,
            number_of_identical_frames: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ScannerMessage::RequireUserMedia => {
                let promise = match web_sys::window()
                    .unwrap()
                    .navigator()
                    .media_devices()
                    .unwrap()
                    .get_user_media_with_constraints(
                        &web_sys::MediaStreamConstraints::new()
                            .video(&web_sys::MediaTrackConstraints::default()),
                    ) {
                    Ok(promise) => promise,
                    Err(error) => {
                        ctx.link()
                            .send_message(ScannerMessage::Error(ApiError::from(error)));
                        return false;
                    }
                };
                ctx.link().send_future(async {
                    match wasm_bindgen_futures::JsFuture::from(promise).await {
                        Ok(stream) => ScannerMessage::ReceivedStream(stream.dyn_into().unwrap()),
                        Err(error) => ScannerMessage::Error(ApiError::from(error)),
                    }
                });
                false
            }
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
            ScannerMessage::Loaded => {
                if self.has_loaded {
                    return false;
                }
                self.has_loaded = true;
                true
            }
            ScannerMessage::ReceivedStream(stream) => {
                self.stream = Some(stream);
                self.video_ref
                    .cast::<HtmlVideoElement>()
                    .expect("video should be an HtmlVideoElement")
                    .set_src_object(self.stream.as_ref().clone());

                ctx.link().send_message(ScannerMessage::FindCameras);

                false
            }
            ScannerMessage::CapturedImage => {
                if !self.is_scanning || !self.has_loaded {
                    return false;
                }

                let canvas = self
                    .canvas_ref
                    .cast::<HtmlCanvasElement>()
                    .expect("canvas should be an HtmlCanvasElement");

                // We prepare context options with desynchronized flag to avoid blocking the main thread.

                let context_options = js_sys::Object::new();
                js_sys::Reflect::set(
                    &context_options,
                    &wasm_bindgen::JsValue::from_str("alpha"),
                    &wasm_bindgen::JsValue::from_bool(false),
                )
                .unwrap();
                js_sys::Reflect::set(
                    &context_options,
                    &wasm_bindgen::JsValue::from_str("desynchronized"),
                    &wasm_bindgen::JsValue::from_bool(true),
                )
                .unwrap();
                js_sys::Reflect::set(
                    &context_options,
                    &wasm_bindgen::JsValue::from_str("willReadFrequently"),
                    &wasm_bindgen::JsValue::from_bool(true),
                )
                .unwrap();

                let context = canvas
                    .get_context_with_context_options("2d", &context_options)
                    .expect("context should be available")
                    .unwrap()
                    .unchecked_into::<CanvasRenderingContext2d>();

                let video = self
                    .video_ref
                    .cast::<HtmlVideoElement>()
                    .expect("video should be an HtmlVideoElement");

                if video.video_width() == 0 || video.video_height() == 0 {
                    return false;
                }

                let previous_image_data = context
                    .get_image_data(
                        0.0,
                        0.0,
                        video.video_width() as f64,
                        video.video_height() as f64,
                    )
                    .unwrap();

                if let Err(error) = context.draw_image_with_html_video_element(&video, 0.0, 0.0) {
                    ctx.link()
                        .send_message(ScannerMessage::Error(ApiError::from(error)));
                    return false;
                }

                let image_data = context
                    .get_image_data(
                        0.0,
                        0.0,
                        video.video_width() as f64,
                        video.video_height() as f64,
                    )
                    .unwrap();

                if image_data.data().len() == 0 {
                    return false;
                }

                if image_data.width() == 0 || image_data.height() == 0 {
                    return false;
                }

                // If the two image data are exactly the same, something went wrong
                // and we assume that the video has stopped.
                if previous_image_data.data() == image_data.data() {
                    self.number_of_identical_frames += 1;
                    if self.number_of_identical_frames > 10 {
                        ctx.link().send_message(ScannerMessage::Close);
                        return false;
                    }
                }

                match decode_barcode(
                    image_data,
                    ctx.props().crop_percentage,
                    ctx.props().crop_dimension,
                ) {
                    Ok(s) => {
                        log::info!("Barcode found: {}", s);
                        ctx.props().onscan.emit(s);
                        ctx.link().send_message(ScannerMessage::Close);
                    }
                    Err(error) => {
                        match error {
                            rxing::Exceptions::NotFoundException(_) => {
                                // No barcode found, continue scanning
                            }
                            error => {
                                ctx.link()
                                    .send_message(ScannerMessage::Error(ApiError::from(vec![
                                        error.to_string(),
                                    ])));
                            }
                        }
                    }
                }
                true
            }
            ScannerMessage::Error(error) => {
                ctx.props().onerror.emit(error);
                ctx.link().send_message(ScannerMessage::Close);
                false
            }
            ScannerMessage::Mirrored => {
                self.mirrored = !self.mirrored;
                true
            }
            ScannerMessage::Start => {
                if self.stream.is_none() {
                    ctx.link().send_message(ScannerMessage::RequireUserMedia);
                    return false;
                }

                let current_device = if let Some((_, current_device)) = self.current_camera.as_ref()
                {
                    current_device.clone()
                } else {
                    return false;
                };
                let torch = self.is_flashlight_on;
                let stream = self.stream.as_ref().unwrap().clone();
                self.has_loaded = false;
                self.is_scanning = true;

                ctx.link().send_future(async move {
                    if apply_stream_filter(&stream, &current_device.device_id, torch, None).await {
                        ScannerMessage::Loaded
                    } else {
                        ScannerMessage::Close
                    }
                });
                true
            }
            ScannerMessage::Close => {
                if self.closing.is_some() {
                    return false;
                }

                let link = ctx.link().clone();
                self.closing = Some(Timeout::new(300, move || {
                    link.send_message(ScannerMessage::EffectivelyClose);
                }));
                true
            }
            ScannerMessage::FindCameras => {
                let stream = self.stream.as_ref().unwrap().clone();
                ctx.link().send_future(async move {
                    match get_available_cameras(&stream).await {
                        Ok(cameras) => ScannerMessage::Cameras(cameras),
                        Err(_) => ScannerMessage::Close,
                    }
                });
                false
            }
            ScannerMessage::EffectivelyClose => {
                // close event
                if let Some(stream) = self.stream.as_ref() {
                    for track in stream.get_tracks().iter() {
                        if let Ok(track) = track.dyn_into::<MediaStreamTrack>() {
                            track.stop();
                        }
                    }
                }

                self.closing = None;
                self.is_scanning = false;
                self.has_loaded = false;
                self.stream = None;
                self.is_flashlight_on = false;
                if let Some(interval) = self.interval.take() {
                    interval.cancel();
                }
                ctx.props().onclose.emit(());
                true
            }
            ScannerMessage::ToggleFlashlight => {
                self.is_flashlight_on = !self.is_flashlight_on;
                ctx.link().send_message(ScannerMessage::Start);
                false
            }
            ScannerMessage::Cameras(cameras) => {
                let new_position = self
                    .current_camera
                    .as_ref()
                    .and_then(|(_, camera)| {
                        cameras.iter().position(|c| c.device_id == camera.device_id)
                    })
                    .unwrap_or(0);
                self.current_camera = Some((new_position, cameras[new_position].clone()));
                self.cameras = cameras;
                ctx.link().send_message(ScannerMessage::Start);
                false
            }
            ScannerMessage::SwitchCamera => {
                if !self.has_cameras() {
                    ctx.link().send_message(ScannerMessage::Close);
                    return false;
                }
                if let Some((index, _)) = self.current_camera {
                    self.has_loaded = false;
                    let next_index = (index + 1) % self.cameras.len();
                    self.current_camera = Some((next_index, self.cameras[next_index].clone()));
                    ctx.link().send_message(ScannerMessage::Start);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let time_update = ctx.link().callback(|_| ScannerMessage::VideoTimeUpdate);
        let toggle_scanner = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::Start
        });
        let close_scanner = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::Close
        });
        let toggle_flashlight = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::ToggleFlashlight
        });
        let toggle_camera = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::SwitchCamera
        });
        let mirror = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::Mirrored
        });

        let classes = format!(
            "active-scanner-ui{}{}{}",
            if self.has_loaded { "" } else { " loading" },
            if self.closing.is_some() {
                " closing"
            } else {
                " opening"
            },
            if self.mirrored { " mirrored" } else { "" }
        );

        let onloaded = ctx.link().callback(|_| ScannerMessage::Loaded);

        let flash_light_message = if self.is_flashlight_on {
            "Turn off flashlight"
        } else {
            "Turn on flashlight"
        };

        html! {
            if !self.is_scanning {
                <button onclick={toggle_scanner} title="Start Scanner" class="start-scanner">
                    <i class="fas fa-qrcode"></i>
                </button>
            } else {
                <div class={classes} onclick={&close_scanner}>
                    <video ref={&self.video_ref} autoPlay="true" ontimeupdate={time_update} onplaying={onloaded}></video>
                    if let Some(video) = self.video_ref.cast::<HtmlVideoElement>() {
                        <canvas ref={&self.canvas_ref} width={video.video_width().to_string()} height={video.video_height().to_string()}></canvas>
                    }
                    <div class="scanner-focus-container">
                        <div class="scanner-focus"></div>
                        <ul class="operations">
                            if self.current_camera.as_ref().map_or(false, |(_, camera)| camera.torch) {
                                <li>
                                    <button title={flash_light_message} onclick={toggle_flashlight}>
                                        <i class="fas fa-lightbulb"></i>
                                    </button>
                                </li>
                            }
                            if let Some((camera_number, camera)) = self.get_next_camera() {
                                <li>
                                    <button class="switch-camera" camera-number={camera_number.to_string()} camera-total={self.number_of_cameras().to_string()} title={camera.label} onclick={toggle_camera}>
                                        <i class="fas fa-sync-alt"></i>
                                    </button>
                                </li>
                            }
                            <li>
                                <button title="Mirror" onclick={mirror}>
                                    <i class="fas fa-arrows-alt-h"></i>
                                </button>
                            </li>
                        </ul>
                    </div>
                </div>
            }
        }
    }
}
