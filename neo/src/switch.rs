use crate::input_wrapper::InputWrapper;
use yew::prelude::*;
use yew::props;
use yewlish_attr_passer::AttrReceiver;
use yewlish_switch::{Switch as PrimitiveSwitch, SwitchProps as PrimitiveSwitchProps, *};
use yewlish_utils::enums::dir::Dir;

#[derive(Clone, PartialEq, Properties)]
pub struct SwitchProps {
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub default_checked: Option<bool>,
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_checked_change: Callback<bool>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub multiline: bool,
    #[prop_or_default]
    pub has_error: bool,
    #[prop_or(Dir::Ltr)]
    pub dir: Dir,
}

/// A `Switch` consists of a checkbox and some text as label. Thus it allows end-users to toggle between a true/false state.
/// Switch label is passed in as children.  Label placement can be on either side of the checkbox.
/// By default, the checkbox is placed to the left of the label.  To place label to the left of the checkbox,
/// set dir to "rtl" on the parent of a Switch or on the Switch itself.
///
/// Examples:
/// use yew::prelude::*;
/// use neo::{enums::dir::Dir, switch::*};
///
/// #[function_component(ExampleDefault)]
/// pub fn example_with_default_value() -> Html {
///     html! {
///       <Switch>{"Enable"}</Switch>
///     }
/// }
///
/// #[function_component(ExampleRtl)]
/// pub fn example_rtl() -> Html {
///   let checked = use_state(|| CheckedState::Unchecked);
///
///   html! {
///      <Switch dir={Dir::Rtl}>{"Enable"}</Switch>
///   }
/// }
///
/// #[function_component(ExampleControllableState)]
/// pub fn example_controllable_state() -> Html {
///    html! {
///       <>
///           <Switch checked={*switch_state} onclick={{
///               let switch_state = switch_state.clone();
///
///               Callback::from(move |_| {
///                   switch_state.set(!*switch_state);
///               })
///             }}>
///               {"Enable"}
///           </Switch>
///
///           <button onclick={Callback::from(move |_| {
///               switch_state.set(!*switch_state);
///           })}>
///               {"Toggle"}
///           </button>
///      </>
///   }
/// }
///
/// For more information, see:
/// - TODO: add link to the storybook page
/// - [Avaya Neo Design System](https://design.avayacloud.com/components/switch)
/// - [WAI-ARIA Checkbox Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/switch/)
#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let switch_props = props! {
        PrimitiveSwitchProps {
            r#ref: props.r#ref.clone(),
            id: props.id.clone(),
            class: props.class.clone(),
            default_checked: props.default_checked,
            checked: props.checked,
            disabled: props.disabled,
            on_checked_change: props.on_checked_change.clone(),
            required: props.required,
            name: props.name.clone(),
            value: props.value.clone(),
            readonly: props.readonly,
            children: ChildrenWithProps::new(vec![]),
        }
    };

    let render_as = use_callback((), |props: SwitchRenderAsProps, (): &()| {
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
            <AttrReceiver name="switch">
                <input
                    ref={props.r#ref}
                    id={props.id}
                    type="checkbox"
                    role="switch"
                    class={props.class}
                    checked={props.checked}
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

    html! {
        <InputWrapper
            disabled={props.disabled}
            has_error={props.has_error}
            required={props.required}
            dir={props.dir.clone()}
        >
            <label class={classes!(
                "brix-neo-switch",
                if props.multiline { Some("brix-neo-switch--multiline") } else { None },
                if props.disabled { Some("brix-neo-switch--disabled") } else { None },
            )}>
                <PrimitiveSwitch {render_as} ..switch_props />
                <i class="brix-neo-switch__icon" />

                {
                    if props.multiline {
                        html! {
                            <span class="brix-neo-switch-children">{for props.children.iter()}</span>
                        }
                    } else {
                        html! {
                            { for props.children.iter() }
                        }
                    }
                }
            </label>
        </InputWrapper>
    }
}
