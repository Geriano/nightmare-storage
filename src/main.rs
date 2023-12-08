use actix_web::web;

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
    nightmare_common::app::serve(|| {
        web::scope("")
            .service(api::service())
            .service(
                web::scope("/api/v1")
                    .service(controllers::storage::paginate)
                    .service(controllers::storage::store)
                    .service(controllers::storage::signed)
                    .service(controllers::storage::show)
                    .service(controllers::storage::delete)
            )
    }).await
}
