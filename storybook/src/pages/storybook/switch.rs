use super::common::*;
use neo::switch::*;
use yew::prelude::*;

#[function_component(SwitchPage)]
pub fn switch_page() -> Html {
    let switch_state = use_state(|| false);

    html! {
        <Wrapper title="Switch">
            <Section title="Default">
                <Switch>{"Enable"}</Switch>
            </Section>

            <Section title="Controllable">
                <Switch checked={*switch_state} onclick={{
                    let switch_state = switch_state.clone();

                    Callback::from(move |_| {
                        switch_state.set(!*switch_state);
                    })
                }}>
                    {"Enable"}
                </Switch>

                <button onclick={Callback::from(move |_| {
                    switch_state.set(!*switch_state);
                })}>
                    {"Toggle"}
                </button>
            </Section>

            <Section title="Default Checked">
                <Switch default_checked={true}>{"Enable"}</Switch>
            </Section>

            <Section title="Disabled">
                <Switch disabled={true}>{"Enable"}</Switch>
            </Section>

            <Section title="Disabled * Checked">
                <Switch disabled={true} default_checked={true}>{"Enable"}</Switch>
            </Section>
        </Wrapper>
    }
}
