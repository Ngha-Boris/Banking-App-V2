use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use supabase_rs::SupabaseClient;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Bank App is running!")
}

async fn init_supabase() -> SupabaseClient {
    dotenv().ok();
    let url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");
    println!("Connecting to Supabase at: {}", url);
    SupabaseClient::new(url, key)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let supabase_client = init_supabase().await;

    match supabase_client.select("users").execute().await {
        Ok(response) => println!("Supabase response: {:?}", response),
        Err(e) => println!("Supabase error: {:?}", e),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(supabase_client.clone()))
            .service(health_check)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
