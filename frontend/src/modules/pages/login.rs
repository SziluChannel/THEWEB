use web_sys::HtmlInputElement;
use yew::function_component;

use yew::prelude::*;
use yew_hooks::*;
use web_sys::window;
use yew_router::prelude::{Link, use_navigator};

use gloo::console::log;
use models::{LoginUser};
use crate::modules::{router::Route, requests::{post_request}};

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let login_info = use_state(|| LoginUser::default());
    let navigator = use_navigator().unwrap();
    let login = {
        let login_info = login_info.clone();
        use_async( async move {
            let res = post_request::<LoginUser, String>("/users/login", (*login_info).clone()).await.unwrap();
            match res.content {
                Some(result) => {
                    log!(format!("OK: {}", res.message));
                    let session_storage = window().unwrap().session_storage().unwrap().unwrap();
                    match session_storage.set("jwt", &result) {
                        Ok(()) => Ok("Ok".to_string()),
                        Err(e) => Err(e.as_string().unwrap_or("Error setting token!".to_string()))
                    }
                },
                None => {log!(format!("Error with authentication: {:#?}", res.message)); Err(res.message)}
            }
        })
    };

    use_effect_with_deps(
        move |login| {
            if !login.loading{
                if login.error.is_none() && login.data.is_some() {
                    navigator.push(&Route::Root)
                }
            }
            || ()
        },
        login.clone()
    );

    let onsubmit = {
        let login = login.clone();
        Callback::from( move |_e: SubmitEvent| {
            _e.prevent_default();
            log!("Submit!!");
            login.run();
        })
    };
    let oninput_email = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.email = input.value();
            log!(format!("{:#?}", info));
            login_info.set(info)
        })
    };
    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            log!(format!("{:#?}", info));
            login_info.set(info)
        })
    };
    html! {
        <>
            <form {onsubmit}>
                <fieldset>
                    <input
                        type="email"
                        oninput={oninput_email}
                        placeholder="Email goes here"/>
                </fieldset>
                <fieldset>
                    <input
                        type="password"
                        oninput={oninput_password}
                        placeholder="Password goes here"/>
                </fieldset>
                <fieldset>
                    <label for="submit">{
                        if let Some(e) = &login.error{
                            e
                        }else {
                            "No error"
                        }
                    }</label>
                    <button
                        type="submit">
                        {"Login"}
                    </button>
                </fieldset>
            </form>
            <h1>{"OR"}</h1>
            <h2><Link<Route> to={Route::Register}>{"REGISTER"}</Link<Route>></h2>
        </>
    }
}

