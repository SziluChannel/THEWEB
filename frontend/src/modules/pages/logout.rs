use yew::prelude::*;
use yew_router::prelude::{use_navigator};
use web_sys::{window};

#[function_component(Logout)]
pub fn logout() -> Html {
    let session_storage = window().unwrap().session_storage().unwrap().unwrap();
    session_storage.delete("jwt").unwrap();
    let navigator = use_navigator().unwrap();
    //navigator.push(&crate::modules::router::Route::Root);
    navigator.replace(&crate::modules::router::Route::Root);
    html! {
        {"Hello logout!"}
    }
}
