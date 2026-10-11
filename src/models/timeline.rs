#[derive(Clone)]
pub struct TimelineEntry {
    pub date: String,
    pub head: String,
    pub title: String,
    pub body: String,
    /// Optional live metric rendered next to the body:
    /// "" | "codeforces" | "leetcode" | "atcoder".
    pub metric: String,
}
