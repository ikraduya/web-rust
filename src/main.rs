#[macro_use]
extern crate diesel;
pub mod schema;
pub mod models;

use actix_web::{get, post, web, web::Data, App, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use models::{LoginUser, BookReview, NewBookReview, Comment};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Serialize)]
struct SocialMedia {
    title: String,
    link: String,
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("head_title", "Ikraduya's web");
    data.insert("my_name", "Ikra");

    let socmed = [
        SocialMedia {
            title: String::from("LinkedIn"),
            link: String::from("https://www.linkedin.com/in/ikraduya/"),
        },
        SocialMedia {
            title: String::from("Github"),
            link: String::from("https://github.com/ikraduya"),
        }
    ];

    data.insert("social_media", &socmed);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn book(tera: web::Data<Tera>) -> impl Responder {
    use schema::book_review::dsl::{book_review};

    let mut data = Context::new();
    data.insert("head_title", "Ikraduya's book review");

    let connection = establish_connection();
    let all_reviews : Vec<BookReview> = book_review
        .load(&connection)
        .expect("Error retrieving book reviews");
    
    data.insert("book_reviews", &all_reviews);
    
    let rendered = tera.render("book_review.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn new_book(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("head_title", "Add new book review");
    
    if let Some(id) = id.identity() {
        let rendered = tera.render("new_book.html", &data).unwrap();
        return HttpResponse::Ok().body(rendered)
    }

    HttpResponse::Unauthorized().body("User not logged in")
}

async fn process_new_book(mut data: web::Form<NewBookReview>, id: Identity) -> impl Responder {
    if let Some(id) = id.identity() {
        use schema::book_review;
        
        data.id = None;
        
        let connection = establish_connection();
        diesel::insert_into(book_review::table)
            .values(&*data)
            .get_result::<BookReview>(&connection)
            .expect("Error adding new book review.");
    
        println!("{:?}", data);
        
        return HttpResponse::Ok().body(format!("Successfully saved new book review: {}", data.title))
    }
    HttpResponse::Unauthorized().body("User not logged in")
}

async fn login(tera: Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("head_title", "Login");

    if let Some(id) = id.identity() {
        return HttpResponse::Ok().body("Already logged in")
    }
    let rendered = tera.render("login.html", &data).unwrap();
    return HttpResponse::Ok().body(rendered)
}

async fn process_login(data: web::Form<LoginUser>, id: Identity) -> impl Responder {
    use schema::one_user::dsl::{username, one_user};

    let connection = establish_connection();
    let user = one_user.filter(username.eq(&data.username)).first::<LoginUser>(&connection);

    match user {
        Ok(u) => {
            if u.passcode == data.passcode {
                let session_token = String::from(u.username);
                id.remember(session_token);
                println!("{:?}", data);
                HttpResponse::Ok().body(format!("Logged in: {}. Welcome master", data.username))
            } else {
                HttpResponse::Ok().body("Password is incorrect")
            }
        },
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::Ok().body("User doesn't exist.")
        }
    }
}

async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("Logged out")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let tera = Tera::new("templates/**/*").expect("Failed to parse template file");
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0;32])
                    .name("auth-cookie")
                    .secure(false)
            ))
            .app_data(web::Data::new(tera))
            .service(echo)
            .route("/", web::get().to(index))
            .route("/book", web::get().to(book))
            .route("/new_book", web::get().to(new_book))
            .route("/new_book", web::post().to(process_new_book))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(process_login))
            .route("/logout", web::get().to(logout))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
