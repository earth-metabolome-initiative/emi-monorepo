//! Submodule defining the `InputError` component, which is used to define the error message for an input or form.

use gloo::timers::callback::Timeout;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputErrorProp {
    pub error: String,
    pub on_delete: Callback<String>,
}

pub struct InputError {
    hide: Option<Timeout>,
}

pub enum InputErrorMessage {
    Hide,
    StartHide,
}

impl Component for InputError {
    type Message = InputErrorMessage;
    type Properties = InputErrorProp;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { hide: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputErrorMessage::StartHide => {
                // When we receive the hide message, we start a timeout to hide the error message
                // so to allow for an adequate CSS transition before the component is removed from the DOM.
                let link = ctx.link().clone();

                // If the user has clicked multiple times on the delete button, we do not want to
                // create multiple timeouts, so we check if there is already a timeout running.
                if self.hide.is_some() {
                    return false;
                }

                self.hide = Some(Timeout::new(200, move || {
                    link.send_message(InputErrorMessage::Hide);
                }));
            }
            InputErrorMessage::Hide => {
                // When the timeout is finished, we remove the error message from the DOM.
                self.hide = None;
                ctx.props().on_delete.emit(ctx.props().error.clone());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let classes = if self.hide.is_some() {
            "input-error hiding"
        } else {
            "input-error"
        };

        html! {
            <li class={classes}>
                <p>{&props.error}</p>
                <button onclick={ctx.link().callback(|_| InputErrorMessage::StartHide)}>
                    <i class="fas fa-times"></i>
                </button>
            </li>
        }
    }
}
