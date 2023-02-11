pub mod schema;
mod insertables;
use insertables::{InsertableNewUser};
use diesel::{pg::PgConnection, prelude::*, result, result::Error::DatabaseError, result::DatabaseErrorKind::UniqueViolation};
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

pub fn get_user_by_id(uid: uuid::Uuid) -> Result<User, result::Error> {
    use schema::users::dsl::*;
    users.filter(id.eq(uid))
    .get_result::<User>(
        &mut *DATABASE_CONNECTION.lock().unwrap()
    )
}

pub fn generate_new_token(_email: &str) -> Result<(), String> {
    todo!()
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
