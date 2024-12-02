use super::common::*;
use neo::{attributify, checkbox::*, AttrPasser};
use yew::prelude::*;

#[function_component(CheckboxPage)]
pub fn checkbox_page() -> Html {
    let checkbox_state = use_state(|| CheckedState::Unchecked);

    html! {
        <Wrapper title="Checkbox">
            <Section title="Default">
                <Checkbox id="checkbox#1" on_checked_change={Callback::from(move |checked| log::debug!("checked: {checked:?}") )}>
                    {"Accept terms and conditions"}
                </Checkbox>
            </Section>

            <Section title="Default value">
                <Checkbox id="checkbox#2" default_checked={CheckedState::Checked}>
                    {"Accept terms and conditions"}
                </Checkbox>
            </Section>

            <Section title="Controlled">
                <Checkbox id="checkbox#3" checked={(*checkbox_state).clone()} on_checked_change={{
                    let checkbox_state = checkbox_state.clone();
                    Callback::from(move |checked: CheckedState| checkbox_state.set(checked))
                }}>
                    {"Accept terms and conditions: "} {if *checkbox_state == CheckedState::Checked {"+"} else {"-"}}
                </Checkbox>

                <button class="border border-2 p-2 rounded-sm" onclick={Callback::from(move |_| checkbox_state.set(
                    match *checkbox_state {
                        CheckedState::Checked => CheckedState::Unchecked,
                        CheckedState::Indeterminate | CheckedState::Unchecked => CheckedState::Checked,
                    }
                ))}>
                    { "Toggle" }
                </button>
            </Section>

            <Section title="Aria-labelledby">
                <AttrPasser name="checkbox" ..attributify! {
                    "aria-labelledby" => "label#1"
                }>
                    <Checkbox id="checkbox#4" />
                </AttrPasser>

                <label id="label#1" for="checkbox#4">{"Accept terms and conditions"}</label>
            </Section>

            <Section title="ReadOnly">
                <Checkbox id="checkbox#5" readonly={true}>{"Accept terms and conditions: "}</Checkbox>
            </Section>

            <Section title="Disabled">
                <Checkbox id="checkbox#6" disabled={true}>{"Accept terms and conditions: "}</Checkbox>
            </Section>
        </Wrapper>
    }
}
