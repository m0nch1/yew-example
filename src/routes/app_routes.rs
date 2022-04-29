use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoutes {
  #[at("/")]
  Home,
  #[at("/about")]
  About,
  #[not_found]
  #[at("/404")]
  NotFound,
}
