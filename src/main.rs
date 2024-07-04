use actix_web::{
  App,
  HttpServer,
};
use views::view_factory;

mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(view_factory)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}