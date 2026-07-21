use crate::models::githubstructs::Repo;
use crate::models::hblogs::Blogs;

pub struct AppState {
    pub repos: Vec<Repo>,
    pub rating: u16,
    pub max_rating: u16,
    pub leetcode_problems: u16,
    pub blogs: Vec<Blogs>,
}

