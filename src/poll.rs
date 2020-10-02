use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct PollModel {
    link: ComponentLink<Self>,
    state: PollState,
}

pub struct PollState {
    question: String,
    answer_value: String,
    answers: Vec<String>,
}

pub enum PollMsg {
    UpdateQuestion(String),
    SubmitAnswer,
    UpdateAnswer(String),
    SubmitPoll,
}

impl Component for PollModel {
    type Message = PollMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: PollState {
                question: "".into(),
                answer_value: "".into(),
                answers: Vec::new(),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PollMsg::UpdateQuestion(value) => {}
            PollMsg::UpdateAnswer(value) => {
                self.state.answer_value = value;
            }
            PollMsg::SubmitAnswer => {
                self.state.answers.push(self.state.answer_value.to_string());
                self.state.answer_value = "".to_string();
            }
            PollMsg::SubmitPoll => todo!(),
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
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
                                    <input class="input" type="text" placeholder="How many holes does a straw have?.." />
                                </div>
                            </div>
                        </div>
                    </div>
                    { for self.state.answers.iter().map(|q| self.view_answer(q)) }
                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{""}</label>
                        </div>
                        <div class="field-body">
                            <div class="field is-grouped">
                                <p class="control is-expanded">
                                    <input class="input" type="text" placeholder="One" value=&self.state.answer_value
                                        oninput=self.link.callback(|e: InputData| PollMsg::UpdateAnswer(e.value)) />
                                </p>
                                <p class="control">
                                    <a onclick=self.link.callback(|_| PollMsg::SubmitAnswer) class="button is-info">
                                        {"+"}
                                    </a>
                                </p>
                            </div>
                        </div>
                    </div>
                </form>
            </div>
        }
    }
}

impl PollModel {
    fn view_answer(&self, answer: &str) -> Html {
        html! {
            <div class="field is-horizontal">
                <div class="field-label is-normal">
                    <label class="label">{""}</label>
                </div>
                <div class="field-body">
                    <div class="field has-addons">
                        <p class="control is-expanded">
                            <input class="input" type="text" value={answer} />
                        </p>
                        <p class="control">
                            <a onclick=self.link.callback(|_| PollMsg::SubmitAnswer) class="button is-danger">
                                {"x"}
                            </a>
                        </p>
                    </div>
                </div>
            </div>
        }
    }
}
