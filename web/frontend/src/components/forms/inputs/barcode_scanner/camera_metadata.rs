use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{MediaDeviceInfo, MediaStreamConstraints, window};

use crate::errors::DeviceError;

/// Checks available cameras and their torch capabilities.
///
/// This function identifies all video input devices, checks each one for torch
/// capability, and determines if they are front or back cameras. It returns a
/// vector of CameraInfo structures containing details for each camera.
///
/// # Returns
/// A `Result` containing a vector of `CameraInfo` or a `ApiError` error if the
/// process fails.
pub async fn get_available_cameras() -> Result<Vec<MediaDeviceInfo>, DeviceError> {
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

    let cameras: Vec<MediaDeviceInfo> = devices
        .into_iter()
        .filter_map(|x: JsValue| {
            let device: MediaDeviceInfo = x.dyn_into().ok()?;
            if device.kind() != web_sys::MediaDeviceKind::Videoinput {
                return None;
            }

            // In devices such as emulator, you will receive an error if you try to get a
            // stream with constraints that are not supported by the device.
            if device.device_id().is_empty() {
                return None;
            }

            Some(device)
        })
        .collect();

    if cameras.is_empty() {
        return Err(DeviceError::NoCameras.into());
    }

    Ok(cameras)
}

/// Attempts to get the media stream for a specific camera using its device ID.
pub async fn get_camera_media_stream(device_id: &str) -> Result<web_sys::MediaStream, DeviceError> {
    let mut constraints = MediaStreamConstraints::new();
    let mut video = web_sys::MediaTrackConstraints::new();
    video.set_device_id(&device_id.into());
    constraints.set_video(&video);

    Ok(wasm_bindgen_futures::JsFuture::from(
        window()
            .unwrap()
            .navigator()
            .media_devices()
            .map_err(|_| DeviceError::NoCameras)?
            .get_user_media_with_constraints(&constraints)
            .map_err(|_| DeviceError::NoCameras)?,
    )
    .await
    .map_err(|_| DeviceError::NoCameras)?
    .dyn_into::<web_sys::MediaStream>()
    .map_err(|_| DeviceError::NoCameras)?)
}

/// Attempts to get a media stream from a specific camera using its device ID.
///
/// # Arguments
/// * `stream` - A reference to a `MediaStream` object.
/// * `torch` - A boolean that indicates if the torch should be enabled.
///
/// # Returns
/// A `Result` containing a `MediaStream` if successful, or a `JsValue` error if
/// failed.
pub async fn apply_torch_filter(stream: &web_sys::MediaStream, torch: bool) -> bool {
    for video_track in stream.get_video_tracks() {
        let track = match video_track.dyn_into::<web_sys::MediaStreamTrack>() {
            Ok(track) => track,
            Err(_) => {
                continue;
            }
        };

        let video_constraints = web_sys::MediaTrackConstraints::new();
        if let Err(_err) = js_sys::Reflect::set(
            &video_constraints,
            &JsValue::from_str("torch"),
            &JsValue::from_bool(torch),
        ) {
            continue;
        }

        // First, we apply the constraints regarding the device
        let promise = match track.apply_constraints_with_constraints(&video_constraints) {
            Ok(promise) => promise,
            Err(_) => {
                continue;
            }
        };

        if let Err(err) = wasm_bindgen_futures::JsFuture::from(promise).await {
            log::error!(
                "Failed to apply constraints, errror: {:?}, constraint: {:?}",
                err,
                video_constraints
            );
            continue;
        }

        return true;
    }

    false
}
