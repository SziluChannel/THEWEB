use web_sys::HtmlInputElement;
use yew::function_component;

use yew::prelude::*;
use yew_hooks::*;
use web_sys::window;

use gloo::console::log;
use models::{LoginUser, ResultMessage};
use crate::modules::requests::{post_request};


#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let login_info = use_state(|| LoginUser::default());

    let login = {
        let login_info = login_info.clone();
        use_async( async move {
            let res = post_request::<LoginUser, ResultMessage>("/users/login", (*login_info).clone()).await;
            match res {
                Ok(result) => {
                    log!(format!("OK: {}", result.message));
                    let session_storage = window().unwrap().session_storage().unwrap().unwrap();
                    match session_storage.set("jwt", &result.message) {
                        Ok(()) => Ok(()),
                        Err(e) => Err(ResultMessage { message: format!("{:#?}", e) })
                    }
                },
                Err(e) => {log!(format!("Error getting token: {:#?}", e)); Err(ResultMessage{message:"Error with getting token!".to_string()})}
            }
        })
    };

    let onsubmit = {
        Callback::from( move |e: SubmitEvent| {
            e.prevent_default();
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
                    <button
                        type="submit">
                        {"Login"}
                    </button>
                </fieldset>
            </form>
        </>
    }
}
