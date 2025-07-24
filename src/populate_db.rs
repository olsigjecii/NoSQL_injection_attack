use dotenv::dotenv;
use mongodb::{Client, Collection, bson::doc};
use std::env;

mod models;
use models::{Plant, User};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();
    let mongo_uri =
        env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let client = Client::with_uri_str(&mongo_uri).await?;
    let db = client.database("PerfectPlant");
    let user_collection: Collection<User> = db.collection("users");
    let plant_collection: Collection<Plant> = db.collection("plants");

    // Clear existing data
    user_collection.delete_many(doc! {}, None).await?;
    plant_collection.delete_many(doc! {}, None).await?;

    // Insert Users
    let users = vec![
        User {
            username: "philippe".to_string(),
            password: Some("SuperSecret123".to_string()),
        },
        User {
            username: "gianna".to_string(),
            password: Some("GardeningIsFun".to_string()),
        },
    ];
    user_collection.insert_many(users, None).await?;
    println!("Successfully inserted users");

    // Insert Plants
    let plants = vec![
        Plant {
            name: "Venus Flytrap".to_string(),
            owner: "philippe".to_string(),
        },
        Plant {
            name: "Pitcher Plant".to_string(),
            owner: "philippe".to_string(),
        },
        Plant {
            name: "Sunflower".to_string(),
            owner: "gianna".to_string(),
        },
        Plant {
            name: "Rose".to_string(),
            owner: "gianna".to_string(),
        },
    ];
    plant_collection.insert_many(plants, None).await?;
    println!("Successfully inserted plants");

    Ok(())
}
