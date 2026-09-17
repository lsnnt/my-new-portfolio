pub mod libs;
mod models;
pub mod state;

use crate::libs::lcapi::lcapi;
use crate::libs::rating::rating;
use crate::libs::mblog::getblog;
use crate::models::githubstructs::Repo;
use crate::state::AppState;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use askama::Template;
use std::sync::Arc;
use actix_web::middleware::Compress;
use tokio::sync::RwLock;
use tokio::try_join;
use crate::libs::getpinnedrepo::get_pinned_repo;
use crate::models::hblogs::Blogs;

type SharedState = Arc<RwLock<AppState>>;

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    repos: &'a Vec<Repo>,
    rating: &'a u16,
    max_rating: &'a u16,
    leetcode_problems: &'a u16,
    blogs: &'a Vec<Blogs>,
}

async fn hello(
    state: web::Data<SharedState>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let dta = state.read().await;
    let hello = IndexTemplate {
        repos: &dta.repos,
        rating : &dta.rating,
        max_rating: &dta.max_rating,
        leetcode_problems: &dta.leetcode_problems,
        blogs: &dta.blogs,
    };
    let body = hello.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}


async fn update_loop(state: SharedState) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(10*60 ));
    let client = reqwest::Client::new();
    loop {
        println!("Updating...");
        interval.tick().await;

        match try_join!(
        get_pinned_repo(&client),
        rating(&client),
        lcapi(&client),
        getblog(&client),
    ) {
            Ok((repos, (rating, max_rating), lc, blogs)) => {
                println!(
                    "repos={}, rating={}, max={}, lc={}, blogs={}",
                    repos.len(),
                    rating,
                    max_rating,
                    lc,
                    blogs.len()
                );
                let mut data = state.write().await;
                data.repos = repos;
                data.rating = rating;
                data.max_rating = max_rating;
                data.leetcode_problems = lc;
                data.blogs = blogs;
            }
            Err(e) => {
                eprintln!("update failed: {e}");
            }
        }
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
        blogs: vec![],
    }));

    {
        let state_clone = state.clone();
        tokio::spawn(async move {
            update_loop(state_clone).await;
        });
    }
    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .default_service(web::to(hello))
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
