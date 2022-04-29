use yew::prelude::*;
use yew_router::prelude::*;

mod layouts;
mod pages;
use pages::about::About;
use pages::home::Home;
use pages::page_not_found::PageNotFound;

mod routes;
use routes::app_routes::AppRoutes;

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <BrowserRouter>
            <Switch<AppRoutes> render={Switch::render(switch)} />
          </BrowserRouter>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

fn switch(routes: &AppRoutes) -> Html {
    match routes {
        AppRoutes::Home => html! { <Home /> },
        AppRoutes::About => html! { <About /> },
        AppRoutes::NotFound => html! { <PageNotFound /> },
    }
}
