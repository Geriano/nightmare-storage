use actix_web::Scope;
use actix_web::web;
use actix_web::web::redirect;
use nightmare_common::api::{Authentication, Common};
use nightmare_common::request::pagination::Request as PaginationRequest;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::controllers;
use crate::requests;
use crate::requests::storage::Order;

#[derive(OpenApi)]
#[openapi(
    modifiers(&Authentication, &Common),
    info(
        title = "Storage",
        description = "Storage Service",
        contact(
            name = "Geriano",
            email = "gerznewbie@gmail.com",
            url = "geriano.github.io",
        ),
    ),
    tags(
        (name = "Storage"),
    ),
    paths(
        controllers::storage::paginate,
        controllers::storage::store,
        controllers::storage::signed,
        controllers::storage::show,
        controllers::storage::delete,
    ),
    components(
        schemas(requests::storage::Upload),
        schemas(requests::storage::Delete),
        schemas(PaginationRequest<Order>),
    ),
)]
pub struct Doc;

pub fn route() -> SwaggerUi {
    SwaggerUi::new("/{_:.*}")
        .urls(vec![
            (Url::new("Storage", "/doc/api.json"), Doc::openapi()),
        ])
}

pub async fn json() -> Result<String, serde_json::error::Error> {
    Doc::openapi().to_json()
}

pub fn service() -> Scope {
    web::scope("/doc")
        .route("/api.json", web::to(json))
        .service(redirect("", "/doc/index.html"))
        .service(route())
}
