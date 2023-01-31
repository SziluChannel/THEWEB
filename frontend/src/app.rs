use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use crate::modules::router::{Route, switch};
use crate::modules::pages::navbar::{Navbar};

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Navbar/>
                <Switch <Route> render={switch}/>
            </BrowserRouter>
        </>
    }
}
