use yew::prelude::*;

use crate::components::markup::Markup;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="justify-content-center m-5">

                <div class="d-flex justify-content-center m-5">
                    <h1 class="text-truncate">{"Building a Website in Rust"}</h1>
                </div>

                <div class="container-sm justify-content-center m-5">
                    <Markup id={"homepage.md"}/>
                </div>
            </div>
        }
    }
}
