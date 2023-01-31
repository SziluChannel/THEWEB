use actix_web::{get, post, delete, web, web::Json, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use models::{NewUser};
use db;

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
    //HttpResponse::Ok()
}

#[post("/users/new")]
async fn new_user(user: Json<NewUser>) -> impl Responder {
    HttpResponse::Ok().body(db::new_user(&user))
}

#[delete("/users/{user}")]
async fn delete_user(user: web::Path<String>) -> impl Responder {
    println!("Deleting user: {}", &user);
    let result = db::delete_user(&user);
    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
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
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
