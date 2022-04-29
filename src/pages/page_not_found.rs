use crate::routes::app_routes::AppRoutes;
use yew::prelude::*;
use yew_router::components::Link;

pub struct PageNotFound;

impl Component for PageNotFound {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div class="max-w-screen-xl px-4 md:px-8 mx-auto">
        <div>
          <div class="grid sm:grid-cols-2 gap-8 sm:gap-12">
            <div class="h-80 md:h-auto bg-gray-100 overflow-hidden shadow-lg sm:shadow-none rounded-lg sm:rounded-none">
              <img src="https://images.unsplash.com/photo-1452022449339-59005948ec5b?auto=format&q=75&fit=crop&w=600" loading="lazy" alt="Photo by Jeremy Cai" class="w-full h-full object-cover object-center" />
            </div>
            <div class="flex flex-col justify-center items-center sm:items-start md:py-24 lg:py-32 xl:py-64">
              <p class="text-indigo-500 text-sm md:text-base font-semibold uppercase mb-4">{"Error 404"}</p>
              <h1 class="text-gray-800 text-2xl md:text-3xl font-bold text-center sm:text-left mb-2">{"Page not found"}</h1>
              <p class="text-gray-500 md:text-lg text-center sm:text-left mb-8">{"The page you’re looking for doesn’t exist."}</p>
              <Link<AppRoutes> classes="inline-block bg-gray-200 hover:bg-gray-300 focus-visible:ring ring-indigo-300 text-gray-500 active:text-gray-700 text-sm md:text-base font-semibold text-center rounded-lg outline-none transition duration-100 px-8 py-3d" to={AppRoutes::Home}>{"Go Home"}</Link<AppRoutes>>
            </div>
          </div>
        </div>
      </div>
    }
  }
}
