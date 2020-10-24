use std::collections::HashMap;

use crate::api;
use crate::component::{Panel, PanelBlock, PanelHeading};
use yew::events::MouseEvent;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::storage::{Area, StorageService};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    // required
    pub poll_id: String,
}

struct State {
    voted: bool,
    poll: Option<api::Poll>,
    votes: HashMap<i32, i32>,
    name: String,
    dots_remaining: i32,
}

pub enum Msg {
    AddDot(i32),
    RemoveDot(i32),
    UpdateName(String),
    FetchSuccess(api::Poll),
    SubmitVote,
    FetchFailed,
    VoteSuccess,
    VoteFailed,
}

pub struct ShowPoll {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
    props: Props,
    tasks: Vec<FetchTask>,
}

impl Component for ShowPoll {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("browser storage disabled");

        // On creation, start fetching the poll from the backend
        let task = api::get_poll(&props.poll_id, &link, |response| {
            if let (meta, Json(Ok(body))) = response.into_parts() {
                if meta.status.is_success() {
                    return Msg::FetchSuccess(body);
                }
            }
            Msg::FetchFailed
        });

        let state = {
            if let Json(Ok(votes)) =
                storage.restore(&format!("com.dotdotyew.votes.{}", &props.poll_id))
            {
                State {
                    poll: None,
                    name: "".into(),
                    votes,
                    dots_remaining: 0,
                    voted: true,
                }
            } else {
                State {
                    poll: None,
                    name: "".into(),
                    votes: HashMap::new(),
                    dots_remaining: 2,
                    voted: false,
                }
            }
        };

        Self {
            link,
            storage,
            props,
            state,
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
            Msg::UpdateName(value) => {
                self.state.name = value;
                true
            }
            Msg::AddDot(id) => {
                if self.state.dots_remaining == 0 {
                    return false;
                }
                *self.state.votes.entry(id).or_insert(0) += 1;
                self.state.dots_remaining -= 1;
                true
            }
            Msg::RemoveDot(id) => match self.state.votes.get_mut(&id) {
                Some(v) => {
                    if *v >= 1 {
                        *v -= 1;
                        self.state.dots_remaining += 1;
                        true
                    } else {
                        false
                    }
                }
                None => false,
            },
            Msg::SubmitVote => {
                let task = api::vote(
                    &self.props.poll_id,
                    &self.state.name,
                    self.state.votes.clone(),
                    &self.link,
                    |response| {
                        let (meta, _) = response.into_parts();
                        if meta.status.is_success() {
                            return Msg::VoteSuccess;
                        }
                        Msg::VoteFailed
                    },
                );
                self.tasks.push(task);
                true
            }
            Msg::VoteSuccess => {
                self.state.voted = true;

                self.storage.store(
                    &format!("com.dotdotyew.votes.{}", &self.props.poll_id),
                    Json(&self.state.votes),
                );
                true
            }
            Msg::VoteFailed => {
                // TODO
                true
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
            if self.state.voted {
                self.show_voted(poll)
            } else {
                self.show_can_vote(poll)
            }
        } else {
            html!(
                <h1>{"Loading..."}</h1>
            )
        }
    }
}

impl ShowPoll {
    fn show_can_vote(&self, poll: &api::Poll) -> Html {
        let can_submit = self.state.name.len() > 0 && self.state.dots_remaining == 0;
        html!(
            <Panel>
                <PanelHeading>
                    <div class="level">
                        <div class="level-left">
                            <div class="level-item">
                                {&poll.poll.title}
                            </div>
                        </div>
                        <div class="level-right">
                            <div class="level-item">
                                {format!("Dots Left: {}", self.state.dots_remaining)}
                            </div>
                        </div>
                    </div>
                </PanelHeading>
                <PanelBlock notification=true light=true>
                    <p class="has-text-centered">{"Click on a choice to allocate dots. You must allocate
                        all dots to vote."}</p>
                </PanelBlock>
                { for poll.choices.iter().map(|choice| self.vote_choice(choice)) }
                <PanelBlock>
                    <input class="input is-fullwidth" type="text" placeholder="Your Name..."
                        value=&self.state.name oninput=self.link.callback(|e: InputData|
                        Msg::UpdateName(e.value)) />
                </PanelBlock>
                <PanelBlock>
                    <button class="button is-primary is-fullwidth" disabled={!can_submit} onclick=self.link.callback(|_| Msg::SubmitVote)>
                        {"Submit Votes"}
                    </button>
                </PanelBlock>
            </Panel>
        )
    }

    fn show_voted(&self, poll: &api::Poll) -> Html {
        html!(
            <Panel>
                <PanelHeading>
                    <div class="level">
                        <div class="level-left">
                            <div class="level-item">
                                {&poll.poll.title}
                            </div>
                        </div>
                    </div>
                </PanelHeading>
                <PanelBlock notification=true light=true>
                    <p class="has-text-centered">{"You voted in this poll already"}</p>
                </PanelBlock>
                { for poll.choices.iter().map(|choice| self.vote_choice(choice)) }
                <PanelBlock>
                    <button class="button is-primary is-fullwidth">
                        {"View Results"}
                    </button>
                </PanelBlock>
            </Panel>
        )
    }

    fn vote_choice(&self, choice: &api::PollChoice) -> Html {
        let votes = self.state.votes.get(&choice.id).cloned().unwrap_or(0);
        let id = choice.id;
        html!(
            <a class="panel-block" style="display:block;" onclick=self.link.callback(move |_| Msg::AddDot(id))>
              <div class="level">
                <div class="level-left">
                    <span class="panel-icon">
                        <i class="fas fa-angle-right" aria-hidden="true"></i>
                    </span>
                    <div class="level-item">
                        {&choice.details}
                    </div>
                </div>
                <div class="level-right">
                    <div class="level-item">
                        { for (0..votes).map(|_| html!(<span class="icon has-text-info"><i class="fas fa-circle"></i></span>)) }
                        { if votes > 0 && !self.state.voted { html!(<span class="icon"><div class="delete" onclick=self.link.callback(move |e: MouseEvent| { e.stop_propagation(); Msg::RemoveDot(id) })></div></span>) } else { html!()} }
                    </div>
                </div>
              </div>
            </a>
        )
    }
}
