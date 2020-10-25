use crate::api;
use crate::component::{Panel, PanelBlock, PanelHeading};
use std::collections::hash_map::Entry;
use wasm_bindgen::__rt::std::collections::HashMap;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

const COLOURS: [&str; 64] = [
    "#6B6882", "#5FAD4E", "#A75740", "#A5FFD2", "#FFB167", "#009BFF", "#E85EBE", "#005F39",
    "#620E00", "#008F9C", "#98FF52", "#7544B1", "#B500FF", "#00FF78", "#FF6E41", "#FFE502",
    "#91D0CB", "#BE9970", "#968AE8", "#BB8800", "#43002C", "#DEFF74", "#00FFC6", "#0E4CA1",
    "#001544", "#C28C9F", "#FF74A3", "#01D0FF", "#004754", "#E56FFE", "#788231", "#9E008E",
    "#A42400", "#00AE7E", "#683D3B", "#BDC6FF", "#263400", "#BDD393", "#00B917", "#FF0056",
    "#FF937E", "#6A826C", "#FF029D", "#FE8900", "#7A4782", "#7E2DD2", "#85A900", "#D5FF00",
    "#95003A", "#007DB5", "#FF00F6", "#FFEEE8", "#774D00", "#90FB92", "#0076FF", "#010067",
    "#00FF00", "#0000FF", "#FF0000", "#01FFFE", "#FFA6FE", "#FFDB66", "#006401", "#000000",
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
                            colour_index = (colour_index + 1) % 64;
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
                                {format!("{} Votes Submitted", self.state.voter_colours.len())}
                            </div>
                        </div>
                    </div>
                </PanelHeading>
                { for results.choices.iter().map(|choice| self.show_choice(choice)) }
            </Panel>
        )
    }

    fn show_choice(&self, choice: &api::PollChoice) -> Html {
        let votes: Vec<&'static str> = self
            .state
            .results
            .as_ref()
            .unwrap()
            .votes
            .iter()
            .filter(|vote| vote.choice_id == choice.id)
            .flat_map(|vote| {
                let voter = &vote.voter;
                (0..vote.dots).map(move |_| *self.state.voter_colours.get(voter).unwrap())
            })
            .collect();

        html!(
            <PanelBlock tag="a" style="display:block;">
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
                        { for votes.iter().map(|c| html!(<span class="icon" style={format!("color:{};", c)}><i class="fas fa-circle"></i></span>)) }
                    </div>
                </div>
              </div>
            </PanelBlock>
        )
    }
}
