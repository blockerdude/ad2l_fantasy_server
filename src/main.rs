extern crate ad2l_fantasy_server;
extern crate diesel;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use self::ad2l_fantasy_server::*;
use self::models::*;
use self::diesel::prelude::*;
use ad2l_fantasy_server::schema::conference::dsl::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/users", web::get().to(get_users))
            .route("/friends", web::post().to(get_friends))
                
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn get_friends() -> impl Responder {
    let connection = establish_connection();
    let results = conference
    .limit(5)
    .load::<Conference>(&connection)
    .expect("Error loading posts");

    HttpResponse::Ok().body(serde_json::to_string(&results).unwrap())
}

