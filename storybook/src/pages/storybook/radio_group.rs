use super::common::*;
use neo::radio_group::*;
use yew::prelude::*;

#[function_component(RadioGroupPage)]
pub fn radio_group_page() -> Html {
    html! {
        <Wrapper title="Radio Group">
            <Section title="Default">
                <RadioGroup label="Example #1" show_label={true}>
                    <Radio id="example#1-1" name="color" value="blue">
                        {"Option 1"}
                    </Radio>

                    <Radio id="example#1-2" name="color" value="orange">
                        {"Option 2"}
                    </Radio>

                    <Radio id="example#1-3" name="color" value="plum">
                        {"Option 3"}
                    </Radio>
                </RadioGroup>
            </Section>

            <Section title="Inline">
                <RadioGroup label="Example #2" show_label={true} inline={true}>
                    <Radio id="example#2-1" name="color" value="blue">
                        {"Option 1"}
                    </Radio>

                    <Radio id="example#2-2" name="color" value="orange">
                        {"Option 2"}
                    </Radio>

                    <Radio id="example#2-3" name="color" value="plum">
                        {"Option 3"}
                    </Radio>
                </RadioGroup>
            </Section>

            <Section title="Disabled item">
                <RadioGroup label="Example #3" show_label={true}>
                    <Radio id="example#3-1" name="color" value="blue" disabled={true}>
                        {"Option 1"}
                    </Radio>

                    <Radio id="example#3-2" name="color" value="orange">
                        {"Option 2"}
                    </Radio>

                    <Radio id="example#3-3" name="color" value="plum">
                        {"Option 3"}
                    </Radio>
                </RadioGroup>
            </Section>

            <Section title="Disabled group">
                <RadioGroup label="Example #3" show_label={true} disabled={true}>
                    <Radio id="example#3-1" name="color" value="blue">
                        {"Option 1"}
                    </Radio>

                    <Radio id="example#3-2" name="color" value="orange">
                        {"Option 2"}
                    </Radio>

                    <Radio id="example#3-3" name="color" value="plum">
                        {"Option 3"}
                    </Radio>
                </RadioGroup>
            </Section>
        </Wrapper>
    }
}
