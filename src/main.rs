pub mod libs;
mod models;
pub mod state;

use libs::getpinnedrepo;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use crate::libs::lcapi::lcapi;
use crate::libs::rating::rating;
use crate::models::githubstructs::Repo;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::state::AppState;

type SharedState = Arc<RwLock<AppState>>;

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")]
struct IndexTemplate {
    repos : Vec<Repo>,
    rating : u16,
    max_rating: u16,
    leetcode_problems : u16,
}

#[get("/")]
async fn hello() -> Result<impl Responder, Box<dyn std::error::Error>> {
    let (rating, max_rating) = rating().await?;
    let hello = IndexTemplate { repos : getpinnedrepo::get_pinned_repo().await? ,
        rating, max_rating,
        leetcode_problems : lcapi().await?,
    };
    let body = hello.render()?;
    Ok(
        HttpResponse::Ok().content_type("text/html").body(body)
    )
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ok(())
    HttpServer::new(move || {
        App::new()
            .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

