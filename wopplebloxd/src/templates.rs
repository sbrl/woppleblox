use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct MainTemplate<'a> {
    pub title: &'a str,
    pub content: &'a str
}
