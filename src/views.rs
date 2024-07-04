use actix_web::web::ServiceConfig;

mod auth;
pub mod path;

pub fn view_factory(service: &mut ServiceConfig) {
  auth::auth_factory(service);
}