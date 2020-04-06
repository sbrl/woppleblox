use actix_web::HttpRequest;
use fluent::fluent_args;

use crate::global_state::GlobalState;
use crate::yarte::Template;
use crate::templates::{ PageParametersMain, TemplateMain };
use crate::helpers_actix::HttpRequestHelper;

use crate::intl::Translations;

pub struct PageRenderer;

impl PageRenderer {
    pub fn render_main<T: Template>(global_state: &GlobalState, req: &HttpRequest, params: &PageParametersMain, template: &T) -> String {
        let title = global_state.tr.translate(
            &req.get_req_lang(),
            "template_main_title",
            fluent_args![
                "title_page" => global_state.tr.translate_simple(&req.get_req_lang(), params.title_code).unwrap()
            ]
        ).unwrap();
        let main = TemplateMain {
            title: &title,
            content: &template.render()
        };
        
        main.render()
    }
}

/**
 * Helper for yarte templates.
 * This trait (and it's associated impl) is abit special, because it basically extends
 * the existing yarte Template trait to add new functionality.
 * It appears to be the C# equivalent of a generic.
 */
pub trait TemplateHelper: Template {
    fn render(&self) -> String;
}

impl<T: Template> TemplateHelper for T {
    fn render(&self) -> String {
        self.call().expect(&format!("Error: Failed to render template {}", self))
    }
}
