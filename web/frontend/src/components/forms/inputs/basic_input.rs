//! Module providing a yew component that handles a basic input, which is meant to be used in combination with BasicForm.

use super::InputError;
use crate::workers::WebsocketWorker;
use web_common::api::ws::messages::*;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;

#[derive(Clone, PartialEq, Properties)]
pub struct InputProp {
    pub label: String,
    pub value: String,
    pub input_type: String,
}

impl InputProp {
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn input_type(&self) -> String {
        self.input_type.clone()
    }
}

pub struct BasicInput {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    errors: Vec<String>,
}

pub enum InputMessage {
    Backend(BackendMessage),
    Frontend(FrontendMessage),
    RemoveError(usize),
}

impl Component for BasicInput {
    type Message = InputMessage;
    type Properties = InputProp;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: BackendMessage| {
                    link.send_message(InputMessage::Backend(message));
                }
            })),
            errors: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputMessage::Frontend(fm) => self.websocket.send(fm.into()),
            InputMessage::Backend(_bm) => {}
            InputMessage::RemoveError(error_number) => {
                self.errors.remove(error_number);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let classes = if self.errors.is_empty() {
            "input-group"
        } else {
            "input-group error"
        };

        html! {
            <div class={classes}>
                <label for={props.label()}>{format!("{}:", props.label())}</label>
                <input
                    type={props.input_type()}
                    class="input-control"
                    name={props.label().to_lowercase().replace(" ", "_")}
                    value={props.value().to_string()}
                />
                <ul class="errors">
                    { for self.errors.iter().enumerate().map(|(error_number, error)| {
                        let link = ctx.link().clone();
                        let on_delete = Callback::from(move |_| {
                            link.send_message(InputMessage::RemoveError(error_number));
                        });
                        html! {
                            <li><InputError error={error.clone()} on_delete={on_delete}/></li>
                        }
                    })
                    }
                </ul>
            </div>
        }
    }
}