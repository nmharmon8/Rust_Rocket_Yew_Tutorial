use yew::prelude::*;
use yew_router::prelude::*;


use yew::html::Scope;


#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:id")]
    Post { id: String },
    //#[at("/")]
    //Home,
    //#[not_found]
    //#[at("/404")]
    //NotFound,
}

pub struct App {
}

pub enum Msg {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App { }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>{ "Hello yew!"}</div>
        }
    }
}
