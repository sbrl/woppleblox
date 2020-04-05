use yarte::Template;

#[derive(Template)]
#[template(path = "index.html")]
// #[template(path = "index.html", mode ="html-min")]
pub struct Main<'a> {
	pub title: &'a str,
	pub content: &'a str
}

/**
 * HTML fragment for a single post
 */
#[derive(Template)]
#[template(path = "post.html")]
pub struct FragmentPost<'a> {
	pub username: &'a str,
	pub content: &'a str,
	pub logo_url: &'a str
}

/**
 * The user profile page content
 */
#[derive(Template)]
#[template(path = "user-profile.html")]
pub struct UserProfile<'a> {
	pub username: &'a str,
	pub posts: &'a str,
	pub logo_url: &'a str
}

/**
 * The first run page content
 */
#[derive(Template)]
#[template(path = "firstrun.html")]
pub struct FirstRun<'a> {
	pub str_firstrun_welcome: &'a str,
	pub str_firstrun_header_info: &'a str,
	pub str_firstrun_button_begin: &'a str,
	pub str_label_username: &'a str,
	pub str_label_password: &'a str,
	pub str_label_password_repeat: &'a str,
	pub str_firstrun_header_admindetails: &'a str,
	pub str_label_secret: &'a str,
	pub str_secret_explanation: &'a str
}

/**
 * The login page content
 */
#[derive(Template)]
#[template(path = "login.html")]
pub struct Login<'a> {
	pub str_label_username: &'a str,
	pub str_label_password: &'a str,
	pub str_button_login: &'a str
}
