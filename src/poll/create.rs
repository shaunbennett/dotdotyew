use crate::api;
use serde::{Deserialize, Serialize};
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct State {
    title: String,
    choices: Vec<String>,
    loading: bool,
}

pub enum Msg {
    UpdateTitle(String),
    UpdateChoice(usize, String),
    Submit,
    PostSuccess(api::CreatePollResponse),
    PostFailed,
}

pub struct CreatePoll {
    link: ComponentLink<Self>,
    state: State,
    router: RouteAgentDispatcher<()>,
    tasks: Vec<FetchTask>,
}

impl Component for CreatePoll {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: State {
                title: "".into(),
                choices: vec!["".into(); 3],
                loading: false,
            },
            router: RouteAgentDispatcher::new(),
            tasks: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTitle(value) => {
                self.state.title = value;
                true
            }
            Msg::UpdateChoice(i, value) => {
                self.state.choices[i] = value;
                if i == self.state.choices.len() - 1 {
                    self.state.choices.push("".to_owned());
                }
                true
            }
            Msg::Submit => {
                self.state.loading = true;

                let task = api::create_poll(
                    &self.state.title,
                    &self.state.choices,
                    &self.link,
                    |response| {
                        if let (meta, Json(Ok(body))) = response.into_parts() {
                            if meta.status.is_success() {
                                return Msg::PostSuccess(body);
                            }
                        }
                        Msg::PostFailed
                    },
                );
                self.tasks.push(task);
                true
            }
            Msg::PostSuccess(response) => {
                self.router
                    .send(yew_router::agent::RouteRequest::ChangeRoute(
                        yew_router::route::Route::from(crate::AppRoute::Poll(response.poll)),
                    ));
                false
            }
            Msg::PostFailed => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let button_class = if self.state.loading {
            "button is-primary is-loading"
        } else {
            "button is-primary"
        };
        html! {
            <div class="poll">
                <form>
                    <h1 class="title">{"Create a Poll"}</h1>
                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{"Question"}</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control">
                                    <input class="input" type="text" placeholder="How many holes does a straw have?.."
                                        value=&self.state.title oninput=self.link.callback(|e: InputData|
                                        Msg::UpdateTitle(e.value)) />
                                </div>
                            </div>
                        </div>
                    </div>
                    { for self.state.choices.iter().enumerate().map(|(i, _)| self.view_answer(i)) }
                    <div class="field is-grouped is-grouped-right">
                        <p class="control">
                            <a class={button_class} onclick=self.link.callback(|_| Msg::Submit)>
                                {"Create Poll"}
                            </a>
                        </p>
                    </div>
                </form>
            </div>
        }
    }
}

impl CreatePoll {
    fn view_answer(&self, i: usize) -> Html {
        html! {
            <div class="field is-horizontal">
                <div class="field-label is-normal">
                    <label class="label">{if i == 0 { "Answers" } else { "" }}</label>
                </div>
                <div class="field-body">
                    <div class="field has-addons">
                        <p class="control is-expanded">
                            <input class="input" type="text" placeholder="" value=&self.state.choices[i]
                                oninput=self.link.callback(move |e: InputData| Msg::UpdateChoice(i, e.value)) />
                        </p>
                    </div>
                </div>
            </div>
        }
    }
}
