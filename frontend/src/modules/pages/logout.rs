use yew::prelude::*;
use yew_router::prelude::{use_navigator};
use web_sys::{window};
use gloo::console::log;

#[function_component(Logout)]
pub fn logout() -> Html {
    let session_storage = window().unwrap().session_storage().unwrap().unwrap();
    session_storage.delete("jwt").unwrap();
    let navigator = use_navigator().unwrap();
    log!("Logging out!!");
    //navigator.push(&crate::modules::router::Route::Root);
    navigator.replace(&crate::modules::router::Route::Login);
    html! {
        {"Hello logout!"}
    }
}
