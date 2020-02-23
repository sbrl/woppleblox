use yarte::Template;

#[derive(Template)]
#[template(path = "index.html")]
// #[template(path = "index.html", mode =" html-min")]
pub struct MainTemplate<'a> {
    pub title: &'a str,
    pub content: &'a str
}
