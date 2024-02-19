//! A reusable component showing a flat-designed input field with an icon on the right that looks like
//! part of the input field. The icon is specified using Font Awesome classes. The input field is
//! responsive and will shrink and grow depending on the size of the screen.
//! 

use yew::prelude::*;

pub struct InputWithIcon {
    icon: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputWithIconProperties {
    pub icon: String,
}

impl Component for InputWithIcon {
    type Message = ();
    type Properties = InputWithIconProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            icon: ctx.props().icon.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="input_wrapper">
                <input
                    type="text"
                    placeholder="Search..."
                />
                <label class="icon">
                    <i class={format!("fas {}", self.icon)}></i>
                </label>
            </div>
        }
    }
}