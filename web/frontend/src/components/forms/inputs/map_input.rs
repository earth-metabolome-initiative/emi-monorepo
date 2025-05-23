use leaflet::{DragEndEvent, DragEvents, LatLng, Layer, MapOptions, Marker, TileLayer};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node};
use yew::prelude::*;

#[derive(Clone)]
pub struct MapInput {
    map: leaflet::Map,
    container: HtmlElement,
    marker: Marker,
    layer: Option<Layer>,
}

impl MapInput {
    fn set_marker(&mut self, latlng: &LatLng) {
        self.map.set_view(&latlng, self.map.get_zoom());
        let layer =
            TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(&self.map);
        if let Some(layer) = self.layer.as_ref() {
            self.map.remove_layer(layer);
        }
        self.layer = Some(layer);
        self.marker.set_lat_lng(&latlng);
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct MapInputProps {
    pub latitude: f64,
    pub longitude: f64,
    pub callback: Callback<(f64, f64)>,
    #[prop_or(13.0)]
    pub zoom: f64,
}

impl MapInputProps {
    pub fn latlng(&self) -> LatLng {
        LatLng::new(self.latitude, self.longitude)
    }
}

pub enum Msg {
    MarkerDrag(LatLng),
}

impl Component for MapInput {
    type Message = Msg;
    type Properties = MapInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        let container: Element =
            web_sys::window().unwrap().document().unwrap().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("map");

        let marker_options = leaflet::MarkerOptions::default();
        marker_options.set_draggable(true);

        let marker = Marker::new_with_options(&ctx.props().latlng(), &marker_options);

        let map_options = MapOptions::default();
        map_options.set_zoom(ctx.props().zoom);

        let map = leaflet::Map::new_with_element(&container, &map_options);
        marker.add_to(&map);

        let link = ctx.link().clone();
        marker.on_drag_end(Box::new(move |event: DragEndEvent| {
            let latlng = event.target().unchecked_into::<Marker>().get_lat_lng();
            link.send_message(Msg::MarkerDrag(latlng));
        }));

        let link = ctx.link().clone();
        map.on_mouse_click(Box::new(move |event: leaflet::MouseEvent| {
            link.send_message(Msg::MarkerDrag(event.lat_lng()));
        }));

        Self { map, container, marker, layer: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MarkerDrag(new_pos) => {
                ctx.props().callback.emit((new_pos.lat(), new_pos.lng()));
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.set_marker(&ctx.props().latlng());
        }

        let center = self.map.get_center();
        if (center.lat() - ctx.props().latitude).abs() > f32::EPSILON as f64 * 10.0
            || (center.lng() - ctx.props().longitude).abs() > f32::EPSILON as f64 * 10.0
        {
            self.set_marker(&ctx.props().latlng());
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let node: &Node = &self.container.clone().into();
        let map = Html::VRef(node.clone());

        html! {
            {map}
        }
    }
}
