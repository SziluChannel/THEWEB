use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/login")]
    Login,
    #[at("/users")]
    Users,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Root => html! {<h1>{"Das main page"}</h1>},
        Route::Login => html! {"login"},
        Route::Users => html! {"users"},
        Route::NotFound => html! {<h1>{"404"}</h1>},
    }
}
