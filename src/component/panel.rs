use crate::component::{Pure, PureComponent};
use yew::prelude::*;

pub type Panel = Pure<PanelImpl>;
pub type PanelHeading = Pure<PanelHeadingImpl>;
pub type PanelBlock = Pure<PanelBlockImpl>;

#[derive(Clone, Properties, PartialEq)]
pub struct PanelImpl {
    pub children: Children,
}

#[derive(Clone, Properties, PartialEq)]
pub struct PanelHeadingImpl {
    pub children: Children,
}

#[derive(Clone, Properties, PartialEq)]
pub struct PanelBlockImpl {
    pub children: Children,
    #[prop_or(false)]
    pub notification: bool,
    #[prop_or(false)]
    pub light: bool,
}

impl PureComponent for PanelImpl {
    fn view(&self) -> Html {
        html! (
            <div class="columns is-mobile is-centered">
                <div class="column is-half-desktop">
                    <div class="panel is-primary">
                        { self.children.clone() }
                    </div>
                </div>
            </div>
        )
    }
}

impl PureComponent for PanelHeadingImpl {
    fn view(&self) -> Html {
        html! (
            <p class="panel-heading">
                { self.children.clone() }
            </p>
        )
    }
}

impl PureComponent for PanelBlockImpl {
    fn view(&self) -> Html {
        let mut class = "panel-block".to_owned();
        if self.notification {
            class += " notification";
        }
        if self.light {
            class += " is-light";
        }
        html! (
            <a class={class}>
                { self.children.clone() }
            </a>
        )
    }
}
