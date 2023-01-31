use crate::schema::*;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertableNewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str
}
