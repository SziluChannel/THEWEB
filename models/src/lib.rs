use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{prelude::*};
use email_address::*;
use chrono::{NaiveDateTime};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct HttpAnswer<T> {
    pub message: String,
    pub content: T,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Queryable)]
pub struct Chat {
    pub id: Uuid,
    pub name: String,
    pub created: NaiveDateTime
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Queryable, Debug)]
pub struct User{
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
    pub confirmation_token: String,
    pub confirmed: bool
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConfirmUser {
    pub name: String,
    pub email: String,
    pub link: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserClaims {
    pub name: String,
    pub email: String,
    pub admin: bool
}

impl From<User> for UserClaims {
    fn from(user: User) -> Self {
        UserClaims {
            name: user.name,
            email: user.email,
            admin: user.admin
        }
    }
}
#[derive(Debug, PartialEq, Queryable, Serialize, Deserialize, Clone)]
pub struct NewUser{
    pub name: String,
    pub email: String,
    pub password: String
}
impl NewUser {
    pub fn validated(&self) -> bool {
        if !self.name.is_empty() && !self.email.is_empty() && !self.password.is_empty() {
            EmailAddress::is_valid(&self.email)
        }
        else{
            false
        }
    }
}
impl Default for NewUser {
    fn default() -> Self {
        NewUser {
            name: String::new(),
            email: String::new(),
            password: String::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String
}
impl Default for LoginUser {
    fn default() -> Self {
        LoginUser {
            email: String::new(),
            password: String::new()
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
