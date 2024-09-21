use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum IconSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl std::fmt::Display for IconSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size_str = match self {
            IconSize::Small => "small",
            IconSize::Medium => "medium",
            IconSize::Large => "large",
        };
        write!(f, "{}", size_str)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum IconStatus {
    Available,
    Away,
    Busy,
    DoNotDisturb,
    Offline,
    Lock,
    Warning,
    Missed,
    Connected,
    Inbound,
    Outbound,
}

impl std::fmt::Display for IconStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size_str = match self {
            IconStatus::Available => "available",
            IconStatus::Away => "away",
            IconStatus::Busy => "busy",
            IconStatus::DoNotDisturb => "do-not-disturb",
            IconStatus::Offline => "offline",
            IconStatus::Lock => "lock",
            IconStatus::Warning => "warning",
            IconStatus::Missed => "missed",
            IconStatus::Connected => "connected",
            IconStatus::Inbound => "inbound",
            IconStatus::Outbound => "outbound",
        };
        write!(f, "{}", size_str)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum IconName {
    Check,
}

impl std::fmt::Display for IconName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon_name_str = match self {
            IconName::Check => "check",
        };
        write!(f, "{}", icon_name_str)
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct IconProps {
    pub name: IconName,
    pub aria_label: AttrValue,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub size: IconSize,
    #[prop_or_default]
    pub status: Option<IconStatus>,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    if props.size == IconSize::Small && props.status.is_some() {
        panic!("Status icons are not supported in small size.");
    }

    html! {
        <span
            role="img"
            class={classes!(
                format!("brix-neo-icon-{}", props.name),
                props.status.as_ref().map(|status| format!("brix-neo-icon-state brix-neo-icon-state--{}", status)),
                props.status.as_ref().map(|_| if props.size == IconSize::Large { "brix-neo-icon-state--large".to_string() } else { format!("brix-neo-icon--{}", props.size) }),
                &props.class,
            )}
        />
    }
}
