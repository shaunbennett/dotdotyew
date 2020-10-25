use crate::api;
use crate::component::{Panel, PanelBlock, PanelHeading};
use yew::prelude::*;

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
}

impl Component for PollResults {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            state: State { results: None },
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
        html!(
            <Panel>
                <PanelHeading>{"Results"}</PanelHeading>
                <PanelBlock>{"Work In Progress"}</PanelBlock>
            </Panel>
        )
    }
}
