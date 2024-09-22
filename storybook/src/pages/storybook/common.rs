use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct WrapperProps {
    pub title: AttrValue,
    pub children: Children,
}

#[function_component(Wrapper)]
pub fn wrapper(props: &WrapperProps) -> Html {
    html! {
        <div class="min-w-screen flex flex-col gap-y-10 p-20 brix-neo-dark">
            <h2 class="text-xl whitespace-nowrap text-neutral-200">{props.title.clone()}</h2>

            <div class="flex flex-wrap items-center gap-10">
                {props.children.clone()}
            </div>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SectionProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    pub children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let class = classes!(
        "flex",
        "flex-col",
        "flex-1",
        "gap-y-5",
        "items-center",
        "border",
        "rounded-md",
        "p-5",
        "border-neutral-600",
        "focus-within:border-neutral-100",
        "text-neutral-400",
        "focus-within:text-neutral-100",
        "hover:text-neutral-100",
        "transition-colors",
        props.class.as_ref()
    );

    html! {
        <section class={class}>
            <h3 class="text-lg whitespace-nowrap text-inherit">{props.title.clone()}</h3>
            {props.children.clone()}
        </section>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LinkProps {
    pub href: String,
    pub children: Children,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let href = {
        #[cfg(feature = "hash-based-routing")]
        let href = {
            if props.href.starts_with('/') {
                format!("#{}", &props.href)
            } else {
                format!("#/{}", &props.href)
            }
        };

        #[cfg(not(feature = "hash-based-routing"))]
        let href = {
            if props.href.starts_with('/') {
                props.href[1..].to_string()
            } else {
                props.href.clone()
            }
        };

        href
    };

    html! {
        <a
            class="text-blue-400 hover:text-blue-300"
            href={href.clone()}
        >
            {props.children.clone()}
        </a>
    }
}
