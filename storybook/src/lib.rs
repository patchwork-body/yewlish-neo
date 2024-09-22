use std::{collections::HashMap, rc::Rc};
#[cfg(target_arch = "wasm32")]
use web_sys::wasm_bindgen::UnwrapThrowExt;
#[cfg(feature = "hash-based-routing")]
use web_sys::wasm_bindgen::{prelude::Closure, JsCast};
use yew::prelude::*;

mod pages;

use pages::{Link, StorybookPage};

#[derive(Clone, Debug, PartialEq)]
struct Router {
    pub path: AttrValue,
    pub query: Rc<HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Properties)]
pub struct AppProps {
    pub path: AttrValue,
    pub query: Rc<HashMap<String, String>>,
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let router = {
        #[cfg(not(target_arch = "wasm32"))]
        let router = use_memo(
            (props.path.clone(), props.query.clone()),
            |(path, query)| Router {
                path: path.into(),
                query: query.clone(),
            },
        );

        #[cfg(target_arch = "wasm32")]
        let router = {
            let path = use_state(|| {
                let window = web_sys::window().unwrap_throw();

                let pathname = {
                    #[cfg(feature = "hash-based-routing")]
                    let pathname = window.location().hash().unwrap_throw();

                    #[cfg(not(feature = "hash-based-routing"))]
                    let pathname = window.location().pathname().unwrap_throw();

                    pathname
                };

                pathname
            });

            #[cfg(feature = "hash-based-routing")]
            use_effect_with((), {
                let path = path.clone();

                |()| {
                    let window = web_sys::window().unwrap_throw();

                    let closure = Closure::wrap(Box::new({
                        let window = window.clone();

                        move || {
                            path.set(window.location().hash().unwrap_throw());
                        }
                    }) as Box<dyn Fn()>);

                    window
                        .add_event_listener_with_callback(
                            "hashchange",
                            closure.as_ref().unchecked_ref(),
                        )
                        .unwrap_throw();

                    move || {
                        window
                            .remove_event_listener_with_callback(
                                "hashchange",
                                closure.as_ref().unchecked_ref(),
                            )
                            .unwrap_throw();
                    }
                }
            });

            use_memo(path.clone(), |path| {
                let window = web_sys::window().unwrap_throw();
                let location = window.location();
                let search = location.search().unwrap_throw();

                Router {
                    path: (**path).clone().into(),
                    query: Rc::new(
                        search
                            .trim_start_matches('?')
                            .split('&')
                            .filter_map(|pair| {
                                let mut pair = pair.split('=');

                                let key = pair.next()?;
                                let value = pair.next()?;

                                Some((key.to_string(), value.to_string()))
                            })
                            .collect(),
                    ),
                }
            })
        };

        router
    };

    html! {
        <ContextProvider<Router> context={(*router).clone()}>
            <div class="flex flex-col min-h-screen bg-neutral-950 text-white">
                <aside>
                    <nav>
                        <li>
                            <Link href="/checkbox">{"Checkbox"}</Link>
                        </li>

                        <li>
                            <Link href="/switch">{"Switch"}</Link>
                        </li>
                    </nav>
                </aside>

                <StorybookPage />
            </div>
        </ContextProvider<Router>>
    }
}
