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
            <div class="container">
                <h1 class="visually-hidden">{"I am Josh Espinoza"}</h1>
                <Markup id={"homepage.md"}/>



            </div>
        }
    }
}
