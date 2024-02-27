use crate::{Model, Form};
use yew::{Callback, Component, Html, html, Properties, Context};

pub enum CheckBoxMessage {
    OnToggle,
}

#[derive(Properties, Clone, PartialEq)]
pub struct CheckBoxProperties<T: Model> {
    pub field_name: String,
    pub form: Form<T>,
    #[prop_or_else(Callback::noop)]
    pub ontoggle: Callback<bool>,
}

pub struct CheckBox<T: Model> {
    props: CheckBoxProperties<T>,
}

impl<T: Model> CheckBox<T> {
    fn value(&self) -> bool {
        let field_path = &self.props.field_name;

        self.props.form.field_value(field_path) == "true"
    }

    fn set_value(&mut self, value: bool) {
        let field_path = &self.props.field_name;

        self.props.form.set_field_value(field_path, &value.to_string());
    }
}

impl<T: Model + Clone> Component for CheckBox<T> {
    type Message = CheckBoxMessage;
    type Properties = CheckBoxProperties<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        Self{ props }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CheckBoxMessage::OnToggle => {
                let value = !self.value();
                self.set_value(value);
                self.props.ontoggle.emit(value);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
            <input
                type="checkbox"
                value={self.props.field_name.clone()}
                onclick={link.callback(|_e| CheckBoxMessage::OnToggle)}
                checked={self.value()}
                class="form-check-input form-input"
             />
            </>
        }
    }
}

