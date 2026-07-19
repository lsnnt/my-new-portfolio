pub mod libs;
mod models;
pub mod state;

use crate::libs::lcapi::lcapi;
use crate::libs::rating::rating;
use crate::models::githubstructs::Repo;
use crate::state::AppState;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use askama::Template;
use libs::getpinnedrepo;
use std::sync::Arc;
use tokio::sync::RwLock;

type SharedState = Arc<RwLock<AppState>>;

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")]
struct IndexTemplate {
    repos: Vec<Repo>,
    rating: u16,
    max_rating: u16,
    leetcode_problems: u16,
}

#[get("/")]
async fn hello(
    state: web::Data<SharedState>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let dta = state.read().await;
    let hello = IndexTemplate {
        repos: dta.repos.clone(),
        rating : dta.rating.clone(),
        max_rating: dta.max_rating.clone(),
        leetcode_problems: dta.leetcode_problems.clone(),
    };
    let body = hello.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

async fn update_loop(state: SharedState) -> Result<(), Box<dyn std::error::Error>> {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(20 * 60));

    loop {
        interval.tick().await;
        let repos = getpinnedrepo::get_pinned_repo().await?;

        let (rating, max_rating) = rating().await?;

        let leetcode_problems = lcapi().await?;

        let mut data = state.write().await;

        data.repos = repos;
        data.rating = rating;
        data.max_rating = max_rating;
        data.leetcode_problems = leetcode_problems;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ok(())
    let state = Arc::new(RwLock::new(AppState {
        repos: vec![],
        rating: 0,
        max_rating: 0,
        leetcode_problems: 0,
    }));

    {
        let state_clone = state.clone();
        tokio::spawn(async move {
            update_loop(state_clone).await.expect("TODO: panic message");
        });
    }
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
