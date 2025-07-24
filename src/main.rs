use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use handlebars::Handlebars;
use mongodb::{Client, Collection};
use std::env;

mod handlers;
mod models;

use models::{Plant, User};

pub struct AppState {
    user_collection: Collection<User>,
    plant_collection: Collection<Plant>,
    hb: Handlebars<'static>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_uri =
        env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to connect to MongoDB");
    let db = client.database("PerfectPlant");
    let user_collection: Collection<User> = db.collection("users");
    let plant_collection: Collection<Plant> = db.collection("plants");

    let mut hb = Handlebars::new();
    hb.register_templates_directory(".hbs", "./templates")
        .expect("Failed to register templates directory");

    let app_state = web::Data::new(AppState {
        user_collection,
        plant_collection,
        hb,
    });

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::resource("/").route(web::get().to(handlers::index)))
            .service(
                web::scope("/vulnerable")
                    .route("/login", web::get().to(handlers::login_page))
                    .route("/login", web::post().to(handlers::vulnerable_login))
                    .route("/search", web::post().to(handlers::vulnerable_search)),
            )
            .service(
                web::scope("/secure")
                    .route("/login", web::get().to(handlers::login_page))
                    .route("/login", web::post().to(handlers::secure_login))
                    .route("/search", web::post().to(handlers::secure_search)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
