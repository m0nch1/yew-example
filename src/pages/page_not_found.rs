use yew::prelude::*;

pub struct PageNotFound;

impl Component for PageNotFound {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
        <section class={classes!("bg-red-100")}>
            <div class="hero-body">
                <div class="container">
                    <h1 class="title">
                        { "Page not found" }
                    </h1>
                    <h2 class="subtitle">
                        { "Page page does not seem to exist" }
                    </h2>
                </div>
            </div>
        </section>
    }
  }
}
