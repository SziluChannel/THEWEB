pub mod schema;
mod insertables;
use insertables::{InsertableNewUser, InsertableNewChatConnector, InsertableNewChat, InsertableNewMessage};
use diesel::{pg::PgConnection, prelude::*, result, result::Error::DatabaseError, result::DatabaseErrorKind::UniqueViolation};
use models::{User, NewUser, Chat, NewChat, Message, NewMessage, QueryableMessage};
use std::{error::Error, path::PathBuf};
use std::str::FromStr;
use lazy_static::lazy_static;
use std::sync::Mutex;
use rand::distributions::{Alphanumeric, DistString};

use std::env;
use dotenvy::dotenv;

lazy_static!{
    static ref DATABASE_CONNECTION: Mutex<PgConnection> = Mutex::new({
        dotenv().unwrap_or_else(|e| {
            println!("ERROR with dotenvy: {:#?}", e);
            PathBuf::new()
        });
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|e| {println!("Dotenvy error: {}", e.to_string()); String::new()});
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to url: {}", database_url))
    });
    static ref CHAT_DAEMON_ID: uuid::Uuid = {
        use schema::users::dsl::*;
        users.filter(email.eq("szilubot@gmail.com"))
            .get_result::<User>(&mut *DATABASE_CONNECTION.lock().unwrap()).unwrap().id
    };
}

pub fn new_chat(chat: &NewChat) -> Result<(), result::Error> {
    use schema::chats::dsl::*;
    let new_chat = InsertableNewChat{
        name: &chat.name
    };
    let new_chat = diesel::insert_into(chats)
        .values(new_chat)
        .get_result::<Chat>(&mut *DATABASE_CONNECTION.lock().unwrap())?;
    for email in chat.members.iter() {
        create_chat_connector(
            get_user_by_email(email).unwrap().id,
            new_chat.id)?;
    };
    let creator = chat.members.last().unwrap();
    let at = new_chat.created.to_string();
    new_message(NewMessage {
        user_id: *CHAT_DAEMON_ID,
        chat_id: new_chat.id,
        content: format!("{creator} created this chat at {at}!")
    })?;
    Ok(())
}


fn create_chat_connector(user: uuid::Uuid, chat: uuid::Uuid) -> Result<(), result::Error>{
    use schema::chat_connector::dsl::*;
    let new_connector =
    InsertableNewChatConnector {
        chat_id: chat,
        user_id: user
    };
    diesel::insert_into(chat_connector)
        .values(new_connector)
        .execute(&mut *DATABASE_CONNECTION.lock().unwrap())?;
    Ok(())
}

pub fn new_message(message: NewMessage) -> Result<(), result::Error> {
    use schema::messages::dsl::*;
    let message = InsertableNewMessage {
        chat_id: message.chat_id,
        user_id: message.user_id,
        content: &message.content
    };
    let res = diesel::insert_into(messages)
        .values(message)
        .get_result::<QueryableMessage>(&mut *DATABASE_CONNECTION.lock().unwrap());
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

pub fn get_messages_for_chat(cid: uuid::Uuid) -> Result<Vec<Message>, result::Error> {
    use schema::{
        messages,
        users,
        chats,
    };
    let result
        = messages::table
            .inner_join(chats::table)
            .inner_join(users::table)
            .filter(chats::id.eq(cid))
            .select((messages::id, users::all_columns, messages::chat_id, messages::content, messages::created))
            .limit(50)
            .order_by(messages::created)
            .get_results::<Message>(&mut *DATABASE_CONNECTION.lock().unwrap());
    result
}

pub fn get_chats_for_user(uid: uuid::Uuid) -> Result<Vec<Chat>, result::Error> {
    use schema::*;
    let result = chats::table.left_join(
        chat_connector::table::on(
                chat_connector::table,
            chats::id.eq(chat_connector::chat_id)
                .and(chat_connector::user_id.eq(uid))
        ))
        .select((chats::id, chats::name, chats::created))
        .get_results::<Chat>(&mut *DATABASE_CONNECTION.lock().unwrap());
    result
}

pub fn get_all_users() -> Vec<User>{
    use schema::users::dsl::*;
    let results = users
        .load::<User>(&mut *DATABASE_CONNECTION.lock().unwrap())
        .expect("Error loading users!!");
    results
}

pub fn get_user_by_id(uid: uuid::Uuid) -> Result<User, result::Error> {
    use schema::users::dsl::*;
    users.filter(id.eq(uid))
    .get_result::<User>(
        &mut *DATABASE_CONNECTION.lock().unwrap()
    )
}

pub fn validate_token(token: &str) -> Result<(), String> {
    use schema::users::dsl::*;
    let res = diesel::update(users)
        .filter(confirmation_token.eq(token))
        .set(confirmed.eq(true))
        .execute(
            &mut *DATABASE_CONNECTION.lock().unwrap()
    );
    match res {
        Ok(i) => {
            if i == 1 as usize {
                Ok(())
            }else {
                Err("Multiple lines are touched something went terribly wrong!".to_string())
            }
        },
        Err(e) => Err(format!("Error updating table: {}!",e))
    }
}

pub fn get_user_by_email(mail: &str) -> Result<User, String>{
    use schema::users::dsl::*;
    let res = users.filter(
        email.eq(mail))
        .get_result::<User>(&mut *DATABASE_CONNECTION.lock().unwrap());
    match res {
        Ok(u) => Ok(u),
        Err(e) => Err(e.to_string())
    }
}

fn get_all_tokens() -> Result<Vec<String>, result::Error>{
    use schema::users::dsl::*;
    users.select(confirmation_token)
        .get_results::<String>(
            &mut *DATABASE_CONNECTION.lock().unwrap()
        )
}

fn generate_confirmation_token() -> String {
    let mut tok = Alphanumeric.sample_string(&mut rand::thread_rng(), 50);
    let all = get_all_tokens().unwrap();
    println!("Vertification token: {:#?}", all);
    while all.iter().any(|i| i == &tok){
        tok = Alphanumeric.sample_string(&mut rand::thread_rng(), 50);
        println!("Token regenerated!!!!!");
    }
    tok
}

pub fn new_user(user: &NewUser) -> Result<String, String>{
    use schema::users::dsl::*;
    let new_user = InsertableNewUser {
        name: &user.name,
        email: &user.email,
        password: &user.password,
        confirmation_token: &generate_confirmation_token(), //generate token
    };
    let res = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut *DATABASE_CONNECTION.lock().unwrap());
    match res {
        Ok(_) => Ok("Ok".to_string()),
        Err(e) => {
            match e {
                DatabaseError(k, _) => match k {
                    UniqueViolation => Err(String::from("User exists!")),
                    _ => Err(format!("{:#?}", e))
                },
                _ => Err(format!("{:#?}", e))
            }
        }
    }
}

pub fn delete_user(user_id: &str) -> Result<(), Box<dyn Error>>{
    use schema::users::dsl::*;
    let user_id = uuid::Uuid::from_str(user_id)?;
    diesel::delete(
        users.filter(
            id.eq(user_id)
        )).execute(&mut *DATABASE_CONNECTION.lock().unwrap())?;
    Ok(())
}






#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

    }
}
