use yarte::Template;

/**
 * Holds the parameters for the main template.
 * The main template is special because it's applied by the page renderer and it wraps
 * an existing template's content.
 */
pub struct PageParametersMain<'a> {
	/**
	 * The translation code for the title of this page.
	 * Note that this translation code should only contain the string for the page
	 * itself - the end bit is appended automatically.
	 */
	pub title_code: &'a str
}

#[derive(Template)]
#[template(path = "index.html")]
// #[template(path = "index.html", mode ="html-min")]
pub struct TemplateMain<'a> {
	pub title: &'a str,
	pub content: &'a str
}

/**
 * HTML fragment for a single post
 */
#[derive(Template)]
#[template(path = "post.html")]
pub struct TemplateFragmentPost<'a> {
	pub username: &'a str,
	pub content: &'a str,
	pub logo_url: &'a str
}

/**
 * The user profile page content
 */
#[derive(Template)]
#[template(path = "user-profile.html")]
pub struct TemplateUserProfile<'a> {
	pub username: &'a str,
	pub posts: &'a str,
	pub logo_url: &'a str
}

/**
 * The first run page content
 */
#[derive(Template)]
#[template(path = "firstrun.html")]
pub struct TemplateFirstRun<'a> {
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
pub struct TemplateLogin<'a> {
	pub str_label_username: &'a str,
	pub str_label_password: &'a str,
	pub str_button_login: &'a str
}
