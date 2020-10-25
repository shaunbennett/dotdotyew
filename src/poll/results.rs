use crate::api;
use crate::component::{Panel, PanelBlock, PanelHeading};
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub poll_id: String,
}

struct State {
    results: Option<api::PollResults>,
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
            state: State { results: None },
            tasks: vec![task],
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchSuccess(results) => {
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
                                {format!("{} Votes Submitted", results.votes.len())}
                            </div>
                        </div>
                    </div>
                </PanelHeading>
                { for results.choices.iter().map(|choice| self.show_choice(choice)) }
            </Panel>
        )
    }

    fn show_choice(&self, choice: &api::PollChoice) -> Html {
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
                        // { for (0..votes).map(|_| html!(<span class="icon has-text-info"><i class="fas fa-circle"></i></span>)) }
                        // { if votes > 0 && !self.state.voted { html!(<span class="icon"><div class="delete" onclick=self.link.callback(move |e: MouseEvent| { e.stop_propagation(); Msg::RemoveDot(id) })></div></span>) } else { html!()} }
                    </div>
                </div>
              </div>
            </PanelBlock>
        )
    }
}
