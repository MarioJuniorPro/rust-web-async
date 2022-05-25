use actix_web::http::StatusCode;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::env;
use std::sync::Mutex;


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
            }))
            .app_data(counter.clone())
            .wrap(Logger::default())
            .service(index)
            .service(get_counter)
            .service(web::scope("/admin").service(get_admin))
            
    })
    .workers(2)
    .bind((host, port))?
    .run()
    .await
}
