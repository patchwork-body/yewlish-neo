use std::fmt;
use yew::prelude::*;
use yewlish_attr_passer::AttrReceiver;

/// Enum representing the size of the spinner.
#[derive(Clone, PartialEq)]
pub enum SpinnerSize {
    /// Large spinner size.
    Lg,
    /// Extra-large spinner size.
    Xl,
}

impl fmt::Display for SpinnerSize {
    /// Formats the `SpinnerSize` as a CSS class string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class = match self {
            SpinnerSize::Lg => "brix-neo-spinner--large",
            SpinnerSize::Xl => "brix-neo-spinner--x-large",
        };
        write!(f, "{class}")
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SpinnerProps {
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub size: Option<SpinnerSize>,
}

/// `Spinner` component that displays a loading spinner.
///
/// Examples:
/// ```
/// use yew::prelude::*;
/// use neo::spinner::*;
///
/// #[function_component(ExampleSpinner)]
/// pub fn example_spinner() -> Html {
///    html! {
///       <Spinner />
///   }
/// }
///
/// #[function_component(ExampleSpinnerLarge)]
/// pub fn example_spinner_large() -> Html {
///   html! {
///     <Spinner size={SpinnerSize::Lg} />
///  }
/// }
///
/// #[function_component(ExampleSpinnerExtraLarge)]
/// pub fn example_spinner_extra_large() -> Html {
///  html! {
///    <Spinner size={SpinnerSize::Xl} />
/// }
///
/// #[function_component(ExampleSpinnerWithRef)]
/// pub fn example_spinner_with_ref() -> Html {
///  let spinner_ref = NodeRef::default();
///
/// html! {
///   <Spinner r#ref={spinner_ref.clone()} />
/// }
/// ```
///
/// - [Storybook - Checkbox](https://patchwork-body.github.io/yewlish-neo/#/checkbox)
/// - [Avaya Neo Design System](https://design.avayacloud.com/components/web/checkbox-web)
/// - [WAI-ARIA Checkbox Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/checkbox/)
#[function_component(Spinner)]
pub fn spinner(props: &SpinnerProps) -> Html {
    html! {
        <AttrReceiver name="spinner">
            <div
                ref={&props.r#ref}
                id={&props.id}
                class={classes!("brix-neo-spinner", props.size.as_ref().map(std::string::ToString::to_string), &props.class)}
            />
        </AttrReceiver>
    }
}
