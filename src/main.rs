use dotenvy::dotenv;
use routes::init::run;

mod routes;
mod controllers;   
mod db;
mod models;
mod utils;
mod repository;
mod services;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    run().await;
}
