use {
    actix_web::{middleware::Logger, web, web::Data, App, HttpResponse, HttpServer, Responder},
    bb8::Pool,
    bb8_postgres::PostgresConnectionManager,
    clap::{Arg, Command},
    tokio_postgres::{Config, NoTls},
};

mod endpoints;
mod error;
mod structs;
mod utils;

use crate::structs::context::Context;

pub async fn handler_404() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found: The requested URL was not found on the server.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let command = Command::new("candles-server")
        .arg(Arg::new("markets_json_path").required(true))
        .arg(Arg::new("user").required(true))
        .arg(Arg::new("password").required(true))
        .arg(Arg::new("host").required(true))
        .arg(Arg::new("port").required(true))
        .arg(Arg::new("dbname").required(true));

    let matches = command.get_matches();

    let markets_path = matches.value_of("markets_json_path").unwrap();

    let mut config = Config::new();

    config.user(matches.value_of("user").unwrap());
    config.password(matches.value_of("password").unwrap());
    config.host(matches.value_of("host").unwrap());
    config.port(matches.value_of("port").unwrap().parse::<u16>().unwrap());
    config.dbname(matches.value_of("dbname").unwrap());

    let manager = PostgresConnectionManager::new(config, NoTls);
    let pool = Pool::builder().max_size(15).build(manager).await.unwrap();

    let context = Data::new(Context {
        markets: utils::markets::load_markets(markets_path),
        pool,
    });

    println!("Starting server");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(context.clone())
            .service(endpoints::tradingview::service())
            .default_service(web::get().to(handler_404))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
