use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use handler::general_handlers::not_found;
use router::{general_routes, blog_routes, static_routes};
use state::AppState;

#[path ="../template.rs"]
mod template;

#[path ="../router.rs"]
mod router;

#[path ="../handler.rs"]
mod handler;

#[path = "../state.rs"]
mod state;

#[path = "../generator.rs"]
mod generator;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    generator::generate()?;
    let shared_data = web::Data::new(AppState {
        health_check_response: "I am OK".to_string(),
        vis_count: Mutex::new(0),
    });
    let factory = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(blog_routes)
            .configure(static_routes)
            .default_service(
                web::route().to(not_found)
            )
    };
    HttpServer::new(factory).bind("127.0.0.1:7878")?.run().await
}
