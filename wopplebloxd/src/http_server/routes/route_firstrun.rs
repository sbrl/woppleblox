use actix_web::web;
use actix_web::{ HttpRequest, Responder };

use crate::global_state::GlobalState;
// use crate::templates::TemplateFirstRun;
use crate::views::ViewFirstRun;

pub struct RouteFirstRun;

impl RouteFirstRun {
    pub async fn main(req: HttpRequest, state : web::Data<GlobalState>) -> impl Responder {
        ViewFirstRun::render(state.get_ref(), &req)
    }
}
