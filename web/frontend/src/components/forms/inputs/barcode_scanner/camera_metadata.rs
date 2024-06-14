use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_common::api::DeviceError;
use web_sys::{window, MediaDeviceInfo, MediaStreamConstraints};

/// A struct to hold information about a camera and its torch capability.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CameraInfo {
    pub device_id: String,
    pub group_id: String,
    pub label: String,
}

impl CameraInfo {
    pub fn is_front_camera(&self) -> bool {
        self.label.contains("iPhone")
    }
}

/// Checks available cameras and their torch capabilities.
///
/// This function identifies all video input devices, checks each one for torch capability,
/// and determines if they are front or back cameras. It returns a vector of CameraInfo structures
/// containing details for each camera.
///
/// # Returns
/// A `Result` containing a vector of `CameraInfo` or a `ApiError` error if the process fails.
pub async fn get_available_cameras() -> Result<Vec<CameraInfo>, web_common::api::ApiError> {
    let window = window().ok_or_else(|| JsValue::from_str("No global `window` exists"))?;
    let navigator = window.navigator();
    let media_devices = navigator
        .media_devices()
        .map_err(|_| DeviceError::NoCameras)?;

    let devices = wasm_bindgen_futures::JsFuture::from(
        media_devices
            .enumerate_devices()
            .map_err(|_| DeviceError::NoCameras)?,
    )
    .await
    .map_err(|_| DeviceError::NoCameras)?
    .dyn_into::<js_sys::Array>()
    .unwrap();

    let mut cameras: Vec<CameraInfo> = vec![];

    for x in devices.iter() {
        let device: MediaDeviceInfo = x.dyn_into()?;
        if device.kind() != web_sys::MediaDeviceKind::Videoinput {
            continue;
        }

        let mut constraints = MediaStreamConstraints::new();
        let mut video_constraints = web_sys::MediaTrackConstraints::new();

        let advanced_constraints = js_sys::Array::new();
        video_constraints.device_id(&JsValue::from_str(&device.device_id()));
        advanced_constraints.push(&video_constraints.into());
        constraints.video(&advanced_constraints.into());

        // In devices such as emulator, you will receive an error if you try to get a stream
        // with constraints that are not supported by the device.
        if device.device_id().is_empty() {
            continue;
        }

        cameras.push(CameraInfo {
            device_id: device.device_id(),
            label: device.label(),
            group_id: device.group_id(),
        });
    }

    if cameras.is_empty() {
        return Err(DeviceError::NoCameras.into());
    }

    Ok(cameras)
}

/// Attempts to get a media stream from a specific camera using its device ID.
///
/// # Arguments
/// * `device_id` - A string slice that holds the device ID of the camera.
/// * `torch` - A boolean that indicates if the torch should be enabled.
///
/// # Returns
/// A `Result` containing a `MediaStream` if successful, or a `JsValue` error if failed.
pub async fn get_device_stream(
    device_id: &str,
    torch: bool,
) -> Result<web_sys::MediaStream, JsValue> {
    // Access the window's navigator object to get media devices.
    let media_devices = window().unwrap().navigator().media_devices().unwrap();

    // Create a new MediaStreamConstraints object and set video constraints with the device ID.
    let mut constraints = MediaStreamConstraints::new();
    let mut video_constraints = web_sys::MediaTrackConstraints::new();
    js_sys::Reflect::set(
        &video_constraints,
        &JsValue::from("deviceId"),
        &JsValue::from(device_id),
    )?;
    let advanced_constraints = js_sys::Array::new();
    let torch_constraint = js_sys::Object::new();
    js_sys::Reflect::set(
        &torch_constraint,
        &JsValue::from_str("torch"),
        &JsValue::from_bool(torch),
    )
    .unwrap();
    advanced_constraints.push(&torch_constraint);
    video_constraints.advanced(&advanced_constraints);
    constraints.video(&video_constraints);

    // Request media stream using the specified constraints.
    let media_stream_promise = media_devices.get_user_media_with_constraints(&constraints)?;
    let media_stream = wasm_bindgen_futures::JsFuture::from(media_stream_promise)
        .await?
        .dyn_into::<web_sys::MediaStream>()
        .map_err(|_| JsValue::from_str("Failed to get media stream with specified device ID"))?;

    Ok(media_stream)
}
