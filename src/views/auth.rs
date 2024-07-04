use actix_web::web::{get, ServiceConfig};
use login::login;
use logout::logout;

use super::path::Path;

mod login;
mod logout;

pub fn auth_factory(service: &mut ServiceConfig) {
  let base = Path {base: String::from("/auth")};
  let (login_path, logout_path) = ("/login", "/logout");
  
  service.route(
    &base.define(&String::from(login_path)), 
    get().to(login)
  ).route(
    &base.define(&String::from(logout_path)), 
    get().to(logout)
  );
}