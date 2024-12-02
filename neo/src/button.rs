use std::fmt;
use yew::prelude::*;
use yewlish_attr_passer::AttrReceiver;
use yewlish_utils::{enums::Dir, hooks::use_conditional_attr};

use crate::{icon::IconName, spinner::Spinner};

#[derive(Clone, PartialEq)]
pub enum Animation {
    Spinner,
    Pulse,
}

impl fmt::Display for Animation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class = match self {
            Animation::Spinner => "",
            Animation::Pulse => "brix-neo-pulse",
        };
        write!(f, "{class}")
    }
}

#[derive(Clone, Default, PartialEq)]
pub enum ButtonSize {
    #[default]
    Default,
    Wide,
    Compact,
}

impl fmt::Display for ButtonSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class = match self {
            ButtonSize::Default => "brix-neo-btn--default",
            ButtonSize::Wide => "brix-neo-btn--wide",
            ButtonSize::Compact => "brix-neo-btn--compact",
        };
        write!(f, "{class}")
    }
}

#[derive(Clone, Default, PartialEq)]
pub enum ButtonStatus {
    #[default]
    Default,
    Success,
    Alert,
    Warning,
    Info,
    Event,
}

impl fmt::Display for ButtonStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class = match self {
            ButtonStatus::Default => "default",
            ButtonStatus::Success => "success",
            ButtonStatus::Alert => "alert",
            ButtonStatus::Warning => "warning",
            ButtonStatus::Info => "info",
            ButtonStatus::Event => "event",
        };
        write!(f, "{class}")
    }
}

#[derive(Clone, Default, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Tertiary,
}

impl fmt::Display for ButtonVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class = match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Tertiary => "tertiary",
        };
        write!(f, "{class}")
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub r#type: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub form: Option<AttrValue>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub animation: Option<Animation>,
    #[prop_or_default]
    pub badge: Option<AttrValue>,
    #[prop_or(Dir::Ltr)]
    pub dir: Dir,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub status: ButtonStatus,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let show_spinner = props.animation == Some(Animation::Spinner);
    let r#type = props.r#type.clone().unwrap_or_else(|| "button".into());

    let badge = props.badge.as_ref().and_then(|badge| {
        let mut badge = badge
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let badge = if badge.len() > 12 {
            badge.split_off(12)
        } else {
            badge
        };

        AttrValue::from(badge).into()
    });

    use_conditional_attr(
        props.r#ref.clone(),
        "data-badge",
        badge.clone(),
        props.badge.is_some(),
    );

    let variant_classes = vec![
        format!("brix-neo-btn-{}", props.variant),
        format!("brix-neo-btn-{}--{}", props.variant, props.status),
    ];

    html! {
        <AttrReceiver name="button">
            <button
                ref={props.r#ref.clone()}
                id={props.id.clone()}
                class={classes!(
                    "brix-neo-btn",
                    variant_classes,
                    props.animation.as_ref().map(std::string::ToString::to_string),
                    props.size.to_string(),
                    if badge.is_some() { "brix-neo-badge" } else { "" },
                    props.icon.as_ref().map(|icon| format!("brix-neo-icon-{icon}")),
                    &props.class
                )}
                dir={props.dir.clone()}
                type={r#type}
                disabled={props.disabled}
                onclick={props.onclick.clone()}
            >
                {show_spinner.then(|| html! {<Spinner />}).unwrap_or_default()}
                {for props.children.iter()}
            </button>
        </AttrReceiver>
    }
}
