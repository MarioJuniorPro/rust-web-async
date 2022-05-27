use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use db::db::{Database, Db};
use serde::Serialize;
use std::env;
use std::sync::Mutex;

mod db;

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[get("/counter")]
async fn get_counter(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[get("/db")]
async fn get_db(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    let mut db = data.db.clone();
    
    format!("Hello {app_name}! Database name: {}", db.get_database().unwrap())
    // format!("Hello {app_name}!") 
}

#[derive(Debug, Serialize)]
struct AdminPayload {
    name: String,
    role: String,
}

#[get("")]
async fn get_admin() -> impl Responder {
    let adm = AdminPayload {
        name: "mario".to_string(),
        role: "admin".to_string(),
    };

    HttpResponse::with_body(StatusCode::OK, serde_json::to_string(&adm).unwrap())
}

struct AppState {
    app_name: String,
    db: Database,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // env::set_var(
    //     "RUST_LOG",
    //     "simple-auth-server=debug,actix_web=info,actix_server=info",
    // );
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let host = env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap();

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    log::info!("Starting HTTP server: go to http://{}:{}", &host, &port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
                db: Database::new("rust_db".to_string(), 1),
            }))
            .app_data(counter.clone())
            .wrap(Logger::default())
            .service(index)
            .service(get_counter)
            .service(get_db)
            .service(web::scope("/admin").service(get_admin))
    })
    .workers(4)
    .bind((host, port))?
    .run()
    .await
}
