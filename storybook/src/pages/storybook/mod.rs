use checkbox::CheckboxPage;
use switch::SwitchPage;
use yew::prelude::*;

use crate::Router;

mod checkbox;
pub mod common;
mod switch;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct StorybookProps {}

#[function_component(StorybookPage)]
pub fn storybook_page(_props: &StorybookProps) -> Html {
    let location = use_context::<Router>().expect("Router Context not found!");

    match location.path.as_str() {
        "checkbox" | "/checkbox" | "#/checkbox" => html! { <CheckboxPage /> },
        "switch" | "/switch" | "#/switch" => html! { <SwitchPage /> },
        _ => html! {{ "Not Found!" }},
    }
}
