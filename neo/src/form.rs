use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct FormProps {
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
}

/// Extends `<form>` with Avaya NEO styling.
///
/// Examples:
/// ```
/// <Form>
///     <input />
/// </Form>
///
/// <Form inline={true}>
///     <input />
/// </Form>
/// ```
///
/// For more information, see:
/// - TODO: add link to the storybook page
/// - [Avaya Neo Design System](https://design.avayacloud.com/components/web/form-layout-web)
#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let inline_class = if props.inline {
        Some("brix-neo-form--inline")
    } else {
        None
    };

    html! {
        <form class={classes!("brix-neo-form", inline_class, &props.class)}>
            {for props.children.iter()}
        </form>
    }
}
