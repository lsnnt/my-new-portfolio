use crate::models::blogpost::BlogPost;
use crate::models::githubstructs::Repo;
use crate::models::timeline::TimelineEntry;

pub struct AppState {
    pub repos: Vec<Repo>,
    pub rating: u16,
    pub max_rating: u16,
    pub leetcode_problems: u16,
    pub atcoder_rating: u16,
    pub atcoder_max_rating: u16,
    pub posts: Vec<BlogPost>,
    pub timeline: Vec<TimelineEntry>,
}
