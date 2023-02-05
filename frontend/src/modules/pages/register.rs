use gloo_console::log;
use yew_hooks::{use_async};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use models::{NewUser};
use crate::modules::requests::{post_request};

#[function_component(Register)]
pub fn sign_up() -> Html {
    let user_info = use_state(|| NewUser::default());
    let error = use_state(|| String::from("OK"));
    let create_user = {
        let user_info = user_info.clone();
        use_async(async move {
            let result = post_request::<NewUser, Result<(), String>>("/users/new", (*user_info).clone()).await;
            log!(format!("Result data: {:#?}", result));
            match result {
                Ok(answer) => {
                    match answer.content {
                        Ok(()) => {
                            log!(format!("OK: {}",answer.message));
                            Ok(())
                        }
                        Err(e) => {
                            log!(format!("Error: {e}"));
                            Err(e)
                        }
                    }
                },
                Err(e) => Err(e.to_string())
            }
        })
    };

    let onsubmit = {
        let user_info = user_info.clone();
        let error = error.clone();
        Callback::from(move |_e: SubmitEvent| {
            _e.prevent_default();
            let error = error.clone();
            log!("Submitted new user...");
            if !user_info.email.is_empty() && !user_info.name.is_empty() && !user_info.password.is_empty() {
                create_user.run();
                if create_user.error != None {
                    log!(format!("Error result: {:#?}", create_user.error));
                    error.set(create_user.error.clone().unwrap_or("OK".to_string()));
                }
            }else {
                log!("Incomplete form!");
                error.set(String::from("Incomplete form!"));
            }
        }
    )};

    let name_input = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*user_info).clone();
            info.name = e.target_unchecked_into::<HtmlInputElement>().value();
            user_info.set(info)
        }
    )};
    let email_input = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*user_info).clone();
            info.email = e.target_unchecked_into::<HtmlInputElement>().value();
            user_info.set(info)
        }
    )};
    let password_input = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*user_info).clone();
            info.password = e.target_unchecked_into::<HtmlInputElement>().value();
            user_info.set(info)
        }
    )};
    html!(
        <form {onsubmit}>
            <fieldset>
                <label for="name">{"Name:"}</label>
                <input
                    type="name"
                    oninput={name_input}
                    placeholder="Name goes here"/>
            </fieldset>
            <fieldset>
                <label for="email">{"Email:"}</label>
                <input
                    type="email"
                    oninput={email_input}
                    placeholder="Emil goes here"/>
            </fieldset>
            <fieldset>
                <label for="password">{"Password:"}</label>
                <input
                    type="password"
                    oninput={password_input}
                    placeholder="Give a pass..."/>
            </fieldset>
            <fieldset>
                <h3><label for="Error">{ (*error).clone() }</label></h3>
            </fieldset>
            <fieldset>
                <input
                    type="submit"
                    value={"Add user"}/>
            </fieldset>
        </form>
    )
}
