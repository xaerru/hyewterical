use serde::Deserialize;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{
    format::{Json, Nothing},
    prelude::*,
};

#[derive(Deserialize, Debug, Clone)]
pub struct Flags {
    nsfw: bool,
    religious: bool,
    political: bool,
    racist: bool,
    sexist: bool,
    explicit: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    error: bool,
    category: String,
    #[serde(rename = "type")]
    type_field: String,
    setup: String,
    delivery: String,
    flags: Flags,
    id: i64,
    safe: bool,
    lang: String,
}

#[derive(Debug)]
pub enum Msg {
    GetLocation,
    ReceiveResponse(Result<Data, anyhow::Error>),
}

#[derive(Debug)]
pub struct Home {
    fetch_task: Option<FetchTask>,
    data: Option<Data>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Home {
    fn view_joke(&self) -> Html {
        match self.data {
            Some(ref joke) => {
                html! {
                    <>
                      <p>{ format!("{}", joke.setup) }</p>
                      <p>{ format!("{}", joke.delivery) }</p>
                    </>
                }
            }
            None => {
                html! {
                    <header class="app-header">
                    { "Press the button to generate a joke" }
                    </header>
                }
            }
        }
    }
    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching...." }</p> }
        } else {
            html! { <p>{"That's hysterical"}</p> }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            data: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            GetLocation => {
                let request = Request::get("https://v2.jokeapi.dev/joke/Any?type=twopart")
                    .body(Nothing)
                    .expect("Could not build request.");

                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Data, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(location) => {
                        self.data = Some(location);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <div class="app-header">
                <p class="fetch">{self.view_fetching()}</p>
                    <p class="joke">
                      { self.view_joke() } { self.view_error()}
                    </p>
                <button class="button" onclick=self.link.callback( | _ | Msg::GetLocation)> { "Get The Joke!!!" } </button>
            </div>
        }
    }
}
