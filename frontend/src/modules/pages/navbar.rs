use yew::prelude::*;
use crate::modules::router::{Route};
use yew_router::prelude::{Link};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <header>
            <h1>
                <table class="navbar">
                    <tr>
                        <td><Link<Route> to={Route::Root}>{"HOME"}</Link<Route>></td>
                        <td><Link<Route> to={Route::Users}>{"USERS"}</Link<Route>></td>
                        <td><Link<Route> to={Route::Login}>{"LOGIN"}</Link<Route>></td>
                        <td><Link<Route> to={Route::Register}>{"REGISTER"}</Link<Route>></td>
                        <td><Link<Route> to={Route::Register}>{"LOGOUT"}</Link<Route>></td>
                    </tr>
                </table>
            </h1>
        </header>
    }
}
