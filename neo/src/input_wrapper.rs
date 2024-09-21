use yew::prelude::*;
use yewlish_utils::enums::dir::Dir;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct InputWrapperProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub has_error: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub group_class: Option<AttrValue>,
    #[prop_or_default]
    pub wrapper_class: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(Dir::Ltr)]
    pub dir: Dir,
}

#[function_component(InputWrapper)]
pub fn input_wrapper(props: &InputWrapperProps) -> Html {
    html! {
        <div dir={props.dir.clone()} class={classes!(
            "brix-neo-form-control",
            if props.disabled { Some("brix-neo-form-control--disabled") } else { None },
            if props.has_error { Some("brix-neo-form-control--error") } else { None },
            if props.required { Some("brix-neo-form-control--required") } else { None },
            &props.wrapper_class
        )}>
            <div class={classes!(
                "brix-neo-input-group",
                if props.inline { Some("brix-neo-input-group--inline") } else { None },
                &props.group_class
            )}>
                {for props.children.iter()}
            </div>
        </div>
    }
}
