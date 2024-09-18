use checkbox::CheckboxPage;
use yew::prelude::*;

use crate::Router;

mod checkbox;
mod common;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct StorybookProps {}

#[function_component(StorybookPage)]
pub fn storybook_page(_props: &StorybookProps) -> Html {
    let location = use_context::<Router>().expect("Router Context not found!");

    match location.path.as_str() {
        "checkbox" | "/checkbox" => html! { <CheckboxPage /> },
        _ => html! {{ "Not Found!" }},
    }
}
