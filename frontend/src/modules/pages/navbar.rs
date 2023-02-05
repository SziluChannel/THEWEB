use yew::prelude::*;
use crate::modules::router::{Route};
use yew_router::prelude::{Link};
use web_sys::window;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let session_storage = window().unwrap().session_storage().unwrap().unwrap();
    let jwt = session_storage.get("jwt").unwrap_or_default().unwrap_or_default();
    let jwt = jwt.trim();
    html! {
        <header>
            <h1>
                <table class="navbar">
                    <tr>
                        <td><Link<Route> to={Route::Root}>{"HOME"}</Link<Route>></td>
                        <td><Link<Route> to={Route::Users}>{"USERS"}</Link<Route>></td>
                        <td><Link<Route> to={Route::Register}>{"REGISTER"}</Link<Route>></td>
                        if jwt != ""{
                            <td><Link<Route> to={Route::Register}>{"LOGOUT"}</Link<Route>></td>
                        }else {
                            <td><Link<Route> to={Route::Login}>{"LOGIN"}</Link<Route>></td>
                        }
                    </tr>
                </table>
            </h1>
        </header>
    }
}
