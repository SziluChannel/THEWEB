use yew::prelude::*;
use crate::modules::router::{Route};
use yew_router::{prelude::{Link}};
use gloo_console::log;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let trigger = yew::functional::use_force_update();
    let onclick = use_state(move || Callback::from(move |_e: MouseEvent| {
        trigger.force_update();
        log!("THe cock inn!!");
    }));
    html! {
        <header>
            <h1>
                <table class="navbar">
                    <tr>
                        <td><Link<Route> to={Route::Root}>{"HOME"}</Link<Route>></td>
                        <td><Link<Route> to={Route::Users}>{"USERS"}</Link<Route>></td>
                        <td onclick={&*onclick}><Link<Route> to={Route::Logout}>{"LOGOUT"}</Link<Route>></td>
                    </tr>
                </table>
            </h1>
        </header>
    }
}
