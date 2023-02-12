use actix_web::{get, post, put, delete, web, web::Json, HttpRequest, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use models::{NewUser, Chat, Message, User, LoginUser, HttpAnswer, NewMessage, UserClaims};
use db;
use password_hash::{PasswordHasher, PasswordVerifier, PasswordHash};
use argon2::Argon2;
use base64::Engine;
use std::{fs, error::Error};

use jwt_simple::prelude::*;
use lazy_static::lazy_static;


lazy_static! {
    static ref JWT_KEY_PAIR: RS384KeyPair = {
        get_key_from_file().unwrap()
    };
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(format!("Secret key: {:#?}", &*JWT_KEY_PAIR))
}

#[put("/messages/new")]
async fn new_message(req: HttpRequest, mut message: Json::<NewMessage>) -> impl Responder {
    match validate_token(&req) {
        Some(clms) => {
            message.user_id = clms.id;
            match db::new_message(message.clone()) {
                Ok(()) => HttpResponse::Ok().json(HttpAnswer::ok(())),
                Err(e) => HttpResponse::InternalServerError().json(HttpAnswer::<()>::err(e.to_string()))
            }
        },
        None => HttpResponse::Forbidden().json(HttpAnswer::<()>::err("User authentication failed!".to_string()))
    }
}

#[get("/users/current")]
async fn get_current_user(req: HttpRequest) -> impl Responder {
    match validate_token(&req) {
        Some(clm) => {
            match db::get_user_by_id(clm.id){
                Ok(user) => {
                    println!("OKKKKKKKKKKKKKKKKKKK");
                    HttpResponse::Ok().json(HttpAnswer::ok(user))
                },
                Err(e) => HttpResponse::InternalServerError().json(
                    HttpAnswer::<User>::err(e.to_string())
                )
            }
        },
        None => HttpResponse::Forbidden().json(
            HttpAnswer::<User>::err( "User not logged in!".to_string())
        )
    }
}

#[get("/chats/{id}/messages")]
async fn get_messages_for_chat(req: HttpRequest, cid: web::Path<uuid::Uuid>) -> impl Responder{
    println!("Getting messages: cid: {cid}");
    match validate_token(&req) {
        Some(_clm) => {
            match db::get_messages_for_chat(*cid) {
                Ok(msgs) => HttpResponse::Ok().json(
                    HttpAnswer::ok(msgs)),
                Err(e) => HttpResponse::InternalServerError().json(
                    HttpAnswer {
                        message: format!("Error getting data from database: {}", e.to_string()),
                        content: None::<Vec<Message>>
                    }
                )
            }
        },
        None => HttpResponse::Forbidden().json(
            HttpAnswer {
                message: "User not logged in!".to_string(),
                content: None::<Vec<Message>>
            }
        )
    }
}

#[get("/chats")]
async fn chats(req: HttpRequest) -> impl Responder {
    match validate_token(&req) {
        Some(clm) => {
            match db::get_chats_for_user(clm.id) {
                Ok(chs) => HttpResponse::Ok().json(
                    HttpAnswer {
                        message: "OK".to_string(),
                        content: Some(chs)
                    }
                ),
                Err(e) => HttpResponse::InternalServerError().json(
                    HttpAnswer {
                        message: format!("Error while getting data from database: {}", e.to_string()),
                        content: None::<Vec<Chat>>
                    }
                )
            }
        },
        None => HttpResponse::Forbidden().json(
            HttpAnswer {
                message: "User not logged in!".to_string(),
                content: None::<Vec<Chat>>
            }
        )
    }
}

fn get_key_from_file() -> Result<RS384KeyPair, Box<dyn Error>> {
    Ok(
        RS384KeyPair::from_pem(
            &fs::read_to_string("private.pem")?
        )?
    )
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

fn get_jwt_value(request: &HttpRequest) -> Option<String> {
    println!("Headers: {:#?}", request.headers().get("authorization"));
    let tmp = request.headers().get("authorization")?.to_str().unwrap().to_string();
    let mut tmp = tmp.split(" ");
    if tmp.any(|s| s == "bearer") {
        Some(tmp.last().unwrap().to_string())
    }else {
        None
    }
}

fn validate_token(req: &HttpRequest) -> Option<UserClaims> {
    let h = get_jwt_value(&req)?;
    println!("The jwt token value: {h}");
    Some(JWT_KEY_PAIR.public_key().verify_token(&h, None).unwrap().custom)
    //val.buil
}

#[get("/users/all")]
async fn get_all_users(req: HttpRequest) -> impl Responder {
    println!("In Users/all!!\nValidating token...");
    match validate_token(&req) {
        Some(clm) => {
            if clm.admin {
                HttpResponse::Ok().json(
                    HttpAnswer{
                        message: "OK".to_string(),
                        content: Some(db::get_all_users()),
                    }
                )
            }else {
                HttpResponse::Forbidden().json(
                    HttpAnswer{
                        message: "Not admin account!".to_string(),
                        content: None::<Vec<User>>,
                    }
                )
            }
        },
        None => HttpResponse::Forbidden().json(
            HttpAnswer {
                message: "Not logged in no data!".to_string(),
                content: None::<Vec<User>>,
            }
        )
    }

}

#[post("/users/new")]
async fn new_user(user: Json<NewUser>) -> impl Responder {
    let mut user = user.clone();
    user.password = PasswordHasher::hash_password(
        &Argon2::default(),
        user.password.as_bytes(),
        &base64::engine::general_purpose::STANDARD.encode(b"G!J4kf4g3lf434fkjKF%!ZJgK!RK5~")
    ).expect("Errror with hashing").to_string();
    HttpResponse::Ok().json(
        match db::new_user(&user) {
                Ok(_) => HttpAnswer::ok(()),
                Err(r) => HttpAnswer::err(r)
        }

    )
}

fn generate_user_login_token(user: &UserClaims) -> Result<String, Box<dyn Error>> {
    let claims = Claims::with_custom_claims(user.clone(), Duration::from_hours(1));
    Ok(JWT_KEY_PAIR.sign(claims)?)
}

#[post("/users/login")]
async fn login_user(req: HttpRequest, user: Json<LoginUser>) -> impl Responder {

    if !req.head().headers.get("jwt").is_some(){    //check if user is logged in
        //than vertify user using the database backend
        println!("Logging in user: {:#?}", user);
        match db::get_user_by_email(&user.email) {
            Ok(u) => {  //user exists
                println!("User exists!\nChecking password...");
                let hash = PasswordHash::new(&u.password).unwrap();
                if PasswordVerifier::verify_password(&Argon2::default(), user.password.as_bytes(), &hash).is_ok() {
                    println!("Password OK!\nCreating token key!");

                    let token = generate_user_login_token(&UserClaims::from(u));
                    match token {
                        Ok(token) => {
                            println!("Session token OK: {token}");
                            HttpResponse::Ok().json(
                                HttpAnswer{
                                    message: String::from("Successful login!"),
                                    content: Some(token)
                                }
                            )
                        },
                        Err(e) => {
                            println!("Error with token: {e}");
                            HttpResponse::BadRequest().json(
                                HttpAnswer{
                                    message: format!("Error with token: {e}"),
                                    content: None::<String>
                                }
                            )
                        }
                    }
                }else{
                    println!("Invalid password!");
                    HttpResponse::BadRequest().json(
                        HttpAnswer{
                            message: String::from("Invalid Email or Password!"),
                            content: None::<String>
                        }
                    )
                }
            },
            Err(s) => { //User does not exist
                println!("Error getting user: {s}");
                println!("User not found!\nBad username probably");
                HttpResponse::BadRequest().json(
                    HttpAnswer{
                        message: String::from("Invalid email or password (Or something even worse happened!"),
                        content: None::<String>
                    }
                )
            }
        }
    } else { //user is logged in
        println!("User is logged in!");
        HttpResponse::BadRequest().json(
            HttpAnswer{
                message: String::from("User logged in!"),
                content: None::<String>
            }
        )
    }
}

#[delete("/users/{user}")]
async fn delete_user(req: HttpRequest, user: web::Path<String>) -> impl Responder {
    match validate_token(&req){
        Some(clm) => {
            if clm.admin {
                println!("Deleting user: {}", &user);
                return match db::delete_user(&user) {
                    Ok(()) => HttpResponse::Ok().json(
                        HttpAnswer {
                            message: "User successfully deleted!".to_string(),
                            content: Some(()),
                        }
                    ),
                    Err(e) => HttpResponse::InternalServerError().json(
                        HttpAnswer {
                            message: e.to_string(),
                            content: None::<()>,
                        }
                    )
                }
            }else {
                HttpResponse::Forbidden().json(
                    HttpAnswer {
                        message: "Not admin user!".to_string(),
                        content: None::<()>,
                    }
                )
            }
        },
        None => HttpResponse::Forbidden().json(
            HttpAnswer {
                message: "User not logged in!".to_string(),
                content: None::<()>,
            }
        )
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(hello)
            .service(echo)
            .service(get_all_users)
            .service(new_user)
            .service(delete_user)
            .service(login_user)
            .service(chats)
            .service(get_messages_for_chat)
            .service(new_message)
            .service(get_current_user)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
