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

pub fn new_user(user: &NewUser) {
    use schema::users::dsl::*;
    let new_user = InsertableNewUser {
        name: &user.name,
        email: &user.email,
        password: &user.password,
    };
    let conn = &mut establish_connection();
    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
        .expect("Error adding new user!");
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
