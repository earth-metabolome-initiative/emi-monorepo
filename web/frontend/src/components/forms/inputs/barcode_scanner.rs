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
use camera_metadata::{get_available_cameras, get_device_stream, CameraInfo};

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

fn is_mobile_device() -> bool {
    let user_agent = web_sys::window()
        .unwrap()
        .navigator()
        .user_agent()
        .unwrap_or_else(|_| String::default())
        .to_lowercase();

    let mobile_regex = regex::Regex::new(r"(android|bb\d+|meego).+mobile|avantgo|bada\/|blackberry|blazer|compal|elaine|fennec|hiptop|iemobile|ip(hone|od)|iris|kindle|lge |maemo|midp|mmp|mobile.+firefox|netfront|opera m(ob|in)i|palm( os)?|phone|p(ixi|re)\/|plucker|pocket|psp|series(4|6)0|symbian|treo|up\.(browser|link)|vodafone|wap|windows ce|xda|xiino|1207|6310|6590|3gso|4thp|50[1-6]i|770s|802s|a wa|abac|ac(er|oo|s\-)|ai(ko|rn)|al(av|ca|co)|amoi|an(ex|ny|yw)|aptu|ar(ch|go)|as(te|us)|attw|au(di|\-m|r |s )|avan|be(ck|ll|nq)|bi(lb|rd)|bl(ac|az)|br(e|v)w|bumb|bw\-(n|u)|c55\/|capi|ccwa|cdm\-|cell|chtm|cldc|cmd\-|co(mp|nd)|craw|da(it|ll|ng)|dbte|dc\-s|devi|dica|dmob|do(c|p)o|ds(12|\-d)|el(49|ai)|em(l2|ul)|er(ic|k0)|esl8|ez([4-7]0|os|wa|ze)|fetc|fly(\-|_)|g1 u|g560|gene|gf\-5|g\-mo|go(\.w|od)|gr(ad|un)|haie|hcit|hd\-(m|p|t)|hei\-|hi(pt|ta)|hp( i|ip)|hs\-c|ht(c(\-| |_|a|g|p|s|t)|tp)|hu(aw|tc)|i\-(20|go|ma)|i230|iac( |\-|\/)|ibro|idea|ig01|ikom|im1k|inno|ipaq|iris|ja(t|v)a|jbro|jemu|jigs|kddi|keji|kgt( |\/)|klon|kpt |kwc\-|kyo(c|k)|le(no|xi)|lg( g|\/(k|l|u)|50|54|\-[a-w])|libw|lynx|m1\-w|m3ga|m50\/|ma(te|ui|xo)|mc(01|21|ca)|m\-cr|me(rc|ri)|mi(o8|oa|ts)|mmef|mo(01|02|bi|de|do|t(\-| |o|v)|zz)|mt(50|p1|v )|mwbp|mywa|n10[0-2]|n20[2-3]|n30(0|2)|n50(0|2|5)|n7(0(0|1)|10)|ne((c|m)\-|on|tf|wf|wg|wt)|nok(6|i)|nzph|o2im|op(ti|wv)|oran|owg1|p800|pan(a|d|t)|pdxg|pg(13|\-([1-8]|c))|phil|pire|pl(ay|uc)|pn\-2|po(ck|rt|se)|prox|psio|pt\-g|qa\-a|qc(07|12|21|32|60|\-[2-7]|i\-)|qtek|r380|r600|raks|rim9|ro(ve|zo)|s55\/|sa(ge|ma|mm|ms|ny|va)|sc(01|h\-|oo|p\-)|sdk\/|se(c(\-|0|1)|47|mc|nd|ri)|sgh\-|shar|sie(\-m)|sk\-0|sl(45|id)|sm(al|ar|b3|it|t5)|so(ft|ny)|sp(01|h\-|v\-|v )|sy(01|mb)|t2(18|50)|t6(00|10|18)|ta(gt|lk)|tcl\-|tdg\-|tel(i|m)|tim\-|t\-mo|to(pl|sh)|ts(70|m\-|m3|m5)|tx\-9|up(\.b|g1|si)|utst|v400|v750|veri|vi(rg|te)|vk(40|5[0-3]|\-v)|vm40|voda|vulc|vx(52|53|60|61|70|80|81|83|85|98)|w3c(\-| )|webc|whit|wi(g |nc|nw)|wmlb|wonu|x700|yas\-|your|zeto|zte\-").unwrap();
    mobile_regex.is_match(&user_agent)
}

impl Scanner {
    /// Returns the number of cameras currently detected.
    pub fn camera_count(&self) -> usize {
        self.cameras.len()
    }

    /// Get the next camera label in the list of available cameras.
    fn get_next_camera_label(&self) -> Option<String> {
        if self.camera_count() < 2 {
            return None;
        }
        if let Some((index, _)) = self.current_camera {
            let next_index = (index + 1) % self.cameras.len();
            Some(self.cameras[next_index].label.clone())
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
    SetLoaded,
    EffectivelyClose,
    SwitchCamera,
    FindCameras,
    DeviceChange,
    Mirrored,
    VideoEnded,
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
            ScannerMessage::DeviceChange => {
                ctx.link().send_message(ScannerMessage::FindCameras);
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
                        ctx.link().send_message(ScannerMessage::VideoEnded);
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
                let current_device = if let Some((_, current_device)) = self.current_camera.as_ref()
                {
                    current_device.clone()
                } else {
                    return false;
                };
                let torch = self.is_flashlight_on;
                ctx.link().send_future(async move {
                    match get_device_stream(&current_device.device_id, torch).await {
                        Ok(stream) => ScannerMessage::ReceivedStream(stream),
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
            ScannerMessage::VideoEnded => {
                // When the video ends but we are still scanning, it may be because the camera
                // has been disconnected. If there are more cameras available, switch to the next one.
                if self.is_scanning && self.has_loaded {
                    // If there is more than one camera, switch to the next one
                    self.has_loaded = false;
                    let current_camera = self.current_camera.take().unwrap();
                    self.cameras
                        .retain(|camera| camera.device_id != current_camera.1.device_id);
                    if self.camera_count() > 0 {
                        self.current_camera = Some((0, self.cameras[0].clone()));
                        ctx.link().send_message(ScannerMessage::Start);
                    } else {
                        ctx.link().send_message(ScannerMessage::Close);
                    }
                    // We also need to search for cameras again, so that our cameras data
                    // is up to date.
                    ctx.link().send_message(ScannerMessage::FindCameras);
                }
                false
            }
            ScannerMessage::FindCameras => {
                ctx.link().send_future(async {
                    match get_available_cameras().await {
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
                if let Some((_, current_device)) = self.current_camera.as_ref() {
                    // If there is already a current camera, we need to check if it is still available.
                    if !cameras
                        .iter()
                        .any(|camera| camera.device_id == current_device.device_id)
                    {
                        // If the current camera is not available, we need to select the first one.
                        self.current_camera = Some((0, cameras[0].clone()));
                    }
                } else {
                    // If there is no current camera, we need to select the first one.
                    self.current_camera = Some((0, cameras[0].clone()));
                }
                self.cameras = cameras;
                true
            }
            ScannerMessage::SwitchCamera => {
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

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(ScannerMessage::FindCameras);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.cameras.is_empty() {
            return html! {};
        }

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
                    <video ref={&self.video_ref} autoPlay="true" ontimeupdate={time_update} onplaying={onloaded}></video>
                    if let Some(video) = self.video_ref.cast::<HtmlVideoElement>() {
                        <canvas ref={&self.canvas_ref} width={video.video_width().to_string()} height={video.video_height().to_string()}></canvas>
                    }
                    <div class="scanner-focus-container">
                        <div class="scanner-focus"></div>
                        <ul class="operations">
                            <li>
                                <button title={flash_light_message} onclick={toggle_flashlight}>
                                    <i class="fas fa-lightbulb"></i>
                                </button>
                            </li>
                            if let Some(label) = self.get_next_camera_label() {
                                <li>
                                    <button title={label} onclick={toggle_camera}>
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
            </>
        }
    }
}
