use crate::schema::*;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertableNewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub confirmation_token: &'a str
}


#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct InsertableNewMessage<'a> {
    pub user_id: Uuid,
    pub chat_id: Uuid,
    pub content: &'a str,
}
