use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_common::api::DeviceError;
use web_sys::VideoFacingModeEnum;
use web_sys::{window, MediaDeviceInfo, MediaStreamConstraints};

/// A struct to hold information about a camera and its torch capability.
#[derive(Debug, Clone)]
pub struct CameraInfo {
    pub device_id: String,
    pub group_id: String,
    pub label: String,
    pub torch: bool,
    pub facing_mode: Option<VideoFacingModeEnum>,
}

/// Returns the device facing mode by testing all possible modes.
async fn get_facing_mode(
    device: &MediaDeviceInfo,
    stream: &web_sys::MediaStream,
) -> Option<VideoFacingModeEnum> {
    let modes = vec![VideoFacingModeEnum::User, VideoFacingModeEnum::Environment];
    for mode in modes {
        if apply_stream_filter(stream, &device.device_id(), false, Some(mode)).await {
            return Some(mode);
        }
    }
    None
}

/// Returns whether the device has a torch.
pub async fn has_torch(device_id: &str, stream: &web_sys::MediaStream) -> bool {
    apply_stream_filter(stream, device_id, true, None).await
}

/// Checks available cameras and their torch capabilities.
///
/// This function identifies all video input devices, checks each one for torch capability,
/// and determines if they are front or back cameras. It returns a vector of CameraInfo structures
/// containing details for each camera.
///
/// # Returns
/// A `Result` containing a vector of `CameraInfo` or a `ApiError` error if the process fails.
pub async fn get_available_cameras(
    stream: &web_sys::MediaStream,
) -> Result<Vec<CameraInfo>, web_common::api::ApiError> {
    let devices = wasm_bindgen_futures::JsFuture::from(
        window()
            .unwrap()
            .navigator()
            .media_devices()
            .map_err(|_| DeviceError::NoCameras)?
            .enumerate_devices()
            .map_err(|_| DeviceError::NoCameras)?,
    )
    .await
    .map_err(|_| DeviceError::NoCameras)?
    .dyn_into::<js_sys::Array>()
    .map_err(|_| DeviceError::NoCameras)?;

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

        if device.device_id().is_empty() {
            continue;
        }

        cameras.push(CameraInfo {
            device_id: device.device_id(),
            label: device.label(),
            group_id: device.group_id(),
            torch: has_torch(&device.device_id(), stream).await,
            facing_mode: get_facing_mode(&device, stream).await,
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
/// * `stream` - A reference to a `MediaStream` object.
/// * `device_id` - A string slice that holds the device ID of the camera.
/// * `torch` - A boolean that indicates if the torch should be enabled.
/// * `facing_mode` - An optional `VideoFacingModeEnum` that specifies the camera facing mode.
///
/// # Returns
/// A `Result` containing a `MediaStream` if successful, or a `JsValue` error if failed.
pub async fn apply_stream_filter(
    stream: &web_sys::MediaStream,
    device_id: &str,
    torch: bool,
    facing_mode: Option<VideoFacingModeEnum>,
) -> bool {
    for video_track in stream.get_video_tracks() {
        let track = match video_track.dyn_into::<web_sys::MediaStreamTrack>() {
            Ok(track) => track,
            Err(_) => {
                continue;
            }
        };

        let advanced_constraints = js_sys::Array::new();
        let torch_constraint = js_sys::Object::new();
        if let Err(_err) = js_sys::Reflect::set(
            &torch_constraint,
            &JsValue::from_str("torch"),
            &JsValue::from_bool(torch),
        ) {
            continue;
        }
        advanced_constraints.push(&torch_constraint);

        let mut video_constraints = web_sys::MediaTrackConstraints::new();

        if let Err(_err) = js_sys::Reflect::set(
            &video_constraints,
            &JsValue::from_str("torch"),
            &JsValue::from_bool(torch),
        ) {
            continue;
        }

        video_constraints
            .device_id(&device_id.into());
        if let Some(facing_mode) = facing_mode {
            video_constraints.facing_mode(&facing_mode.into());
        }

        let promise = match track.apply_constraints_with_constraints(&video_constraints) {
            Ok(promise) => promise,
            Err(_) => {
                continue;
            }
        };

        if let Err(_err) = wasm_bindgen_futures::JsFuture::from(promise).await {
            continue;
        }

        return true;
    }

    false
}
