//! Component for the form requiring user name and surname.

use crate::components::forms::*;
use validator::Validate;
use web_common::api::form_traits::TryFromCallback;
use web_common::custom_validators::NotEmpty;
use web_common::database::inserts::new_project::NewProjectName;
use web_common::database::inserts::NewProject;
use web_sys::FormData;
use yew::prelude::*;

impl TryFromCallback<FormData> for FormWrapper<NewProject> {
    fn try_from_callback<C>(data: FormData, callback: C) -> Result<(), Vec<String>>
    where
        C: FnOnce(Result<Self, Vec<String>>) + 'static,
    {
        let project_name = data.get("name").as_string().ok_or_else(|| {
            vec!["The new project name field is missing or not a string.".to_string()]
        })?;

        let description = data
            .get("description")
            .as_string()
            .ok_or_else(|| {
                vec!["The new project description field is missing or not a string.".to_string()]
            })?;

        let public = data
            .get("public")
            .as_string()
            .unwrap_or_else(||"off".to_string());

        let public: bool = InputBool::try_from(public.to_string())?.into();

        let new_project = NewProject::new(project_name, description, public)?;

        callback(Ok(FormWrapper::from(new_project)));

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy, Default, Validate)]
#[repr(transparent)]
pub struct InputBool {
    value: bool,
}

impl From<InputBool> for bool {
    fn from(value: InputBool) -> bool {
        value.value
    }
}

impl From<bool> for InputBool {
    fn from(value: bool) -> InputBool {
        InputBool { value }
    }
}

impl TryFrom<String> for InputBool {
    type Error = Vec<String>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "on" => Ok(InputBool { value: true }),
            "off" => Ok(InputBool { value: false }),
            _ => Err(vec!["Invalid value for boolean.".to_string()]),
        }
    }
}

impl ToString for InputBool {
    fn to_string(&self) -> String {
        if self.value {
            "on".to_string()
        } else {
            "off".to_string()
        }
    }
}

#[function_component(NewProjectForm)]
pub fn complete_profile_form() -> Html {
    html! {
        <BasicForm<NewProject>>
            <BasicInput<NewProjectName> label="Name" input_type={InputType::Text} />
            <BasicInput<NotEmpty> label="Description" input_type={InputType::Textarea} />
            <BasicInput<InputBool> label="Public" value={InputBool::from(true)} input_type={InputType::Checkbox} />
            <Datalist<web_common::database::Taxa> label="Taxon" />
        </BasicForm<NewProject>>
    }
}
