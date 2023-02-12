use yew::prelude::*;
use web_sys::HtmlInputElement;
use models::{User, NewUser};
use yew_hooks::use_async;
use std::{vec};
use gloo_console::log;
use crate::modules::requests::{get_request, post_request, delete_request};


#[function_component(ListUsers)]
pub fn list_users() -> Html {
    let error = use_state(|| String::default());
    let users: UseStateHandle<Vec<User>> = use_state(|| vec![]);
    {
        let users = users.clone();
        let error = error.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_users = get_request::<Vec<User>>("/users/all")
                    .await.unwrap();
                match fetched_users.content {
                    Some(u) => users.set(u),
                    None => error.set(format!("Error getting users: {}", fetched_users.message))
                }
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
            <td>{format!("{}", user.admin)}</td>
            <td>{format!("{}", user.confirmed)}</td>
            <td>{format!("{}", user.confirmation_token)}</td>
        </tr>
    }).collect::<Html>();

    html!{
        <>
            <div>
                <h4>{ &*error }</h4>
                <table class={"users"}>
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
                        <td>
                            {"Admin"}
                        </td>
                        <td>
                            {"Confirmed"}
                        </td>
                        <td>
                            {"Token"}
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
        let result = post_request::<NewUser, String>("/users/new", user.clone()) //this part is buggy cause the return type wants to be the same while no text is returned it gives an error
            .await.unwrap();
        log!(format!("Creating user: {:#?} \nResult message: {}", user, result.message));
    });
}

#[function_component(DeleteUser)]
fn del_user() -> Html {
    let deleter_handle = use_state(|| String::default());
    let msg = use_state(|| String::default());

    let delete_user = {
        let deleter_handle = deleter_handle.clone();
        use_async(async move {
            match delete_request::<()>(&format!("/users/{}", *deleter_handle), ()).await {
                Ok(a) => Ok(a),
                Err(e) => Err(e.to_string())
            }

        })
    };

    {
        let msg = msg.clone();
        let delete_user = delete_user.clone();
        use_effect_with_deps(move |delete_user| {
            if !delete_user.loading {
                match &delete_user.error {
                    Some(e) => msg.set(e.to_string()),
                    None => {
                        match &delete_user.data {
                            Some(d) => {
                                match &d.content {
                                    Some(_) => msg.set("Ok".to_string()),
                                    None => msg.set(d.message.clone())
                                }
                            },
                            None => msg.set(String::default())
                        }
                    }
                }
            }
        }, delete_user)
    }

    let onclick = {
        let deleter_handle = deleter_handle.clone();
        let delete_user = delete_user.clone();
        let result = Callback::from(move |e: SubmitEvent| {
            if !deleter_handle.is_empty() {
                log!("Deleter value not empty going forward...");
                delete_user.run();
                //delete_user(&deleter_value).unwrap();
            }else {
                e.prevent_default();
            }
        });
        result
    };

    let delete_input = {
        let deleter_handle = deleter_handle.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target_unchecked_into::<HtmlInputElement>().value();
            deleter_handle.set(val)
        })
    };

    html!{
        <>
            <br/><br/><br/>
            <h4>
                <form onsubmit={onclick}>
                    {"Delete: "}
                    <input type="submit" value={"DeleteUser"}/>
                    <br/>
                    <input type="id" oninput={delete_input}/>
                    <br/>
                    <h4>{ &*msg }</h4>
                </form>
            </h4>
        </>
    }
}
/*
fn delete_user(id: &str) -> Result<(), Box<dyn Error>> {
    let id = id.to_owned();
    wasm_bindgen_futures::spawn_local(async move {
        let id = id.to_owned();
        let result = delete_request::<()>(&format!("/users/{id}"), ()).await.unwrap();
        log!(format!("Deleting user: {}\nHttp status: {}", &id, result.to_string()));
    });
    Ok(())
}
 */
