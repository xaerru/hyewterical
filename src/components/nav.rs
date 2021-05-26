use yew::prelude::*;

/// Nav component
pub struct Nav;

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Nav {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav>
              <ul>
                <li>
                  <a class="app-link" href="https://github.com/grvxs" target="_blank" rel="noopener noreferrer">
                    { "Github" }
                   </a>
                 </li>
              </ul>
            </nav>
        }
    }
}
