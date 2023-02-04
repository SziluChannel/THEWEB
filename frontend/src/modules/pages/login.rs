use web_sys::HtmlInputElement;
use yew::function_component;

use yew::prelude::*;

use gloo::console::log;
use models::{LoginUser};


#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let login_info = use_state(|| LoginUser::default());
    let onsubmit = {
        Callback::from( move |e: SubmitEvent| {
            e.prevent_default();
            log!("Submit!!")
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
