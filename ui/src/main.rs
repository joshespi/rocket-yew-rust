use yew::html;
use yew::prelude::*;

//Create the main app that will load all other Components
pub struct App {}

//Message enum that is used for managing the life cycle of Components
pub enum Msg {}

//Implement the Component interface
impl Component for App {
    type Message = Msg;
    type Properties = ();

    //Create a new App
    fn create(_ctx: &Context<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header class="d-flex flex-wrap justify-content-center py-3 mb-4 border-bottom" id="mainHeader">
                <a href="/" class="d-flex align-items-center mb-3 mb-md-0 me-md-auto text-dark text-decoration-none">
                    <img src="/data/images/banner-background.jpg" alt="Espi Avatar" />
                    <span class="fs-4">{"Simple header"}</span>
                </a>
                <ul class="nav nav-pills">
                    <li class="nav-item"><a href="#" class="nav-link" aria-current="page">{"Home"}</a></li>
                    <li class="nav-item"><a href="#" class="nav-link">{"About"}</a></li>
                </ul>
                
            </header>

          }
    }
}

// Entry point for starting the Yew application
pub fn main() {
    //Create the logger
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    //Start the Yew framework
    yew::start_app::<App>();
}
