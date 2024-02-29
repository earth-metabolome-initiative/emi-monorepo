//! Module providing a yew component that handles a basic input, which is meant to be used in combination with BasicForm.

use validator::{Validate, ValidationErrors};
use wasm_bindgen::convert::TryFromJsValue;
use yew::prelude::*;

pub trait Input: Properties + Validate + Clone {
    type DataType: ToString + TryFromJsValue;
    const TYPE: &'static str;

    /// The value of the input.
    fn value(&self) -> Self::DataType;

    /// Sets and validates the value of the input.
    ///
    /// # Arguments
    /// * `value` - The new value of the input.
    fn set(&mut self, value: Self::DataType) -> Result<(), ValidationErrors>;
}

pub trait InputRef {
    type InputType: Input;

    fn input_ref(&self) -> &Self::InputType;

    fn input_mut(&mut self) -> &mut Self::InputType;
}

impl<I> Input for I
where
    I: InputRef
        + Properties
        + Validate
        + Clone
{
    type DataType = <I::InputType as Input>::DataType;

    const TYPE: &'static str = I::InputType::TYPE;

    fn value(&self) -> Self::DataType {
        self.input_ref().value()
    }

    fn set(&mut self, value: Self::DataType) -> Result<(), ValidationErrors> {
        self.input_mut().set(value)
    }
}

pub trait Labeled: Input {
    /// The label of the input.
    fn label(&self) -> String;
}

pub trait Touched {
    /// Whether the input has been touched by the user.
    fn untouched(&self) -> bool;
}

#[function_component(BasicInput)]
pub fn basic_input<I>(input: &I) -> Html
where
    I: Input + Labeled + Properties,
{
    // let error = input.validate();

    // let onblur = Callback::from(move |event: FocusEvent| {
    //     let target = event.target().unwrap();
    //     let value = target
    //         .dyn_ref::<web_sys::HtmlInputElement>()
    //         .unwrap()
    //         .value();
    //     input.set(value);
    // });

    html! {
        <div class="form-group">
            <label for={input.label()}>{input.label()}</label>
            <input
                type={I::TYPE}
                class="form-control"
                id={input.label()}
                value={input.value().to_string()}
                // onblur={onblur}
            />
        </div>
    }
}

