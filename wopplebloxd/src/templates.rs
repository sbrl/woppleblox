use yarte::Template;

#[derive(Template)]
#[template(path = "index.html")]
// #[template(path = "index.html", mode ="html-min")]
pub struct Main<'a> {
    pub title: &'a str,
    pub content: &'a str
}


#[derive(Template)]
#[template(path = "post.html")]
pub struct Post<'a> {
    pub username: &'a str,
    pub content: &'a str,
    pub logo_url: &'a str
}

#[derive(Template)]
#[template(path = "user-profile.html")]
pub struct UserProfile<'a> {
    pub username: &'a str,
    pub posts: &'a str,
    pub logo_url: &'a str
}
