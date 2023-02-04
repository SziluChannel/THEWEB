use yew::prelude::*;
use web_sys::HtmlInputElement;
use models::{User, NewUser};
use std::error::Error;
use std::{vec};
use gloo_console::log;
use crate::modules::requests::{get_request, post_request, delete_request};
use models::{ResultMessage};


#[function_component(ListUsers)]
pub fn list_users() -> Html {
    let users: UseStateHandle<Vec<User>> = use_state(|| vec![]);
    {
        let users = users.clone();
        use_effect_with_deps(move |_| {
            let users = users.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_users = get_request::<Vec<User>>("/users/all")
                    .await
                    .unwrap();
                users.set(fetched_users);
            });
            || ()
        }, ());
    }
    let users = users.iter().map(|user| html! {
        <tr>
            <td>{format!("{}", user.id)}</td>
            <td>{format!("{}", user.name)}</td>
            <td>{format!("{}", user.email)}</td>
            <td>{format!("{}", user.password)}</td>
        </tr>
    }).collect::<Html>();

    html!{
        <>
            <div>
                <table id="users">
                    <tr>
                        <td>
                            {"ID"}
                        </td>
                        <td>
                            {"Name"}
                        </td>
                        <td>
                            {"Email"}
                        </td>
                        <td>
                            {"Password"}
                        </td>
                    </tr>
                    {users}
                    <NewUserComponent/>
                </table>
            </div>
            <div>
                <DeleteUser/>
            </div>
        </>
    }
}

#[function_component(NewUserComponent)]
fn new_user() -> Html {

    let name_node_ref = use_node_ref();
    let email_node_ref = use_node_ref();
    let password_node_ref = use_node_ref();

    let name_value_handle = use_state(String::default);
    let email_value_handle = use_state(String::default);
    let password_value_handle = use_state(String::default);

    let name_value = (*name_value_handle).clone();
    let email_value = (*email_value_handle).clone();
    let pass_value = (*password_value_handle).clone();

    let on_submit = {
        let name_node_ref = name_node_ref.clone();
        let email_node_ref = email_node_ref.clone();
        let password_node_ref = password_node_ref.clone();

        let value = Callback::from(move |e: SubmitEvent| {
            let name_input = name_node_ref.cast::<HtmlInputElement>();
            let email_input = email_node_ref.cast::<HtmlInputElement>();
            let password_input = password_node_ref.cast::<HtmlInputElement>();
            if let Some(input) = name_input {
                if input.value() == "" {e.prevent_default()}
                else {name_value_handle.set(input.value())}
            }
            if let Some(input) = email_input {
                if input.value() == "" {e.prevent_default()}
                else {email_value_handle.set(input.value())}
            }
            if let Some(input) = password_input {
                if input.value() == "" {e.prevent_default()}
                else{password_value_handle.set(input.value())}
            }
        });

        if !name_value.is_empty() && !email_value.is_empty() && !pass_value.is_empty(){
            send_user(&NewUser{
                name: name_value.to_string(),
                email: email_value.to_string(),
                password: pass_value.to_string()
             });
        }
        value
    };

    html! {
        <tr>
            <td>
                <form onsubmit={on_submit}>
                    <input type="submit" value={"Add user"}/>
                </form>
            </td>
            <td><input type="text" ref={name_node_ref}/></td>
            <td><input type="text" ref={email_node_ref}/></td>
            <td><input type="text" ref={password_node_ref}/></td>
        </tr>
    }
}

fn send_user(user: &NewUser){
    let user = user.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let user = user.clone();
        let result = post_request::<NewUser, ResultMessage>("/users/new", user.clone()) //this part is buggy cause the return type wants to be the same while no text is returned it gives an error
            .await.unwrap();
        log!(format!("Creating user: {:#?} \nResult message: {}", user, result.message));
    });
}

#[function_component(DeleteUser)]
fn del_user() -> Html {
    let deleter_ref = use_node_ref();
    let deleter_handle = use_state(String::new);
    let deleter_value = (*deleter_handle).to_string();


    let onclick = {
        let deleter_ref = deleter_ref.clone();
        let result = Callback::from(move |_| {
            let input = deleter_ref.cast::<HtmlInputElement>();
            if let Some(val) = input {
                deleter_handle.set(val.value())
            }
        });
        if !deleter_value.is_empty() {
            delete_user(&deleter_value).unwrap();
        }
        result
    };

    html!{
        <>
            <br/><br/><br/>
            <h4>
                <form onsubmit={onclick}>
                    {"Delete: "}
                    <input type="submit" value={"DeleteUser"}/>
                    <br/>
                    <input type="id" ref={deleter_ref}/>
                </form>
            </h4>
        </>
    }
}

fn delete_user(id: &str) -> Result<(), Box<dyn Error>> {
    let id = id.to_owned();
    wasm_bindgen_futures::spawn_local(async move {
        let id = id.to_owned();
        let result = delete_request::<()>(&format!("/users/{id}"), ()).await.unwrap();
        log!(format!("Deleting user: {}\nHttp status: {}", &id, result.to_string()));
    });
    Ok(())
}
