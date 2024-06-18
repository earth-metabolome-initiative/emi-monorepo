//! Yew component handling GPS input. If possible, it tries to use the browser's
//! geolocation API to get the user's current position. If that fails, it falls
//! back to a set of simple input fields and asks the user to enter the coordinates.
//!
//! These input fields also include the accuracy of the measurement.
//!
//! TODO: also handle the registration device!
use crate::components::forms::InputErrors;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_common::api::ApiError;
use web_common::api::GeolocationError;
use web_sys::{PositionError, PositionOptions};
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

pub struct GPSInput {
    errors: HashSet<ApiError>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    watch_id: Option<i32>,
    success_callback: Option<Closure<dyn FnMut(web_sys::Position)>>,
    error_callback: Option<Closure<dyn FnMut(PositionError)>>,
}

pub enum GPSInputMessage {
    RequireAuthorization,
    Authorized(i32),
    Error(ApiError),
    Coordinates(Option<f64>, Option<f64>),
    Latitude(Option<f64>),
    Longitude(Option<f64>),
}

impl Component for GPSInput {
    type Message = GPSInputMessage;
    type Properties = GPSInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            errors: HashSet::new(),
            latitude: ctx.props().coordinates.map(|point| point.x),
            longitude: ctx.props().coordinates.map(|point| point.y),
            watch_id: None,
            success_callback: None,
            error_callback: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GPSInputMessage::RequireAuthorization => {
                match web_sys::window().unwrap().navigator().geolocation() {
                    Ok(geolocation) => {
                        let success_callback = {
                            let link = ctx.link().clone();
                            Closure::new(move |position: web_sys::Position| {
                                let coords = position.coords();
                                link.send_message(GPSInputMessage::Coordinates(
                                    Some(coords.latitude()),
                                    Some(coords.longitude()),
                                ));
                            })
                        };

                        let error_callback = {
                            let link = ctx.link().clone();
                            Closure::new(move |error: PositionError| {
                                link.send_message(GPSInputMessage::Error(
                                    GeolocationError::from(error).into(),
                                ));
                            })
                        };

                        if let Err(_err) = geolocation
                            .get_current_position_with_error_callback_and_options(
                                success_callback.as_ref().unchecked_ref(),
                                Some(error_callback.as_ref().unchecked_ref()),
                                PositionOptions::new()
                                    .enable_high_accuracy(true)
                                    .maximum_age(10_000),
                            )
                        {
                            ctx.link().send_message(GPSInputMessage::Error(
                                GeolocationError::NotSupported.into(),
                            ));
                        };

                        match geolocation.watch_position_with_error_callback_and_options(
                            success_callback.as_ref().unchecked_ref(),
                            Some(error_callback.as_ref().unchecked_ref()),
                            PositionOptions::new()
                                .enable_high_accuracy(true)
                                .maximum_age(10_000),
                        ) {
                            Ok(watch_id) => {
                                ctx.link()
                                    .send_message(GPSInputMessage::Authorized(watch_id));
                            }
                            Err(_) => {
                                ctx.link().send_message(GPSInputMessage::Error(
                                    GeolocationError::NotSupported.into(),
                                ));
                            }
                        };

                        self.success_callback = Some(success_callback);
                        self.error_callback = Some(error_callback);
                    }
                    Err(_) => {
                        ctx.link().send_message(GPSInputMessage::Error(
                            GeolocationError::NotSupported.into(),
                        ));
                    }
                }

                false
            }
            GPSInputMessage::Authorized(watch_id) => {
                self.watch_id = Some(watch_id);
                true
            }
            GPSInputMessage::Error(error) => self.errors.insert(error),
            GPSInputMessage::Coordinates(latitude, longitude) => {
                self.latitude = latitude;
                self.longitude = longitude;

                let latitude = self
                    .latitude
                    .or_else(|| ctx.props().coordinates.map(|point| point.x));
                let longitude = self
                    .longitude
                    .or_else(|| ctx.props().coordinates.map(|point| point.y));

                if let (Some(latitude), Some(longitude)) = (latitude, longitude) {
                    ctx.props().builder.emit(Some((latitude, longitude).into()));
                } else {
                    ctx.props().builder.emit(None);
                }

                false
            }
            GPSInputMessage::Latitude(latitude) => {
                ctx.link()
                    .send_message(GPSInputMessage::Coordinates(latitude, self.longitude));

                false
            }
            GPSInputMessage::Longitude(longitude) => {
                ctx.link()
                    .send_message(GPSInputMessage::Coordinates(self.latitude, longitude));

                false
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link()
                .send_message(GPSInputMessage::RequireAuthorization);
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(watch_id) = self.watch_id.take() {
            web_sys::window()
                .unwrap()
                .navigator()
                .geolocation()
                .unwrap()
                .clear_watch(watch_id);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let map_callback = {
            let link = ctx.link().clone();
            Callback::from(move |coords: (f64, f64)| {
                link.send_message(GPSInputMessage::Coordinates(Some(coords.0), Some(coords.1)));
            })
        };

        let latitude_callback = {
            let link = ctx.link().clone();
            Callback::from(move |latitude: Option<String>| {
                if let Some(latitude) = latitude {
                    // We try to convert the latitude to a float and if it fails
                    // we add an error to the store.
                    let latitude = latitude.parse::<f64>();
                    if let Ok(latitude) = latitude {
                        link.send_message(GPSInputMessage::Latitude(Some(latitude)));
                    } else {
                        link.send_message(GPSInputMessage::Error(ApiError::BadRequest(vec![
                            "Latitude must be a number".to_string(),
                        ])));
                    }
                } else {
                    link.send_message(GPSInputMessage::Latitude(None));
                }
            })
        };

        let longitude_callback = {
            let link = ctx.link().clone();
            Callback::from(move |longitude: Option<String>| {
                if let Some(longitude) = longitude {
                    // We try to convert the longitude to a float and if it fails
                    // we add an error to the store.
                    let longitude = longitude.parse::<f64>();
                    if let Ok(longitude) = longitude {
                        link.send_message(GPSInputMessage::Longitude(Some(longitude)));
                    } else {
                        link.send_message(GPSInputMessage::Error(ApiError::BadRequest(vec![
                            "Longitude must be a number".to_string(),
                        ])));
                    }
                } else {
                    link.send_message(GPSInputMessage::Longitude(None));
                }
            })
        };

        let (latitude, longitude) = if let Some(point) = ctx.props().coordinates {
            (point.x, point.y)
        } else {
            (self.latitude.unwrap_or(0.0), self.longitude.unwrap_or(0.0))
        };

        let all_errors = ctx
            .props()
            .errors
            .iter()
            .chain(self.errors.iter())
            .cloned()
            .collect::<Vec<ApiError>>();

        html! {
            <div class="gps-input">
                <MapInput latitude={latitude} longitude={longitude} callback={map_callback} />
                <ul class="coords-wrapper input-group">
                    <li>
                        <BasicInput<f64>
                            label={"Latitude".to_string()}
                            value={Rc::from(latitude)}
                            builder={latitude_callback}
                            optional={ctx.props().optional}
                            errors={Vec::new()}
                        />
                    </li>
                    <li>
                        <BasicInput<f64>
                            label={"Longitude".to_string()}
                            value={Rc::from(longitude)}
                            builder={longitude_callback}
                            optional={ctx.props().optional}
                            errors={Vec::new()}
                        />
                    </li>
                </ul>
                <InputErrors errors={all_errors} />
            </div>
        }
    }
}
