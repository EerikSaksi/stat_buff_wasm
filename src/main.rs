use yew::prelude::*;
mod input;
use input::Input;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        html! {
            <div class="flex items-center justify-center h-screen bg-gray-700">
              <form class="flex p-6 bg-white rounded shadow-md justify-evenly">
                  <Input placeholder = "Weight (kg)"/>
                  <p class = "text-xl">{"x"}</p>
                  <Input placeholder = "Reps"/>
              </form>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
