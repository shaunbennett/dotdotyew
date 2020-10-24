use yew::{Component, Properties, Html, ShouldRender, ComponentLink};

pub struct Pure<T>(T);

pub trait PureComponent: Properties +  PartialEq + Sized + 'static {
    fn view(&self) -> Html;
}

impl <T: PureComponent> Component for Pure<T> {
    type Message = ();
    type Properties = T;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Pure(props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.0 != props {
            self.0 = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        self.0.view()
    }
}
