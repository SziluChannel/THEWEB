use web_sys::HtmlInputElement;
use yew::prelude::*;
use models::{Chat, User, Message, NewMessage};
use yew_hooks::use_async;
use crate::modules::{requests::{get_request, put_request}};
use uuid::Uuid;

#[function_component(ChatsPage)]
pub fn chat() -> Html {
    html! {
        <>
            <div class={"chatsdiv"}>
                <table class={"chats"}>
                    <tr>
                        <Chats/>
                    </tr>
                </table>
            </div>
        </>
    }
}

#[function_component(Chats)]
fn chats() -> Html {
    let user = {
        use_async(async move {
            let tmp = get_request::<User>("/users/current").await.unwrap();
            match tmp.content {
                Some(u) => Ok(u),
                None => Err(tmp.message)
            }
        })
    };
    let current_user = use_state(|| User::default());

    let chats = use_state(|| vec![]);
    let chat_message = use_state(|| String::default());
    let sender_message = use_state(|| String::default());
    let current_chat_id = use_state(|| Uuid::default());
    let messages = use_state(|| vec![]);
    let content = use_state(|| String::default());

    let get_messages = {
        let current_chat_id = current_chat_id.clone();
        use_async(async move {
            let fetched_messages
                = get_request::<Vec<Message>>(&format!("/chats/{}/messages", current_chat_id.as_urn().to_string()))
                .await.unwrap();
            match fetched_messages.content {
                Some(msgs) => Ok(msgs),
                None => Err(fetched_messages.message)
            }
        })
    };
    {
        let chats = chats.clone();
        let chat_message = chat_message.clone();
        let current_chat_id = current_chat_id.clone();
        let get_messages = get_messages.clone();
        let user = user.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                user.run();
                let fetched_chats = get_request::<Vec<Chat>>("/chats").await.unwrap();
                match fetched_chats.content {
                    Some(c) => {
                        if c.len() > 0 {
                            let c2 = c.clone();
                            current_chat_id.set(c2.iter().next().unwrap().id);
                            get_messages.run()
                        }
                        chats.set(c);
                    },
                    None => chat_message.set(fetched_chats.message)
                }
            });
            || ()
        }, ());
    }
    {
        let current_user = current_user.clone();
        use_effect_with_deps(move |user| {
            if !user.loading {
                if user.error.is_none() {
                    match user.data.clone() {
                        Some(u) => {
                            current_user.set(u)
                        },
                        None => current_user.set(User::default())
                    }
                }else {
                    current_user.set(User::default())
                }
            }
        }, user.clone())
    }
    {
        let chat_message = chat_message.clone();
        let messages = messages.clone();
        use_effect_with_deps(move |get_messages| {
            if !get_messages.loading {
                if get_messages.error.is_none() {
                    match get_messages.data.clone() {
                        Some(ans) => {
                            chat_message.set("OK".to_string());
                            messages.set(ans)
                        },
                        None => chat_message.set("No messages to show!".to_string())
                    }
                }else {
                    chat_message.set(get_messages.error.clone().unwrap())
                }
            }
        }, get_messages.clone())
    }

    let send_message = {
        let get_messages = get_messages.clone();
        let content = content.clone();
        let current_chat_id = *current_chat_id.clone();
        use_async(async move {
            let message = NewMessage {
                chat_id: current_chat_id,
                user_id: current_chat_id,
                content: String::from(&*content)
            };
            get_messages.run();
            let res = put_request("/messages/new", message).await.unwrap();
            match res.content {
                Some(()) => Ok(res.message),
                None => Err(res.message)
            }
        }
    )};

    {
        let sender_message = sender_message.clone();
        use_effect_with_deps(move |send_message| {
            if !send_message.loading {
                match &send_message.error {
                    Some(e) => sender_message.set(e.clone()),
                    None => match &send_message.data {
                        Some(e) => sender_message.set(e.clone()),
                        None => sender_message.set(String::default())
                    }
                }
            }
        }, send_message.clone())
    }

    let messages = messages.iter().map(|message| html! {
        <tr>
            <td style={
                if message.user == *current_user {
                    {"text-align: right;"}
                }else {
                    {"text-align: left;"}
                }
            }>
                <b>
                    {format!("from: {}",message.user.name) }</b><br/>{format!("{}", message.content)}
                <br/>
                <br/>
            </td>
        </tr>
    }).collect::<Html>();

    let chats = chats.iter().map(|cht| {
        html! {
            <tr>
                <td onclick={
                    let cht = cht.clone();
                    let current_chat_id = current_chat_id.clone();
                    let get_messages = get_messages.clone();
                    Callback::from(move |_| {
                        if *current_chat_id != cht.id {
                            current_chat_id.set(cht.id);
                            get_messages.run();
                        }
                    })
                }>{format!("{}", cht.name)}</td>
            </tr>
        }
    }).collect::<Html>();


    let send = {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            send_message.run()
        })
    };

    let text_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            content.set(e.target_unchecked_into::<HtmlInputElement>().value())
        })
    };
    html!(
        <>
            <td>
                <table>
                    <h4>{ &*chat_message }</h4>
                    { chats }
                </table>
                <table>
                </table>
                <div></div>
            </td>
            <td>
                <div class={"messages"}>
                    <table>
                        { messages }
                    </table>
                </div>
                <form onsubmit={send}>
                    <table>
                        <tr>
                            <td>
                                <fieldset>
                                    <input
                                        type={"message"}
                                        oninput={text_input}
                                        placeholder={"Message here..."}/>
                                </fieldset>
                                <fieldset>
                                    <input
                                        type={"submit"}
                                        value={"SEND"}/>
                                </fieldset>
                                <fieldset>
                                    <label for="Error"> { &*sender_message } </label>
                                </fieldset>
                            </td>
                        </tr>
                    </table>
                </form>
            </td>
        </>
    )
}
