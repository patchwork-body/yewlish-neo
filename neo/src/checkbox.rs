#[cfg(feature = "checkbox")]
mod checkbox {
    use yew::prelude::*;
    use yewlish_checkbox::*;

    #[function_component(Checkbox)]
    pub fn checkbox(props: &CheckboxProps) -> Html {
        html! {
            <Checkbox class={classes!("", props.class)} ..props />
        }
    }

    #[function_component(CheckboxIndicator)]
    pub fn checkbox_indicator(props: &CheckboxIndicatorProps) -> Html {
        html! {
            <CheckboxIndicator class={classes!("", props.class)} ..props />
        }
    }
}
