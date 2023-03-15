use actix_web::{get,web,HttpServer,App};
use sqlx::{MySqlPool};
use dotenv::dotenv;
use std::process::exit;

mod meta_data;
use meta_data::services;

struct AppState {
   pool: MySqlPool,
}

#[get("/")]
async fn index() -> String {
   format!("This is promising")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   dotenv().ok();
   let database_uri: String = std::env::var("DATABASE_URI").unwrap();
   let pool = MySqlPool::connect(&database_uri).await;

   let app_data = web::Data::new(AppState {pool: match pool {
      Ok(val) => val,
      Err(_) => exit(1)
   }});
   HttpServer::new(move || {
      App::new()
         .app_data(app_data.clone())
         .service(index)
         .configure(services::config)
   })
      .bind(("127.0.0.1",6942))?
      .run()
      .await
   // extractor::example_main();
}
