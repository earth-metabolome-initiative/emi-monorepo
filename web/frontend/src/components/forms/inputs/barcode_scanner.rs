use gloo::timers::callback::Interval;
use gloo::timers::callback::Timeout;
use gloo::utils::window;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_common::api::ApiError;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement, MediaStream,
    MediaStreamConstraints, MediaStreamTrack, MediaTrackConstraints, VideoFacingModeEnum,
};
use yew::prelude::*;
mod barcode_preprocessing;
mod decode_barcode;
use decode_barcode::decode_barcode;

pub struct Scanner {
    video_ref: NodeRef,
    canvas_ref: NodeRef,
    stream: Option<MediaStream>,
    is_scanning: bool,
    is_flashlight_on: bool,
    has_loaded: bool,
    closing: Option<Timeout>,
    interval: Option<Interval>,
    image: Option<Vec<u8>>,
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
    SetLoaded,
    EffectivelyClose,
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
    #[prop_or(0.3)]
    crop_percentage: f64,
    #[prop_or(300)]
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
            interval: None,
            closing: None,
            image: None,
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
            ScannerMessage::Loaded => {
                if self.has_loaded {
                    return false;
                }
                // let link = ctx.link().clone();
                // let timeout = Timeout::new(1000, move || {
                //     link.send_message(ScannerMessage::SetLoaded);
                // });
                // timeout.forget();
                self.has_loaded = true;
                true
            }
            ScannerMessage::SetLoaded => {
                self.has_loaded = true;
                true
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
                if !self.is_scanning || !self.has_loaded {
                    return false;
                }

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
            ScannerMessage::Start => {
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
                        Ok(devs) => {
                            let promise =
                                devs.get_user_media_with_constraints(&constraints).unwrap();
                            let stream =
                                wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                            ScannerMessage::ReceivedStream(stream.unchecked_into())
                        }
                        Err(error) => ScannerMessage::Error(ApiError::from(error)),
                    }
                });
                self.is_scanning = true;
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

        let classes = format!(
            "active-scanner-ui{}{}",
            if self.has_loaded { "" } else { " loading" },
            if self.closing.is_some() {
                " closing"
            } else {
                " opening"
            }
        );

        let onloaded = ctx.link().callback(|_| ScannerMessage::Loaded);

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
                <div class={classes} onclick={&close_scanner}>
                    <video ref={&self.video_ref} autoPlay="true" ontimeupdate={time_update} onplaying={onloaded}/>
                    <div class="scanner-focus-container">
                        <div class="scanner-focus">
                            // <button class="toggle-flashlight" onclick={&toggle_flashlight} title="Turn on/off flashlight">
                            //     <i class="fas fa-lightbulb"></i>
                            // </button>
                            if let Some(video) = self.video_ref.cast::<HtmlVideoElement>() {
                                <canvas ref={&self.canvas_ref} width={video.video_width().to_string()} height={video.video_height().to_string()}></canvas>
                            }
                        </div>
                    </div>
                </div>
                }
            </>
        }
    }
}
