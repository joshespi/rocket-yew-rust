mod components;
mod pages;
mod utils;

use pages::home::Home;
use pages::resume::Resume;
use pages::not_found::NotFound;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

//Create the main app that will load all other Components
pub struct App {
    navbar_active: bool,
}

//Message enum that is used for managing the life cycle of Components
pub enum Msg {
    ToggleNavbar,
}

//Implement the Component interface
impl Component for App {
    type Message = Msg;
    type Properties = ();

    //Create a new App
    fn create(_ctx: &Context<Self>) -> Self {
        App {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <BrowserRouter>
              {self.view_nav(&ctx)}
            <main>
                <Switch<Route> render={switch} />
            </main>
          </BrowserRouter>

        }
    }
}

impl App {
    fn view_nav(&self, ctx: &Context<Self>) -> Html {
        let Self { navbar_active } = *self;

        let active_class = if !navbar_active {
            "collapse navbar-collapse"
        } else {
            "navbar-collapse collapse show"
        };
        html! {
            <nav class="navbar navbar-expand-lg p-2 sticky-top navbar-dark bg-dark">

                <Link<Route> classes={classes!("navbar-brand")} to={Route::Home}>

                    <img src="/data/images/espi.jpg" alt="Espi Eth avatar" class="avatar" />
                    {"I Am Josh Espinoza"}
                </Link<Route>>

                <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation"
                    onclick={ctx.link().callback(|_| Msg::ToggleNavbar)}
                >
                    <span class="navbar-toggler-icon"></span>
                </button>

                <div class={classes!(active_class)} id="navbarSupportedContent">
                    <ul class="navbar-nav mr-auto">
                        <li class="nav-item active">
                                <Link<Route> classes={classes!("nav-link")} to={Route::Home}>
                                    { "Home" }
                                </Link<Route>>
                        </li>

                        <li class="nav-item">
                            <a href="https://github.com/joshespi" class="nav-link">
                            {"My Github"}
                            </a>
                        </li>

                        <li class="nav-item">
                            <a href="https://www.youtube.com/user/Joshespi" class="nav-link">
                            {"My Youtube"}
                            </a>
                        </li>
                        <li class="nav-item">
                            <a href="https://twitter.com/The_Espi" class="nav-link">
                            {"My Twitter"}
                            </a>
                        </li>
                        <li class="nav-item">
                            <a href="https://espifam.com/referral-links/" class="nav-link">
                            {"Referral Links"}
                            </a>
                        </li>
                        <li class="nav-item">
                            <a href="https://espifam.com/contact-us/" class="nav-link">
                            {"Contact Me"}
                            </a>
                        </li>
                        <li class="nav-item">
                            <a href="resume/" class="nav-link">
                            {"Resume"}
                            </a>
                        </li>
                    </ul>

                </div>
            </nav>
        }
    }
}

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/resume")]
    Resume,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Resume => {
            html! { <Resume /> }
        }
        _ => {
            html! { <NotFound /> }
        }
    }
}

// Entry point for starting the Yew application
pub fn main() {
    //Create the logger
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    //Start the Yew framework
    yew::Renderer::<App>::new().render();
}
