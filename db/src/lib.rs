pub mod schema;
mod insertables;
use insertables::{InsertableNewUser};
use diesel::{pg::PgConnection, prelude::*};
use models::{User, NewUser};
use std::error::Error;
use std::str::FromStr;

pub fn get_all_users() -> Vec<User>{
    use schema::users::dsl::*;
    let connection = &mut establish_connection();
    let results = users
        .load::<User>(connection)
        .expect("Error loading users!!");
    results
}

pub fn get_user_by_email(mail: &str) -> Result<(String, String), String>{
    use schema::users::dsl::*;
    let conn = &mut establish_connection();
    let res = users.filter(
        email.eq(mail))
        .select((id, password))
        .get_result::<(uuid::Uuid, String)>(conn);
    match res {
        Ok((uid, hash)) => Ok((uid.to_string(), hash)),
        Err(e) => Err(e.to_string())
    }
}

pub fn new_user(user: &NewUser) -> Result<String, String>{
    use schema::users::dsl::*;
    let new_user = InsertableNewUser {
        name: &user.name,
        email: &user.email,
        password: &user.password,
    };
    let conn = &mut establish_connection();
    let res = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn);
    match res {
        Ok(_) => Ok("Ok".to_string()),
        Err(e) => Err(format!("{:#?}", e))
    }
}

pub fn delete_user(user_id: &str) -> Result<(), Box<dyn Error>>{
    use schema::users::dsl::*;
    let conn = &mut establish_connection();

    let user_id = uuid::Uuid::from_str(user_id)?;
    diesel::delete(
        users.filter(
            id.eq(user_id)
        )).execute(conn)?;

    Ok(())
}

fn establish_connection() -> PgConnection {
    let database_url = "postgres://appuser:appuser@localhost/theweb";
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to url: {}", database_url))
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
