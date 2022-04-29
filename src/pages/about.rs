use yew::{html, Component, Context, Html};

use crate::layouts::main_layout::MainLayout;

#[derive(PartialEq)]
pub struct Props;

pub struct About;

impl Component for About {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <MainLayout>
        <h1>{"About"}</h1>
      </MainLayout>
    }
  }
}
