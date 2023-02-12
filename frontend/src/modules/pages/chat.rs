use yew::prelude::*;
use models::{Chat, Message};
use yew_hooks::use_async;
use crate::modules::{requests::{get_request}};
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
    let chats = use_state(|| vec![]);
    let msg = use_state(|| String::default());
    let current_chat_id = use_state(|| Uuid::default());
    let messages = use_state(|| vec![]);
    {
        let chats = chats.clone();
        let msg = msg.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
              let fetched_chats = get_request::<Vec<Chat>>("/chats").await.unwrap();
              match fetched_chats.content {
                Some(c) => chats.set(c),
                None => msg.set(fetched_chats.message)
              }
            });
            || ()
        }, ());
    }
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
        let msg = msg.clone();
        let messages = messages.clone();
        use_effect_with_deps(move |get_messages| {
            if !get_messages.loading {
                if get_messages.error.is_none() {
                    match get_messages.data.clone() {
                        Some(ans) => {
                            msg.set("OK".to_string());
                            messages.set(ans)
                        },
                        None => msg.set("No messages to show!".to_string())
                    }
                }else {
                    msg.set(get_messages.error.clone().unwrap())
                }
            }
        }, get_messages.clone())
    }

    let messages = messages.iter().map(|message| html! {
        <tr>
            <td>{format!("from: {}",message.user.name) }<br/>{format!("{}", message.content)}<br/><br/></td>
        </tr>
    }).collect::<Html>();

    let chats = (chats).iter().map(|cht| {
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
    html!(
        <>
            <td>
                <table>
                    <h4>{ &*msg }</h4>
                    { chats }
                </table>
                <div></div>
            </td>
            <td> { messages } </td>
        </>
    )
}
