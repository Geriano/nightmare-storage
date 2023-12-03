use actix_multipart::form::MultipartForm;
use actix_web::{web::{Data, Path, Json}, Responder};
use nightmare_common::{request::pagination::PaginationRequest, middleware::auth::Auth, response::{pagination::Pagination, http::{Unauthorized, InternalServerError, NotFound}}};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{services, requests::storage::{Order, Upload, Delete}, models::storages};

#[utoipa::path(
    tag = "Storage",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        Pagination<storages::Model>,
        Unauthorized,
        InternalServerError,
    ),
)]
#[get("/storage/{container}")]
pub async fn paginate(
    db: Data<DatabaseConnection>,
    _: Auth,
    request: PaginationRequest<Order>,
    container: Path<String>,
) -> impl Responder {
    services::storage::paginate(&db, request, container.to_owned()).await
}

#[utoipa::path(
    tag = "Storage",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        (status = 200, description = "Ok"),
        NotFound,
        Unauthorized,
        InternalServerError,
    ),
)]
#[put("/storage/{container}/{id}")]
pub async fn signed(
    path: Path<(String, Uuid)>,
) -> impl Responder {
    services::storage::signed(path.1).await
}

#[utoipa::path(
    tag = "Storage",
    context_path = "/api/v1",
    security(("token" = [])),
    responses(
        (status = 200, description = "Ok", content_type = "application/octet-stream"),
        NotFound,
        Unauthorized,
        InternalServerError,
    ),
)]
#[get("/storage/{container}/{id}")]
pub async fn show(
    db: Data<DatabaseConnection>,
    path: Path<(String, String)>,
) -> impl Responder {
    services::storage::show(&db, path.1.to_owned()).await
}

#[utoipa::path(
    tag = "Storage",
    context_path = "/api/v1",
    security(("token" = [])),
    request_body = Upload,
    responses(
        NotFound,
        Unauthorized,
        InternalServerError,
    ),
)]
#[post("/storage/{container}")]
pub async fn store(
    db: Data<DatabaseConnection>,
    auth: Auth,
    container: Path<String>,
    request: MultipartForm<Upload>,
) -> impl Responder {
    services::storage::store(&db, auth, container.to_owned(), request).await
}

#[utoipa::path(
    tag = "Storage",
    context_path = "/api/v1",
    security(("token" = [])),
    request_body = Delete,
    responses(
        NotFound,
        Unauthorized,
        InternalServerError,
    ),
)]
#[delete("/storage/{container}")]
pub async fn delete(
    db: Data<DatabaseConnection>,
    auth: Auth,
    container: Path<String>,
    request: Json<Delete>,
) -> impl Responder {
    services::storage::delete(&db, auth, container.to_owned(), request.into_inner()).await
}
