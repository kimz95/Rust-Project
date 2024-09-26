
use crate::database::db;
use database::db::MIGRATIONS;
use dotenv::dotenv;
use actix_web::{middleware::Logger, App, HttpServer};
use actix_cors::Cors;
use actix_web::web::Data;
use config::Config;

mod database;
mod config;
mod routes;
mod health;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    dotenv().ok();

    let database_url = Config::from_env().database_url;
    let pool = db::establish_connection(&database_url);
    db::run_migrations(&mut pool.get().expect("Unable to get db connection"), MIGRATIONS).expect("Failed to run migrations.");

    HttpServer::new(move||{
        let cors = Cors::permissive();
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(routes::init)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .workers(4)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
