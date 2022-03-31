use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
struct Props {
    placeholder: String,
}
#[function_component(Form)]
pub fn form(props: &Props) -> Html {
    html! {
         <form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
            <div class="mb-4">
              <input class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Username"/>
            </div>
          </form>
    }
}
