use crate::component::{Pure, PureComponent};
use yew::prelude::*;

pub type Panel = Pure<PanelImpl>;
pub type PanelHeading = Pure<PanelHeadingImpl>;
pub type PanelBlock = Pure<PanelBlockImpl>;

#[derive(Clone, Properties, PartialEq)]
pub struct PanelImpl {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, Properties, PartialEq)]
pub struct PanelHeadingImpl {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, Properties, PartialEq)]
pub struct PanelBlockImpl {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_else(|| "".into())]
    pub style: String,
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
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
        let mut classes = Classes::from("panel-block");
        if let Some(extra_classes) = &self.class {
            classes = classes.extend(extra_classes);
        }
        let tag = self.tag.clone();
        html! (
            <@{tag} style=self.style class=classes onclick=self.onclick.clone()>
                { self.children.clone() }
            </@>
        )
    }
}
