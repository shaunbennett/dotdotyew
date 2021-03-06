use crate::api;
use crate::component::{Panel, PanelBlock, PanelHeading};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

const COLOURS: [&str; 12] = [
    "#8ecbb7", "#e4aee0", "#88ddad", "#efa6a6", "#6adcdc", "#e8ba85", "#77cdef", "#d7e599",
    "#acb9ec", "#a0c583", "#c6f0ce", "#d1c99a",
];

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub poll_id: String,
}

struct State {
    results: Option<api::PollResults>,
    voter_colours: HashMap<String, &'static str>,
}

pub enum Msg {
    FetchSuccess(api::PollResults),
    FetchFailed,
}

pub struct PollResults {
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    tasks: Vec<FetchTask>,
}

impl Component for PollResults {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = api::get_results(&props.poll_id, &link, |response| {
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
            state: State {
                results: None,
                voter_colours: HashMap::new(),
            },
            tasks: vec![task],
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchSuccess(results) => {
                let mut colour_index: usize = 0;
                for vote in results.votes.iter() {
                    match self.state.voter_colours.entry(vote.voter.clone()) {
                        Entry::Occupied(_) => {}
                        Entry::Vacant(entry) => {
                            entry.insert(COLOURS[colour_index]);
                            colour_index = (colour_index + 1) % 12;
                        }
                    }
                }
                self.state.results = Some(results);
                true
            }
            Msg::FetchFailed => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if let Some(results) = &self.state.results {
            self.show_results(results)
        } else {
            html!(
                <Panel>
                    <PanelHeading/>
                </Panel>
            )
        }
    }
}

impl PollResults {
    fn show_results(&self, results: &api::PollResults) -> Html {
        let title = results.poll.title.clone() + " - Results";
        let votes = self.state.voter_colours.len();
        html!(
            <Panel>
                <PanelHeading>
                    <div class="level">
                        <div class="level-left">
                            <div class="level-item">
                                {title}
                            </div>
                        </div>
                        <div class="level-right">
                            <div class="level-item">
                                {format!("{} Vote{} Submitted", votes, if votes > 1 { "s" } else { "" })}
                            </div>
                        </div>
                    </div>
                </PanelHeading>
                { for results.choices.iter().map(|choice| self.show_choice(choice)) }
            </Panel>
        )
    }

    fn show_choice(&self, choice: &api::PollChoice) -> Html {
        let votes: Vec<(&String, &'static str)> = self
            .state
            .results
            .as_ref()
            .unwrap()
            .votes
            .iter()
            .filter(|vote| vote.choice_id == choice.id)
            .flat_map(|vote| {
                let voter = &vote.voter;
                (0..vote.dots).map(move |_| (voter, *self.state.voter_colours.get(voter).unwrap()))
            })
            .collect();

        html!(
            <PanelBlock style="display:block;">
              <div class="level">
                <div class="level-left">
                    <div class="level-item">
                        <span class="panel-icon">
                            <i class="fas fa-angle-right" aria-hidden="true"></i>
                        </span>
                        {&choice.details}
                    </div>
                </div>
                <div class="level-right">
                    <div class="level-item">
                        { for votes.iter().map(|c| html!(<span class="icon" style={format!("color:{};", c.1)} data-tooltip={c.0}><i class="fas fa-circle"></i></span>)) }
                    </div>
                </div>
              </div>
            </PanelBlock>
        )
    }
}
