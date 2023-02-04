use actix_web::{get, post, delete, web, web::Json, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use models::{NewUser, LoginUser, ResultMessage};
use db;
use password_hash::{PasswordHasher};
use argon2::Argon2;
use base64::Engine;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::fs;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/users/all")]
async fn get_all_users() -> impl Responder {
    println!("In Users/all!!");
    Json(db::get_all_users())
}

#[post("/users/new")]
async fn new_user(user: Json<NewUser>) -> impl Responder {
    let mut user = user.clone();
    user.password = PasswordHasher::hash_password(
        &Argon2::default(),
        user.password.as_bytes(),
        &base64::engine::general_purpose::STANDARD.encode(b"Hello world~")
    ).expect("Errror with hashing").hash.expect("No good hash!").to_string();
    HttpResponse::Ok().json(
        match db::new_user(&user) {
            Ok(r) => r,
            Err(r) => r
        }
    )
}

#[post("/users/login")]
async fn login_user(user: Json<LoginUser>) -> impl Responder {
    //vertify user using the database backend

    //after successful vertification generate a json web token
    let key = fs::read_to_string("secret.key").unwrap();
    let token = encode(
        &Header::default(),
        &user,
        &EncodingKey::from_secret(key.as_ref()));
    match token {
        Ok(token) => {
            println!("Session token OK: {token}");
            HttpResponse::Ok().json(ResultMessage{ message: token })
        },
        Err(e) => {
            println!("Error with token: {e}");
            HttpResponse::BadRequest().json(ResultMessage{ message: format!("Error with token: {e}") })
        }
    }
}

#[delete("/users/{user}")]
async fn delete_user(user: web::Path<String>) -> impl Responder {
    println!("Deleting user: {}", &user);
    let result = db::delete_user(&user);
    match result {
        Ok(_) => HttpResponse::Ok().body("Ok"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
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
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
