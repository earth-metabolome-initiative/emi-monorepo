//! Yew component handling GPS input. If possible, it tries to use the browser's
//! geolocation API to get the user's current position. If that fails, it falls
//! back to a set of simple input fields and asks the user to enter the coordinates.
//!
//! These input fields also include the accuracy of the measurement.
//!
//! TODO: also handle the registration device!
use validator::Validate;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Coordinates, Geolocation, Position, PositionError, PositionOptions};
use yew::prelude::*;

use super::BasicInput;
use super::MapInput;

#[derive(Clone, PartialEq, Properties)]
pub struct GPSInputProps {
    pub label: String,
    #[prop_or_default]
    pub latitude: Option<f64>,
    #[prop_or_default]
    pub longitude: Option<f64>,
    #[prop_or_default]
    pub altitude: Option<f64>,
    #[prop_or_default]
    pub altitude_accuracy: Option<f64>,
    #[prop_or_default]
    pub accuracy: Option<f64>,
}

#[derive(Clone, PartialEq, Default, Validate)]
struct Float64 {
    value: f64,
}

impl TryFrom<String> for Float64 {
    type Error = Vec<String>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.parse::<f64>() {
            Ok(value) => Ok(Float64 { value }),
            Err(_) => Err(vec![format!(
                "Unable to parse the value '{}' as a floating point number",
                value
            )]),
        }
    }
}

impl From<f64> for Float64 {
    fn from(value: f64) -> Self {
        Float64 { value }
    }
}

impl From<Float64> for String {
    fn from(value: Float64) -> String {
        value.value.to_string()
    }
}

impl From<Float64> for f64 {
    fn from(value: Float64) -> f64 {
        value.value
    }
}

impl ToString for Float64 {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[function_component(GPSInput)]
pub fn gps_input(props: &GPSInputProps) -> Html {
    // First of all, we try to create a new instance of the
    // geolocation API making sure to use the options to enable
    // high accuracy and to set a timeout of 5 seconds. This way
    // we can, if the user allows it, get the current position
    // with a high degree of accuracy and keep it updated.
    let mut position_options = PositionOptions::new();
    position_options.enable_high_accuracy(true).maximum_age(10_000);

    let latitude: UseStateHandle<Option<f64>> = use_state(|| props.latitude);
    let longitude: UseStateHandle<Option<f64>> = use_state(|| props.longitude);
    let altitude: UseStateHandle<Option<f64>> = use_state(|| props.altitude);
    let accuracy: UseStateHandle<Option<f64>> = use_state(|| props.accuracy);
    let altitude_accuracy: UseStateHandle<Option<f64>> = use_state(|| props.altitude_accuracy);

    if latitude.is_none() || longitude.is_none() {
        if let Some(geolocation) = web_sys::window().and_then(|win| win.navigator().geolocation().ok())
        {
            let geolocation: Geolocation = geolocation.clone();
            let latitude = latitude.clone();
            let longitude = longitude.clone();
            let altitude = altitude.clone();
            let accuracy = accuracy.clone();
            let altitude_accuracy = altitude_accuracy.clone();
            let position_options = position_options.clone();

            let callback = Closure::wrap(Box::new(move |position: Position| {
                let coords: Coordinates = position.coords();
                latitude.set(Some(coords.latitude()));
                longitude.set(Some(coords.longitude()));
                altitude.set(coords.altitude());
                accuracy.set(Some(coords.accuracy()));
                altitude_accuracy.set(coords.altitude_accuracy());
            }) as Box<dyn Fn(Position)>);

            let error_callback = Closure::wrap(Box::new(move |err: PositionError| {
                log::error!("Error getting position: {:?}", err);
            }) as Box<dyn Fn(PositionError)>);

            if let Err(error) = geolocation.watch_position_with_error_callback_and_options(
                &callback.as_ref().unchecked_ref(),
                Some(&error_callback.as_ref().unchecked_ref()),
                &position_options,
            ) {}

            callback.forget();
            error_callback.forget();
        }
    }

    let map_callback = {
        let latitude = latitude.clone();
        let longitude = longitude.clone();
        Callback::from(move |coords: (f64, f64)| {
            latitude.set(Some(coords.0));
            longitude.set(Some(coords.1));
        })
    };

    html! {
        <div class="gps-input">
            <MapInput latitude={(*latitude).unwrap_or(0.0)} longitude={(*longitude).unwrap_or(0.0)} callback={map_callback}/>
            <ul class="coords-wrapper input-group">
                <li>
                    <BasicInput<Float64>
                        label={"Latitude".to_string()}
                        value={(*latitude).map(Float64::from)}
                        input_type="number"
                        step={0.000001}
                    />
                </li>
                <li>
                    <BasicInput<Float64>
                        label={"Longitude".to_string()}
                        value={(*longitude).map(Float64::from)}
                        input_type="number"
                        step={0.000001}
                    />
                </li>
                <li>
                    <BasicInput<Float64>
                        label={"Lat/Lng accuracy".to_string()}
                        value={(*accuracy).map(Float64::from)}
                        input_type="number"
                        step={0.000001}
                    />
                </li>
            </ul>
            <ul class="altitude-wrapper input-group">
                <li>
                    <BasicInput<Float64>
                        label={"Altitude".to_string()}
                        value={(*altitude).map(Float64::from)}
                        input_type="number"
                        step={0.000001}
                    />
                </li>
                <li>
                    <BasicInput<Float64>
                        label={"Altitude accuracy".to_string()}
                        value={(*altitude_accuracy).map(Float64::from)}
                        input_type="number"
                        step={0.000001}
                    />
                </li>
            </ul>
        </div>
    }
}
