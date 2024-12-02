use crate::input_wrapper::InputWrapper;
use yew::prelude::*;
use yew::props;
use yewlish_attr_passer::{attributify, AttrPasser, AttrReceiver};
use yewlish_radio_group::{
    RadioGroup as PrimitiveRadioGroup, RadioGroupItem, RadioGroupItemProps,
    RadioGroupItemRenderAsProps, RadioGroupProps as PrimitiveRadioGroupProps,
};
use yewlish_utils::enums::{Dir, Orientation};

#[derive(Clone, PartialEq, Properties)]
pub struct RadioGroupProps {
    pub label: AttrValue,
    pub children: Children,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub default_value: Option<AttrValue>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(Dir::Ltr)]
    pub dir: Dir,
    #[prop_or_default]
    pub orientation: Orientation,
    #[prop_or(true)]
    pub r#loop: bool,
    #[prop_or_default]
    pub on_value_change: Callback<AttrValue>,
    #[prop_or_default]
    pub show_label: bool,
    #[prop_or_default]
    pub hint: Option<AttrValue>,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub has_error: bool,
}

#[function_component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let radio_group_props = props! {
        PrimitiveRadioGroupProps {
            id: props.id.clone(),
            class: props.class.clone(),
            name: props.name.clone(),
            value: props.value.clone(),
            default_value: props.default_value.clone(),
            required: props.required,
            disabled: props.disabled,
            dir: props.dir.clone(),
            orientation: props.orientation.clone(),
            r#loop: props.r#loop,
            on_value_change: props.on_value_change.clone(),
            children: props.children.clone(),
        }
    };

    let label_id = AttrValue::from(format!(
        "{}-label",
        props.id.clone().unwrap_or_else(|| props.label.clone())
    ));

    let hint_id = AttrValue::from(format!(
        "{}-hint",
        props.id.clone().unwrap_or_else(|| props.label.clone())
    ));

    html! {
        <InputWrapper
            disabled={props.disabled}
            has_error={props.has_error}
            required={props.required}
            dir={props.dir.clone()}
        >
            {
                if props.show_label {
                    html! {
                        <label id={label_id.clone()} for={props.id.clone()}>{props.label.clone()}</label>
                    }
                } else {
                    html! {}
                }
            }

            <AttrPasser name="radio-group" ..attributify! {
                "aria-label" => if props.show_label { AttrValue::from("") } else { props.label.clone() },
                "aria-labelledby" => if props.show_label { label_id } else { AttrValue::from("") },
                "aria-describedby" => if props.hint.is_some() { hint_id.clone() } else { AttrValue::from("") },
            }>
                <PrimitiveRadioGroup
                    class={classes!(props.class.clone(), if props.inline { "brix-neo-input-group--inline" } else { "brix-neo-input-group" })}
                    ..radio_group_props
                />
            </AttrPasser>

            {
                if let Some(hint) = &props.hint {
                    html! {
                        <span id={hint_id} class="brix-neo-input-hint">{hint}</span>
                    }
                } else {
                    html! {}
                }
            }
        </InputWrapper>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct RadioProps {
    pub id: AttrValue,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Radio)]
pub fn radio(props: &RadioProps) -> Html {
    let radio_group_item_props = props!(RadioGroupItemProps {
        id: props.id.clone(),
        class: props.class.clone(),
        name: props.name.clone(),
        value: props.value.clone(),
        disabled: props.disabled,
        checked: props.checked,
    });

    let render_as = use_callback((), |props: RadioGroupItemRenderAsProps, ()| {
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
            <AttrReceiver name="radio-group-item">
                <input
                    type="radio"
                    id={props.id.clone().unwrap_or_else(|| AttrValue::from(""))}
                    class={props.class.clone()}
                    name={props.name.clone().unwrap_or_else(|| AttrValue::from(""))}
                    value={props.value.clone().unwrap_or_else(|| AttrValue::from(""))}
                    required={props.required}
                    disabled={props.disabled}
                    checked={props.checked}
                    readonly={props.readonly}
                    onchange={&onchange}
                    onclick={&onclick}
                />
            </AttrReceiver>
        }
    });

    let label_id = AttrValue::from(format!("{}-label", props.id.clone()));

    html! {
        <>
            <AttrPasser name="radio-group-item" ..attributify! {
                "aria-labelledby" => label_id.clone(),
            }>
                <RadioGroupItem
                    class={classes!("", "brix-neo-radio", &props.class)}
                    {render_as}
                    ..radio_group_item_props
                />
            </AttrPasser>

            <label id={label_id} for={props.id.clone()}>
                {props.children.clone()}
            </label>
        </>
    }
}
