use actix_web::HttpRequest;
use fluent::{ FluentArgs };

use crate::state::GlobalState;
use crate::templates::{ PageParametersMain, TemplateFirstRun };
use crate::helpers_actix::HttpRequestHelper;
// Probably dynamically generated by a macro
// use crate::yarte::Template;
use crate::helpers_templating::PageRenderer;

use crate::translate;

pub struct ViewFirstRun;

impl ViewFirstRun {
	pub fn render(global_state: &GlobalState, req: &HttpRequest) -> String {
		let template = TemplateFirstRun {
			// str_firstrun_welcome: &global_state.tr.translate_simple(
			//	&req.get_req_lang(),
			//	"str_firstrun_welcome"
			// ).unwrap(),
			str_firstrun_welcome:
				translate!(global_state, req, "str_firstrun_welcome"),
			str_firstrun_header_info:
				translate!(global_state, req, "str_firstrun_header_info"),
			str_firstrun_button_begin:
				translate!(global_state, req, "str_firstrun_button_begin"),
			str_label_username:
				translate!(global_state, req, "str_label_username"),
			str_label_password:
				translate!(global_state, req, "str_label_password"),
			str_label_password_repeat:
				translate!(global_state, req, "str_label_password_repeat"),
			str_firstrun_header_admindetails:
				translate!(global_state, req, "str_firstrun_header_admindetails"),
			str_label_secret:
				translate!(global_state, req, "str_label_secret"),
			str_secret_explanation:
				translate!(global_state, req, "str_secret_explanation")
		};
		
		PageRenderer::render_main(
			&global_state, &req,
			&PageParametersMain { title_code: "title_firstrun" },
			&template
		)
	}
}
