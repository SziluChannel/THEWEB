use actix_web::{get, post, delete, web, web::Json, HttpRequest, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use models::{NewUser, LoginUser, HttpAnswer};
use db;
use password_hash::{PasswordHasher, PasswordVerifier, PasswordHash};
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
    ).expect("Errror with hashing").to_string();
    HttpResponse::Ok().json(
        HttpAnswer{
            message: String::from("OK"),
            content: match db::new_user(&user) {
                Ok(_) => Ok(()),
                Err(r) => Err(r)
            }
        }
    )
}

#[post("/users/login")]
async fn login_user(req: HttpRequest, user: Json<LoginUser>) -> impl Responder {

    if !req.head().headers.get("jwt").is_some(){    //check if user is logged in
        //than vertify user using the database backend
        println!("Logging in user: {:#?}", user);
        match db::get_user_by_email(&user.email) {
            Ok((id, hash)) => {  //user exists
                println!("User exists!\nChecking password...");
                let hash = PasswordHash::new(&hash).unwrap();
                if PasswordVerifier::verify_password(&Argon2::default(), user.password.as_bytes(), &hash).is_ok() {
                    println!("Password OK!");
                    let key = fs::read_to_string("secret.key").unwrap();
                    let token = encode(
                        &Header::default(),
                        &id,
                        &EncodingKey::from_secret(key.as_ref()));
                    match token {
                        Ok(token) => {
                            println!("Session token OK: {token}");
                            HttpResponse::Ok().json(
                                HttpAnswer{
                                    message: String::from("Successful login!"),
                                    content: Ok::<String, String>(token)
                                }
                            )
                        },
                        Err(e) => {
                            println!("Error with token: {e}");
                            HttpResponse::BadRequest().json(
                                HttpAnswer{
                                    message: format!("Error with token: {e}"),
                                    content: Err::<String, String>(format!("Error with token: {e}"))
                                }
                            )
                        }
                    }
                }else{
                    println!("Invalid password!");
                    HttpResponse::BadRequest().json(
                        HttpAnswer{
                            message: String::from("Invalid Password!"),
                            content: Err::<String, String>(format!("Invalid email or password!"))
                        }
                    )
                }
            },
            Err(s) => { //User does not exist
                println!("Error getting user: {s}");
                println!("User not found!\nBad username probably");
                HttpResponse::BadRequest().json(
                    HttpAnswer{
                        message: String::from("Invalid email or password!"),
                        content: Err::<String, String>(format!("Invalid email or password! (or someting bad happened in the background)"))
                    }
                )
            }
        }
    } else { //user is logged in
        println!("User is logged in!");
        HttpResponse::BadRequest().json(
            HttpAnswer{
                message: String::from("User logged!"),
                content: Err::<String, String>(format!("You are logged in!"))
            }
        )
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
