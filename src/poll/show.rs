use crate::api;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    // required
    pub poll_id: String,
}

struct State {
    poll: Option<api::Poll>,
}

pub enum Msg {
    FetchSuccess(api::Poll),
    FetchFailed,
}

pub struct ShowPoll {
    link: ComponentLink<Self>,
    state: State,
    props: Props,
    tasks: Vec<FetchTask>,
}

impl Component for ShowPoll {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // On creation, start fetching the poll from the backend
        let task = api::get_poll(&props.poll_id, &link, |response| {
            if let (meta, Json(Ok(body))) = response.into_parts() {
                if meta.status.is_success() {
                    return Msg::FetchSuccess(body);
                }
            }
            Msg::FetchFailed
        });

        Self {
            link,
            props,
            state: State { poll: None },
            tasks: vec![task],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchSuccess(poll) => {
                self.state.poll = Some(poll);
                true
            }
            Msg::FetchFailed => {
                // TODO: Error handling
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            // Haven't tested this code path, but I think we just want to refetch when this happens
            self.state.poll = None;
            let task = api::get_poll(&props.poll_id, &self.link, |response| {
                if let (meta, Json(Ok(body))) = response.into_parts() {
                    if meta.status.is_success() {
                        return Msg::FetchSuccess(body);
                    }
                }
                Msg::FetchFailed
            });
            self.tasks.push(task);
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        if let Some(poll) = &self.state.poll {
            html!(
                <div class="content">
                    <h1>{&poll.poll.title}</h1>
                    <ul>
                        { for poll.choices.iter().map(|choice| html!(<li>{&choice.details}</li>)) }
                    </ul>
                </div>
            )
        } else {
            html!(
                <h1>{"Loading..."}</h1>
            )
        }
    }
}
