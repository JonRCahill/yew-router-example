#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use yew::{ Component, ComponentLink, Html, ShouldRender };

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/second-page"]
    SecondPage,
    #[to = "/"]
    FirstPage,
}


struct AppRoot { }

impl Component for AppRoot {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::SecondPage => html!{ <SecondPage /> },
                            AppRoute::FirstPage => html!{ <FirstPage /> }
                        }
                    })
                />
            </div>
        }
    }
}

struct FirstPage { }

impl Component for FirstPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "First Page" }</h1>
                <RouterAnchor<AppRoute> route=AppRoute::SecondPage>
                    <h4>{{ "Second Page" }}</h4>
                </RouterAnchor<AppRoute>>
            </div>
        }
    }
}

struct SecondPage {
    link: ComponentLink<Self>,
}

enum Msg {
    ButtonPress,
}

impl Component for SecondPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ButtonPress => {
                let mut route_service: RouteService<()> = RouteService::new();
                let route: Route<()> = AppRoute::FirstPage.into();
                route_service.set_route(&route, ());

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "Second Page" }</h1>
                <button
                    onclick=self.link.callback(|_| Msg::ButtonPress)
                >
                    { "Take Me To First Page" }
                </button>
                <RouterAnchor<AppRoute> route=AppRoute::FirstPage>
                    <h4>{{ "First Page" }}</h4>
                </RouterAnchor<AppRoute>>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<AppRoot>::new().mount_to_body();
}
