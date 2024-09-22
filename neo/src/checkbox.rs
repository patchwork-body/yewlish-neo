use yew::{prelude::*, props};
use yewlish_attr_passer::AttrReceiver;
pub use yewlish_checkbox::CheckedState;
use yewlish_checkbox::{Checkbox as PrimitiveCheckbox, CheckboxProps as PrimitiveCheckboxProps, *};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub default_checked: Option<CheckedState>,
    #[prop_or_default]
    pub checked: Option<CheckedState>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub on_checked_change: Callback<CheckedState>,
    #[prop_or_default]
    pub children: Children,
}

/// Checkboxes are used when several choices are available and multiple selections are allowed.
///
/// Examples:
/// ```
/// use yew::prelude::*;
/// use neo::{attributify, checkbox::*, AttrPasser};
///
/// #[function_component(ExampleWithDefaultValue)]
/// pub fn example_with_default_value() -> Html {
///     html! {
///        <Checkbox id="checkbox-1" default_checked={CheckedState::Checked}>
///            {"Checkbox"}
///        </Checkbox>
///     }
/// }
///
/// #[function_component(ExampleControllableState)]
/// pub fn example_controllable_state() -> Html {
///   let checked = use_state(|| CheckedState::Unchecked);
///
///   html! {
///      <Checkbox checked={*checked} on_checked_change={Callback::from(move |checked| *checked = checked)}>
///         {"Checkbox"}
///     </Checkbox>
///   }
/// }
///
/// #[function_component(ExampleAriaLabelledBy)]
/// pub fn example_aria_labelled_by() -> Html {
///    html! {
///       <>
///         <AttrPasser name="checkbox" ..attributify! {
///            "aria-labelledby" => "label-1"
///         }>
///           <Checkbox id="checkbox-1">
///             {"Checkbox"}
///           </Checkbox>
///         </AttrPasser>
///
///         <label id="label-1">{"Label"}</label>
///      </>
///   }
/// }
/// ```
///
/// For more information, see:
/// - TODO: add link to the storybook page
/// - [Avaya Neo Design System](https://design.avayacloud.com/components/web/checkbox-web)
/// - [WAI-ARIA Checkbox Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/checkbox/)
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let checkbox_props = props! {
        PrimitiveCheckboxProps {
            r#ref: props.r#ref.clone(),
            id: props.id.clone(),
            class: props.class.clone(),
            default_checked: props.default_checked.clone(),
            checked: props.checked.clone(),
            disabled: props.disabled,
            on_checked_change: props.on_checked_change.clone(),
            required: props.required,
            name: props.name.clone(),
            value: props.value.clone(),
            readonly: props.readonly,
            children: ChildrenWithProps::new(vec![]),
        }
    };

    let render_as = use_callback((), |props: CheckboxRenderAsProps, (): &()| {
        let onchange = Callback::from({
            let toggle = props.toggle.clone();

            move |_event: Event| {
                toggle.emit(());
            }
        });

        let onclick = Callback::from({
            let readonly = props.readonly;

            move |event: MouseEvent| {
                if readonly {
                    event.prevent_default();
                }
            }
        });

        html! {
            <AttrReceiver name="checkbox">
                <input
                    ref={props.r#ref}
                    id={props.id}
                    type="checkbox"
                    role="checkbox"
                    class={props.class}
                    checked={props.checked == CheckedState::Checked}
                    required={props.required}
                    name={props.name}
                    value={props.value}
                    readonly={props.readonly}
                    disabled={props.disabled}
                    onchange={&onchange}
                    onclick={&onclick}
                />
            </AttrReceiver>
        }
    });

    let indeterminate_class = match props.checked {
        Some(CheckedState::Indeterminate) => Some("brix-neo-check--indeterminate"),
        _ => None,
    };

    let readonly_class = if props.readonly {
        Some("brix-neo-check-readonly")
    } else {
        None
    };

    html! {
        <>
            <PrimitiveCheckbox class={classes!("brix-neo-check", indeterminate_class, readonly_class, &props.class)} {render_as} ..checkbox_props />
            <label for={&props.id}>{props.children.clone()}</label>
        </>
    }
}
