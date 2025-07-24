use crate::{AppState, Plant, User};
use actix_web::{HttpResponse, Responder, web};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, to_bson};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct VulnerableLoginPayload {
    username: String,
    password: Value,
}

pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let body = data.hb.render("index", &serde_json::json!({})).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn login_page(data: web::Data<AppState>) -> impl Responder {
    let body = data.hb.render("login", &serde_json::json!({})).unwrap();
    HttpResponse::Ok().body(body)
}

//  Vulnerable Login Handler
pub async fn vulnerable_login(
    data: web::Data<AppState>,
    payload: web::Json<VulnerableLoginPayload>,
) -> impl Responder {
    // Construct the query, converting the incoming password JSON value to a BSON value
    let query = doc! {
        "username": &payload.username,
        "password": to_bson(&payload.password).unwrap(),
    };

    match data.user_collection.find_one(query, None).await {
        Ok(Some(user)) => {
            let plants = find_plants_by_owner(&data, &user.username).await;
            let body = data
                .hb
                .render("plants", &serde_json::json!({ "plants": plants }))
                .unwrap();
            HttpResponse::Ok().body(body)
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}

// Secure Login Handler
pub async fn secure_login(data: web::Data<AppState>, form: web::Form<User>) -> impl Responder {
    let query = doc! {
        "username": &form.username,
        "password": &form.password,
    };

    match data.user_collection.find_one(query, None).await {
        Ok(Some(user)) => {
            let plants = find_plants_by_owner(&data, &user.username).await;
            let body = data
                .hb
                .render("plants", &serde_json::json!({ "plants": plants }))
                .unwrap();
            HttpResponse::Ok().body(body)
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}

// Vulnerable Search Handler
pub async fn vulnerable_search(data: web::Data<AppState>, search_query: String) -> impl Responder {
    let query = doc! { "$where": format!("this.name == '{}'", search_query) };

    let cursor = match data.plant_collection.find(query, None).await {
        Ok(cursor) => cursor,
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let plants: Vec<Plant> = match cursor.try_collect().await {
        Ok(docs) => docs,
        Err(_) => vec![],
    };

    let body = data
        .hb
        .render("plants", &serde_json::json!({ "plants": plants }))
        .unwrap();
    HttpResponse::Ok().body(body)
}

// Secure Search Handler
pub async fn secure_search(data: web::Data<AppState>, search_query: String) -> impl Responder {
    let query = doc! { "name": search_query };

    let cursor = match data.plant_collection.find(query, None).await {
        Ok(cursor) => cursor,
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let plants: Vec<Plant> = match cursor.try_collect().await {
        Ok(docs) => docs,
        Err(_) => vec![],
    };

    let body = data
        .hb
        .render("plants", &serde_json::json!({ "plants": plants }))
        .unwrap();
    HttpResponse::Ok().body(body)
}

async fn find_plants_by_owner(data: &web::Data<AppState>, owner: &str) -> Vec<Plant> {
    let query = doc! { "owner": owner };
    let cursor = match data.plant_collection.find(query, None).await {
        Ok(cursor) => cursor,
        Err(_) => return vec![],
    };

    match cursor.try_collect().await {
        Ok(docs) => docs,
        Err(_) => vec![],
    }
}
