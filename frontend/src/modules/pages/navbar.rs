use yew::prelude::*;
use crate::modules::router::{Route};
use yew_router::prelude::{Link};
use web_sys::window;
use gloo_console::log;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let trigger = yew::functional::use_force_update();
    let session_storage = window().unwrap().session_storage().unwrap().unwrap();
    let jwt = session_storage.get("jwt").unwrap_or_default().unwrap_or_default();
    let jwt = jwt.trim();
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
                        <td><Link<Route> to={Route::Register}>{"REGISTER"}</Link<Route>></td>
                        if jwt == ""{
                            <td onclick={&*onclick}><Link<Route> to={Route::Login}>{"LOGIN"}</Link<Route>></td>
                        }else {
                            <td onclick={&*onclick}><Link<Route> to={Route::Logout}>{"LOGOUT"}</Link<Route>></td>
                        }
                    </tr>
                </table>
            </h1>
        </header>
    }
}
