pub mod libs;
mod models;
pub mod state;

use std::path::Path;

use crate::libs::atcoder::atcoder;
use crate::libs::blogstore::load_posts;
use crate::libs::lcapi::lcapi;
use crate::libs::rating::rating;
use crate::libs::timelinestore::load_timeline;
use crate::models::blogpost::BlogPost;
use crate::models::githubstructs::Repo;
use crate::models::timeline::TimelineEntry;
use crate::state::AppState;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use askama::Template;
use std::sync::Arc;
use actix_web::middleware::Compress;
use tokio::sync::RwLock;
use tokio::try_join;
use crate::libs::getpinnedrepo::get_pinned_repo;

type SharedState = Arc<RwLock<AppState>>;

/// How many recent posts to surface on the homepage; the rest live behind /blog.
const HOME_POSTS: usize = 6;

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    repos: &'a Vec<Repo>,
    rating: &'a u16,
    max_rating: &'a u16,
    leetcode_problems: &'a u16,
    atcoder_rating: &'a u16,
    atcoder_max_rating: &'a u16,
    posts: &'a [BlogPost],
    timeline: &'a Vec<TimelineEntry>,
}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogIndexTemplate<'a> {
    posts: &'a Vec<BlogPost>,
}

#[derive(Template)]
#[template(path = "blog_post.html")]
struct BlogPostTemplate<'a> {
    post: &'a BlogPost,
}

async fn hello(
    state: web::Data<SharedState>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let dta = state.read().await;
    let latest = &dta.posts[..dta.posts.len().min(HOME_POSTS)];
    let hello = IndexTemplate {
        repos: &dta.repos,
        rating : &dta.rating,
        max_rating: &dta.max_rating,
        leetcode_problems: &dta.leetcode_problems,
        atcoder_rating: &dta.atcoder_rating,
        atcoder_max_rating: &dta.atcoder_max_rating,
        posts: latest,
        timeline: &dta.timeline,
    };
    let body = hello.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

async fn blog_index(
    state: web::Data<SharedState>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let dta = state.read().await;
    let page = BlogIndexTemplate { posts: &dta.posts };
    let body = page.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

async fn blog_post(
    state: web::Data<SharedState>,
    path: web::Path<String>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let slug = path.into_inner();
    let dta = state.read().await;
    match dta.posts.iter().find(|post| post.slug == slug) {
        Some(post) => {
            let page = BlogPostTemplate { post };
            let body = page.render()?;
            Ok(HttpResponse::Ok().content_type("text/html").body(body))
        }
        None => Ok(HttpResponse::NotFound().content_type("text/html").body(format!(
            "<!DOCTYPE html><html><head><title>404</title></head>\
             <body style=\"background:#000;color:#e4e4e4;font-family:monospace;padding:2rem\">\
             <h1 style=\"color:#00ff41\">404 — post not found</h1>\
             <p>$ cat ~/blog/{slug}.md: No such file or directory</p>\
             <p><a style=\"color:#00ff41\" href=\"/blog\">&lt;- cd ~/blog</a></p>\
             </body></html>"
        ))),
    }
}

async fn update_loop(state: SharedState) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(10*60 ));
    let client = reqwest::Client::new();
    loop {
        interval.tick().await;
        println!("Updating...");

        match try_join!(
        get_pinned_repo(&client),
        rating(&client),
        lcapi(&client),
    ) {
            Ok((repos, (rating, max_rating), lc)) => {
                println!(
                    "repos={}, rating={}, max={}, lc={}",
                    repos.len(),
                    rating,
                    max_rating,
                    lc,
                );
                let mut data = state.write().await;
                data.repos = repos;
                data.rating = rating;
                data.max_rating = max_rating;
                data.leetcode_problems = lc;
            }
            Err(e) => {
                eprintln!("update failed: {e}");
            }
        }

        // AtCoder is scraped from HTML, so keep it separate: a breakage there
        // must not stop the GitHub/Codeforces/LeetCode updates above.
        match atcoder(&client).await {
            Ok((atcoder_rating, atcoder_max_rating)) => {
                println!("atcoder: rating={atcoder_rating}, max={atcoder_max_rating}");
                let mut data = state.write().await;
                data.atcoder_rating = atcoder_rating;
                data.atcoder_max_rating = atcoder_max_rating;
            }
            Err(e) => {
                eprintln!("atcoder update failed: {e}");
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ok(())
    let blog_dir = std::env::var("BLOG_DIR").unwrap_or_else(|_| "content/blog".to_string());
    let posts = load_posts(Path::new(&blog_dir));
    println!("blog: loaded {} post(s) from {blog_dir}", posts.len());

    let timeline_dir = std::env::var("TIMELINE_DIR").unwrap_or_else(|_| "content/timeline".to_string());
    let timeline = load_timeline(Path::new(&timeline_dir));
    println!("timeline: loaded {} entr(y/ies) from {timeline_dir}", timeline.len());

    let state = Arc::new(RwLock::new(AppState {
        repos: vec![],
        rating: 0,
        max_rating: 0,
        leetcode_problems: 0,
        atcoder_rating: 0,
        atcoder_max_rating: 0,
        posts,
        timeline,
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
            .route("/blog", web::get().to(blog_index))
            .route("/blog/{slug}", web::get().to(blog_post))
            .default_service(web::to(hello))
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
