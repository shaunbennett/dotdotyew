#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

pub mod api;
pub mod component;
pub mod poll;
use poll::{CreatePoll, ShowPoll};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/dotdotyew/poll/{id}"]
    Poll(String),
    #[to = "/dotdotyew"]
    Index,
}

struct Layout {
    _link: ComponentLink<Self>,
}

impl Component for Layout {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="section">
                <div class="container">
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Poll(id) => {
                                    html!(<ShowPoll poll_id={id} />)
                                },
                                AppRoute::Index => html!{<CreatePoll/>},
                            }
                        })
                    />
                </div>
            </section>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Layout>::new().mount_to_body();
}
