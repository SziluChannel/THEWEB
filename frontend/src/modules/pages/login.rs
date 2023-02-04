use yew::function_component;

use yew::prelude::*;

use gloo::console::log;


#[function_component(LoginForm)]
pub fn login_form() -> Html {

    let onsubmit = {
        Callback::from( move |e: SubmitEvent| {
            e.prevent_default();
            log!("Submit!!")
        })
    };
    html! {
        <>
            <form {onsubmit}>
                <fieldset>
                    <input
                        type="email"
                        placeholder="Email goes here"/>
                </fieldset>
                <fieldset>
                    <input
                        type="password"
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
