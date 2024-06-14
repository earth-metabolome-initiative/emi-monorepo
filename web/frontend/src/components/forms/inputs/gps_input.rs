//! Yew component handling GPS input. If possible, it tries to use the browser's
//! geolocation API to get the user's current position. If that fails, it falls
//! back to a set of simple input fields and asks the user to enter the coordinates.
//!
//! These input fields also include the accuracy of the measurement.
//!
//! TODO: also handle the registration device!
use crate::components::forms::InputErrors;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_common::api::ApiError;
use web_sys::{Position, PositionError, PositionOptions};
use yew::prelude::*;

use super::BasicInput;
use super::MapInput;

#[derive(Clone, PartialEq, Properties)]
pub struct GPSInputProps {
    pub label: String,
    pub builder: Callback<Option<web_common::types::Point>>,
    pub errors: Vec<ApiError>,
    #[prop_or_default]
    pub coordinates: Option<web_common::types::Point>,
    #[prop_or(false)]
    pub optional: bool,
}

#[function_component(GPSInput)]
pub fn gps_input(props: &GPSInputProps) -> Html {
    // First of all, we try to create a new instance of the
    // geolocation API making sure to use the options to enable
    // high accuracy and to set a timeout of 5 seconds. This way
    // we can, if the user allows it, get the current position
    // with a high degree of accuracy and keep it updated.
    let mut position_options = PositionOptions::new();
    position_options
        .enable_high_accuracy(true)
        .maximum_age(10_000);

    // We create a state to store the errors that might happen
    let errors = use_state(|| Vec::new());

    if props.coordinates.is_none() {
        if let Some(geolocation) =
            web_sys::window().and_then(|win| win.navigator().geolocation().ok())
        {
            let builder = props.builder.clone();

            let callback = Closure::wrap(Box::new(move |position: Position| {
                builder.emit(Some(position.into()))
            }) as Box<dyn Fn(Position)>);

            let errors1 = errors.clone();
            let errors2 = errors.clone();

            let error_callback = Closure::wrap(Box::new(move |_: PositionError| {
                errors1.set(vec![ApiError::BadRequest(vec![
                    "Unable to get the updated position".to_owned(),
                ])]);
            }) as Box<dyn Fn(PositionError)>);

            if let Err(_) = geolocation.watch_position_with_error_callback_and_options(
                &callback.as_ref().unchecked_ref(),
                Some(&error_callback.as_ref().unchecked_ref()),
                &position_options,
            ) {
                errors2.set(vec![ApiError::BadRequest(vec![
                    "Unable to watch the current position".to_owned(),
                ])]);
            }

            callback.forget();
            error_callback.forget();
        }
    }

    let map_callback = {
        let builder = props.builder.clone();
        Callback::from(move |coords: (f64, f64)| builder.emit(Some(coords.into())))
    };

    let latitude_callback = {
        let builder = props.builder.clone();
        let longitude = props.coordinates.map(|point| point.y).unwrap_or(0.0);
        let errors = errors.clone();
        Callback::from(move |latitude: Option<String>| {
            if let Some(latitude) = latitude {
                // We try to convert the latitude to a float and if it fails
                // we add an error to the store.
                let latitude = latitude.parse::<f64>();
                if let Ok(latitude) = latitude {
                    builder.emit(Some((latitude, longitude).into()));
                } else {
                    errors.set(vec![ApiError::BadRequest(vec![
                        "Latitude must be a number".to_string(),
                    ])]);
                }
            } else {
                builder.emit(None);
            }
        })
    };

    let longitude_callback = {
        let builder = props.builder.clone();
        let latitude = props.coordinates.map(|point| point.x).unwrap_or(0.0);
        let errors = errors.clone();
        Callback::from(move |longitude: Option<String>| {
            if let Some(longitude) = longitude {
                // We try to convert the longitude to a float and if it fails
                // we add an error to the store.
                let longitude = longitude.parse::<f64>();
                if let Ok(longitude) = longitude {
                    builder.emit(Some((latitude, longitude).into()));
                } else {
                    errors.set(vec![ApiError::BadRequest(vec![
                        "Longitude must be a number".to_string(),
                    ])]);
                }
            } else {
                builder.emit(None);
            }
        })
    };

    let (latitude, longitude) = props
        .coordinates
        .map(|point| (point.x, point.y))
        .unwrap_or((0.0, 0.0));

    let all_errors = props
        .errors
        .iter()
        .chain(errors.iter())
        .cloned()
        .collect::<Vec<ApiError>>();

    html! {
        <div class="gps-input">
            <MapInput latitude={latitude} longitude={longitude} callback={map_callback}/>
            <ul class="coords-wrapper input-group">
                <li>
                    <BasicInput<f64>
                        label={"Latitude".to_string()}
                        value={Rc::from(latitude)}
                        builder={latitude_callback}
                        optional={props.optional}
                        errors={Vec::new()}
                    />
                </li>
                <li>
                    <BasicInput<f64>
                        label={"Longitude".to_string()}
                        value={Rc::from(longitude)}
                        builder={longitude_callback}
                        optional={props.optional}
                        errors={Vec::new()}
                    />
                </li>
            </ul>
            <InputErrors errors={all_errors} />
        </div>
    }
}
