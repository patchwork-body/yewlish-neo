use checkbox::CheckboxPage;
use radio_group::RadioGroupPage;
use switch::SwitchPage;
use yew::prelude::*;

use crate::Router;

mod checkbox;
pub mod common;
mod radio_group;
mod switch;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct StorybookProps {}

#[function_component(StorybookPage)]
pub fn storybook_page(_props: &StorybookProps) -> Html {
    let location = use_context::<Router>().expect("Router Context not found!");

    match location.path.as_str() {
        "checkbox" | "/checkbox" | "#/checkbox" => html! { <CheckboxPage /> },
        "switch" | "/switch" | "#/switch" => html! { <SwitchPage /> },
        "radio-group" | "/radio-group" | "#/radio-group" => html! { <RadioGroupPage /> },
        _ => html! {{ "Not Found!" }},
    }
}
