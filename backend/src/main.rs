use actix_web::{get, post, delete, web, web::Json, HttpRequest, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use models::{NewUser, User, LoginUser, HttpAnswer, UserClaims};
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
        HttpAnswer{
            message: String::from("OK"),
            content: match db::new_user(&user) {
                Ok(_) => Ok(()),
                Err(r) => Err(r)
            }
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
async fn delete_user(req: HttpRequest, user: web::Path<String>) -> impl Responder {
    match validate_token(&req){
        Some(clm) => {
            if clm.admin {
                println!("Deleting user: {}", &user);
                return match db::delete_user(&user) {
                    Ok(()) => HttpResponse::Ok().json(
                        HttpAnswer {
                            message: "User successfully deleted!".to_string(),
                            content: Ok::<(), String>(()),
                        }
                    ),
                    Err(e) => HttpResponse::InternalServerError().json(
                        HttpAnswer {
                            message: "Something went wrong while deleting user!".to_string(),
                            content: Err::<(), String>(e.to_string()),
                        }
                    )
                }
            }else {
                HttpResponse::Forbidden().json(
                    HttpAnswer {
                        message: "Not admin user!".to_string(),
                        content: Err::<(), String>("Not admin user!".to_string()),
                    }
                )
            }
        },
        None => HttpResponse::Forbidden().json(
            HttpAnswer {
                message: "User not logged in!".to_string(),
                content: Err::<(), String>("Not admin user!".to_string()),
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
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
