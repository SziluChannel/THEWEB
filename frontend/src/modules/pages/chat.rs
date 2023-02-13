use web_sys::HtmlInputElement;
use yew::prelude::*;
use models::{Chat, User, Message, NewMessage};
use yew_hooks::use_async;
use crate::modules::{requests::{get_request, put_request}};


#[function_component(ChatsGamma)]
pub fn chat_gamma() -> Html {

    let user = {    // get current user
        use_async(async move {
            let tmp = get_request::<User>("/users/current").await.unwrap();
            match tmp.content {
                Some(u) => Ok(u),
                None => Err(tmp.message)
            }
        })
    };
    let current_user = use_state(|| User::default()); //store current user

    let chats = use_state(|| vec![]);   //store list of CHATS
    let chat_message = use_state(|| String::default());     //store the (error)messages of CHATS
    let current_chat = use_state(|| Chat::default());      //store currently viewed CHAT id

    let sender_message = use_state(|| String::from("OK"));   //store the (error)message of the SENDER
    let content = use_state(|| String::default());          //store content of SENDER

    let messages = use_state(|| vec![]);              //store list of MESSAGES

    let get_messages = {        //get MESSAGES for current CHAT
        let current_chat = current_chat.clone();
        use_async(async move {
            let fetched_messages
                = get_request::<Vec<Message>>(&format!("/chats/{}/messages", current_chat.id.as_urn().to_string()))
                .await.unwrap();
            match fetched_messages.content {
                Some(msgs) => Ok(msgs),
                None => Err(fetched_messages.message)
            }
        })
    };

    {       //get CHATS and start getting MESSAGES for that CHAT
        let chats = chats.clone();
        let chat_message = chat_message.clone();
        let current_chat = current_chat.clone();
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
                            current_chat.set(c2.iter().next().unwrap().clone());
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

    {       //get value of current USER
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

    {       //get value of MESSAGES
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

    let send_message = {    //send new MESSAGE
        let get_messages = get_messages.clone();
        let content = content.clone();
        let current_chat = current_chat.clone();
        use_async(async move {
            let message = NewMessage {
                chat_id: current_chat.id,
                user_id: current_chat.id,
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

    {       //get value of  MESSAGE SENDER
        let sender_message = sender_message.clone();
        use_effect_with_deps(move |send_message| {
            if !send_message.loading {
                match &send_message.error {
                    Some(e) => sender_message.set(e.clone()),
                    None => match &send_message.data {
                        Some(e) => sender_message.set(e.clone()),
                        None => sender_message.set(String::from("OK"))
                    }
                }
            }
        }, send_message.clone())
    }

    let messages = messages.iter().map(|message| html! { //format the MESSAGES
        <tr>
            <td style={
                if message.user == *current_user {
                    {"text-align: right;"}
                } else if message.user.email == "chatdaemon@gmail.com" {
                    {"text-align: center;"}
                }
                else {
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

    let chats = chats.iter().map(|cht| {    //format the CHATS
        html! {
            <tr>
                <td style={
                    if cht == &*current_chat {
                        {"background-color: black;"}
                    }else {
                        {""}
                    }
                } onclick={
                    let cht = cht.clone();
                    let current_chat = current_chat.clone();
                    let get_messages = get_messages.clone();
                    Callback::from(move |_| {
                        if *current_chat != cht {
                            current_chat.set(cht.clone());
                            get_messages.run();
                        }
                    })
                }>{format!("{}", cht.name)}</td>
            </tr>
        }
    }).collect::<Html>();


    let send = {    //send NEW MESSAGE
        let content = content.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if !content.is_empty() {
                send_message.run();
                content.set("".to_string());
            }
        })
    };

    let message_text_input = {      //receive text input
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            content.set(e.target_unchecked_into::<HtmlInputElement>().value());
        })
    };

    let show_chats = html! { //element for showing CHATS
        <table>     //table for listing all chats
            <h4>{ &*chat_message }</h4>
            { chats }
        </table>
    };

    let create_chat = html! {   //element for showing NEWCHATS FORM
        <div>
            <table>     //table for creating chat todo!()
            </table>
        </div>
    };

    let left_side = html! {     //left side for showing CHATS section
        <>
            { show_chats }
            { create_chat }
        </>
    };

    let show_messages = html! {    //element for showing MESSAGES
        <>
            <div class={"messages"}>
                <table>     //table for showing messages
                    { messages }
                </table>
            </div>
        </>
    };

    let send_message = html! {      //element for showing SEND MESSAGE FORM
        <>
            <form onsubmit={send.clone()}>
                <table>     //table for sending messages
                    <tr>
                        <td>
                            <fieldset>
                                <input
                                    type={"message"}
                                    style={"width: 85%;"}
                                    oninput={message_text_input}
                                    value={(*content).clone()}
                                    placeholder={"Message here..."}/>
                                <input
                                    type={"submit"}
                                    style={"width: 10%;"}
                                    value={"SEND"}/>
                            </fieldset>
                            <fieldset>
                                <label for="Error"> { &*sender_message } </label>
                            </fieldset>
                        </td>
                    </tr>
                </table>
            </form>
        </>
    };

    let right_side = html! {        //element for showing MESSAGES section
        <>
            { show_messages }
            { send_message }
        </>
    };

    html!(
        <>
            <div class={"chatsdiv"}>
                <table class={"chats"}>
                    <tr>
                        <td>
                            {left_side}
                        </td>
                        <td>
                            {right_side}
                        </td>

                    </tr>
                </table>
            </div>
        </>
    )

}


#[function_component(LeftSide)]
fn left() -> Html {
    html! {
        "left"
    }
}

#[function_component(ListChats)]
fn list_chats() -> Html {
    html! {
        "hi"
    }
}

#[function_component(ChatsPage)]
pub fn chat() -> Html {

    let user = {    // get current user
        use_async(async move {
            let tmp = get_request::<User>("/users/current").await.unwrap();
            match tmp.content {
                Some(u) => Ok(u),
                None => Err(tmp.message)
            }
        })
    };
    let current_user = use_state(|| User::default()); //store current user

    let chats = use_state(|| vec![]);   //store list of CHATS
    let chat_message = use_state(|| String::default());     //store the (error)messages of CHATS
    let current_chat = use_state(|| Chat::default());      //store currently viewed CHAT id

    let sender_message = use_state(|| String::from("OK"));   //store the (error)message of the SENDER
    let content = use_state(|| String::default());          //store content of SENDER

    let messages = use_state(|| vec![]);              //store list of MESSAGES

    let get_messages = {        //get MESSAGES for current CHAT
        let current_chat = current_chat.clone();
        use_async(async move {
            let fetched_messages
                = get_request::<Vec<Message>>(&format!("/chats/{}/messages", current_chat.id.as_urn().to_string()))
                .await.unwrap();
            match fetched_messages.content {
                Some(msgs) => Ok(msgs),
                None => Err(fetched_messages.message)
            }
        })
    };

    {       //get CHATS and start getting MESSAGES for that CHAT
        let chats = chats.clone();
        let chat_message = chat_message.clone();
        let current_chat = current_chat.clone();
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
                            current_chat.set(c2.iter().next().unwrap().clone());
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

    {       //get value of current USER
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

    {       //get value of MESSAGES
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

    let send_message = {    //send new MESSAGE
        let get_messages = get_messages.clone();
        let content = content.clone();
        let current_chat = current_chat.clone();
        use_async(async move {
            let message = NewMessage {
                chat_id: current_chat.id,
                user_id: current_chat.id,
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

    {       //get value of  MESSAGE SENDER
        let sender_message = sender_message.clone();
        use_effect_with_deps(move |send_message| {
            if !send_message.loading {
                match &send_message.error {
                    Some(e) => sender_message.set(e.clone()),
                    None => match &send_message.data {
                        Some(e) => sender_message.set(e.clone()),
                        None => sender_message.set(String::from("OK"))
                    }
                }
            }
        }, send_message.clone())
    }

    let messages = messages.iter().map(|message| html! { //format the MESSAGES
        <tr>
            <td style={
                if message.user == *current_user {
                    {"text-align: right;"}
                } else if message.user.email == "chatdaemon@gmail.com" {
                    {"text-align: center;"}
                }
                else {
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

    let chats = chats.iter().map(|cht| {    //format the CHATS
        html! {
            <tr>
                <td style={
                    if cht == &*current_chat {
                        {"background-color: black;"}
                    }else {
                        {""}
                    }
                } onclick={
                    let cht = cht.clone();
                    let current_chat = current_chat.clone();
                    let get_messages = get_messages.clone();
                    Callback::from(move |_| {
                        if *current_chat != cht {
                            current_chat.set(cht.clone());
                            get_messages.run();
                        }
                    })
                }>{format!("{}", cht.name)}</td>
            </tr>
        }
    }).collect::<Html>();


    let send = {    //send NEW MESSAGE
        let content = content.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if !content.is_empty() {
                send_message.run();
                content.set("".to_string());
            }
        })
    };

    let message_text_input = {      //receive text input
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            content.set(e.target_unchecked_into::<HtmlInputElement>().value());
        })
    };

    let show_chats = html! { //element for showing CHATS
        <table>     //table for listing all chats
            <h4>{ &*chat_message }</h4>
            { chats }
        </table>
    };

    let create_chat = html! {   //element for showing NEWCHATS FORM
        <div>
            <table>     //table for creating chat todo!()
            </table>
        </div>
    };

    let left_side = html! {     //left side for showing CHATS section
        <>
            { show_chats }
            { create_chat }
        </>
    };

    let show_messages = html! {    //element for showing MESSAGES
        <>
            <div class={"messages"}>
                <table>     //table for showing messages
                    { messages }
                </table>
            </div>
        </>
    };

    let send_message = html! {      //element for showing SEND MESSAGE FORM
        <>
            <form onsubmit={send.clone()}>
                <table>     //table for sending messages
                    <tr>
                        <td>
                            <fieldset>
                                <input
                                    type={"message"}
                                    style={"width: 85%;"}
                                    oninput={message_text_input}
                                    value={(*content).clone()}
                                    placeholder={"Message here..."}/>
                                <input
                                    type={"submit"}
                                    style={"width: 10%;"}
                                    value={"SEND"}/>
                            </fieldset>
                            <fieldset>
                                <label for="Error"> { &*sender_message } </label>
                            </fieldset>
                        </td>
                    </tr>
                </table>
            </form>
        </>
    };

    let right_side = html! {        //element for showing MESSAGES section
        <>
            { show_messages }
            { send_message }
        </>
    };

    html!(
        <>
            <div class={"chatsdiv"}>
                <table class={"chats"}>
                    <tr>
                        <td>
                            {left_side}
                        </td>
                        <td>
                            {right_side}
                        </td>

                    </tr>
                </table>
            </div>
        </>
    )
}
