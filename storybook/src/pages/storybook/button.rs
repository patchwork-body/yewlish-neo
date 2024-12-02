use super::common::*;
use neo::{
    button::{Animation, Button, ButtonVariant},
    icon::IconName,
};
use yew::prelude::*;

#[function_component(ButtonPage)]
pub fn button_page() -> Html {
    html! {
        <>
            <Wrapper title="Button -- Primary">
                <Section title="Default">
                    <Button onclick={Callback::from(|_| log::debug!("clicked"))}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="Disabled">
                    <Button disabled={true}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Icon">
                    <Button icon={IconName::Check}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Animation">
                    <Button animation={Animation::Spinner}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge">
                    <Button badge={"99"}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge and Icon">
                    <Button badge={"99"} icon={IconName::Check}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge and Animation">
                    <Button badge={"99"} animation={Animation::Pulse}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge, Icon and Animation">
                    <Button badge={"99"} icon={IconName::Check} animation={Animation::Spinner}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge, Icon and Animation (Disabled)">
                    <Button badge={"99"} icon={IconName::Check} animation={Animation::Spinner} disabled={true}>
                        {"Click me"}
                    </Button>
                </Section>
            </Wrapper>

            <Wrapper title="Button -- Secondary">
                <Section title="Default">
                    <Button variant={ButtonVariant::Secondary} onclick={Callback::from(|_| log::debug!("clicked"))}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="Disabled">
                    <Button variant={ButtonVariant::Secondary} disabled={true}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Icon">
                    <Button variant={ButtonVariant::Secondary} icon={IconName::Check}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Animation">
                    <Button variant={ButtonVariant::Secondary} animation={Animation::Spinner}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge">
                    <Button variant={ButtonVariant::Secondary} badge={"99"}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge and Icon">
                    <Button variant={ButtonVariant::Secondary} badge={"99"} icon={IconName::Check}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge and Animation">
                    <Button variant={ButtonVariant::Secondary} badge={"99"} animation={Animation::Pulse}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge, Icon and Animation">
                    <Button variant={ButtonVariant::Secondary} badge={"99"} icon={IconName::Check} animation={Animation::Spinner}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge, Icon and Animation (Disabled)">
                    <Button variant={ButtonVariant::Secondary} badge={"99"} icon={IconName::Check} animation={Animation::Spinner} disabled={true}>
                        {"Click me"}
                    </Button>
                </Section>
            </Wrapper>

            <Wrapper title="Button -- Tertiary">
                <Section title="Default">
                    <Button variant={ButtonVariant::Tertiary} onclick={Callback::from(|_| log::debug!("clicked"))}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="Disabled">
                    <Button variant={ButtonVariant::Tertiary} disabled={true}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Icon">
                    <Button variant={ButtonVariant::Tertiary} icon={IconName::Check}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Animation">
                    <Button variant={ButtonVariant::Tertiary} animation={Animation::Spinner}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge">
                    <Button variant={ButtonVariant::Tertiary} badge={"99"}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge and Icon">
                    <Button variant={ButtonVariant::Tertiary} badge={"99"} icon={IconName::Check}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge and Animation">
                    <Button variant={ButtonVariant::Tertiary} badge={"99"} animation={Animation::Pulse}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge, Icon and Animation">
                    <Button variant={ButtonVariant::Tertiary} badge={"99"} icon={IconName::Check} animation={Animation::Spinner}>
                        {"Click me"}
                    </Button>
                </Section>

                <Section title="With Badge, Icon and Animation (Disabled)">
                    <Button variant={ButtonVariant::Tertiary} badge={"99"} icon={IconName::Check} animation={Animation::Spinner} disabled={true}>
                        {"Click me"}
                    </Button>
                </Section>
            </Wrapper>
        </>
    }
}
