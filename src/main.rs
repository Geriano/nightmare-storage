use std::{env, usize::MAX};

use actix_web::{HttpServer, App, web::{PayloadConfig, JsonConfig, FormConfig, self, Data}};
use nightmare_common::{database::{DB, self}, log};

#[macro_use] extern crate actix_web;

mod api;
mod controllers;
mod dao;
mod models;
mod requests;
mod responses;
mod services;

#[cfg(test)]
mod test;

#[actix::main]
async fn main() -> Result<(), std::io::Error> {
    let db = DB {
        auth: database::connect(env::var("AUTH_URL").unwrap()).await,
        main: database::connect(env::var("DATABASE_URL").unwrap()).await,
    };

    let auth = db.auth.ping().await;
    let main = db.main.ping().await;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(PayloadConfig::new(MAX))
            .app_data(JsonConfig::default().limit(MAX))
            .app_data(FormConfig::default().limit(MAX))
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(db.main.clone()))
            .service(api::service())
            .service(
                web::scope("/api/v1")
                    .service(controllers::storage::paginate)
                    .service(controllers::storage::store)
                    .service(controllers::storage::signed)
                    .service(controllers::storage::show)
                    .service(controllers::storage::delete)
            )
    })
        .workers(4)
        .bind(("0.0.0.0", 8000))?
        .run();

    log::info!(main, "Server started at http://localhost:8080",);
    log::info!(main, "Auth database status {:?}", auth);
    log::info!(main, "Auth database status {:?}", main);

    server.await
}
