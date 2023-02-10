pub mod schema;
mod insertables;
use insertables::{InsertableNewUser};
use diesel::{pg::PgConnection, prelude::*, result::Error::DatabaseError, result::DatabaseErrorKind::UniqueViolation};
use models::{User, NewUser};
use std::error::Error;
use std::str::FromStr;
use lazy_static::lazy_static;
use std::sync::Mutex;
use rand::distributions::{Alphanumeric, DistString};

lazy_static!{
    static ref DATABASE_CONNECTION: Mutex<PgConnection> = Mutex::new({
        let database_url = "postgres://appuser:appuser@localhost/theweb";
        PgConnection::establish(database_url)
            .unwrap_or_else(|_| panic!("Error connecting to url: {}", database_url))
    });
}

pub fn get_all_users() -> Vec<User>{
    use schema::users::dsl::*;
    let results = users
        .load::<User>(&mut *DATABASE_CONNECTION.lock().unwrap())
        .expect("Error loading users!!");
    results
}

pub fn get_user_by_email(mail: &str) -> Result<(String, String), String>{
    use schema::users::dsl::*;
    let res = users.filter(
        email.eq(mail))
        .select((id, password))
        .get_result::<(uuid::Uuid, String)>(&mut *DATABASE_CONNECTION.lock().unwrap());
    match res {
        Ok((uid, hash)) => Ok((uid.to_string(), hash)),
        Err(e) => Err(e.to_string())
    }
}

fn generate_confirmation_token() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 50)
}

pub fn new_user(user: &NewUser) -> Result<String, String>{
    use schema::users::dsl::*;
    let new_user = InsertableNewUser {
        name: &user.name,
        email: &user.email,
        password: &user.password,
        confirmation_token: &generate_confirmation_token(),
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
