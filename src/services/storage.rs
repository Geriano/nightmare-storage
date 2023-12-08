use std::{collections::HashMap, fs::{self, create_dir_all}, path::Path};

use actix_files::NamedFile;
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, http::header::ContentDisposition, Responder, ResponseError};
use nightmare_common::{middleware::auth::Auth, log, request::pagination::PaginationRequest, time, response::http, base58};
use reqwest::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Condition, PaginatorTrait, QuerySelect, QueryTrait, ConnectionTrait};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{requests::storage::{Upload, Delete, Order}, models::storages, dao};

pub async fn paginate(
    db: &DatabaseConnection,
    request: PaginationRequest<Order>,
    container: String,
) -> HttpResponse {
    let mut query = storages::Entity::find()
        .filter(storages::Column::DeletedAt.is_null())
        .filter(storages::Column::Container.eq(container));

    if request.search.is_some() {
        query = query.filter(
            Condition::any()
                .add(storages::Column::Name.like(request.search()))
                .add(storages::Column::Mime.like(request.search()))
                .add(storages::Column::Extension.like(request.search()))
                .add(storages::Column::Path.like(request.search()))
        )
    }

    let total = query.clone().count(db).await.unwrap();
    let query = query.limit(Some(request.limit()))
        .offset(Some((request.page() - 1) * request.limit()));

    log::debug!(paginate, "{}", query.build(db.get_database_backend()).to_string());
    
    match query.all(db).await {
        Err(e) => {
            log::error!(paginate, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(storages) => {
            HttpResponse::Ok().json(json!({
                "total": {
                    "data": total,
                    "page": total / request.limit(),
                },
                "data": storages,
            }))
        },
    }
}

pub async fn signed(id: Uuid) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "id": base58::to_string(id),
    }))
}

pub async fn show(
    db: &DatabaseConnection,
    id: String,
) -> Result<impl Responder, impl ResponseError> {
    let id = base58::decode(id);

    if let Err(e) = id {
        return Err(http::Error {
            code: StatusCode::BAD_REQUEST,
            message: e.to_string(),
        })
    }

    let id = Uuid::from_slice(&id.unwrap());

    if let Err(e) = id {
        return Err(http::Error {
            code: StatusCode::BAD_REQUEST,
            message: e.to_string(),
        })
    }

    match storages::Entity::find_by_id(id.unwrap()).one(db).await {
        Err(e) => {
            log::error!(controllers::storages::show, "{}", e);

            Err(http::Error {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: e.to_string(),
            })
        },
        Ok(storage) => match storage {
            None => Err(http::Error { 
                code: StatusCode::NOT_FOUND,
                message: "file not found".to_string(),
            }),
            Some(storage) => {
                match NamedFile::open(storage.path.clone()) {
                    Err(e) => {
                        log::error!(show, "{}", e);

                        Err(http::Error {
                            code: StatusCode::INTERNAL_SERVER_ERROR,
                            message: e.to_string(),
                        })
                    },
                    Ok(file) => {
                        Ok(file.set_content_disposition(
                            ContentDisposition::attachment(storage.name)
                        ))
                    },
                }
            },
        }
    }
}

pub async fn store(
    db: &DatabaseConnection,
    auth: Auth,
    container: String,
    MultipartForm(form): MultipartForm<Upload>,
) -> HttpResponse {
    let mut validation = HashMap::new();

    if form.files.is_empty() {
        validation.insert("files", vec!["files field is required".to_string()]);
    }

    for (i, file) in form.files.iter().enumerate() {
        if file.file_name.is_none() {
            validation.insert("file", vec![
                format!("files[{i}] name field is required")
            ]);
        }
    }

    if !validation.is_empty() {
        return HttpResponse::UnprocessableEntity().json(json!({
            "errors": validation,
        }))
    }
    
    let path = format!("./uploads/{container}");
    let path = Path::new(&path);
    let user = auth.user.id.clone();
    let now = time::now();
    let mut storages = vec![];

    create_dir_all(path).ok();
    
    for request in &form.files {
        let id = Uuid::new_v4();
        let name = request.file_name.clone().unwrap();
        let mime = request.content_type.clone().and_then(|mime| Some(mime.to_string()));
        let extension = name.split(".").last().and_then(|ext| Some(ext.to_string()));
        let path = format!("{}/{}", path.to_str().unwrap(), id);

        fs::copy(request.file.path(), &path).unwrap();

        storages.push(storages::Model {
            id, name, mime, extension, path,
            container: container.clone(),
            created_at: now.clone(),
            created_by_id: Some(user.clone().to_string()),
            updated_at: now.clone(),
            updated_by_id: Some(user.clone().to_string()),
            deleted_at: None,
            deleted_by_id: None,
        });
    }

    match dao::storage::store(db, storages.clone()).await {
        Err(e) => {
            log::error!(store, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(_) => {
            let storages = storages.iter()
                .map(|storage| json!({
                    "id": base58::to_string(storage.id.clone()),
                    "name": storage.name.clone(),
                    "mime": storage.mime.clone(),
                    "extension": storage.extension.clone(),
                    "path": storage.path.clone(),
                }))
                .collect::<Vec<Value>>();

            HttpResponse::Created().json(json!({
                "storages": storages,
            }))
        },
    }
}

pub async fn delete(
    db: &DatabaseConnection,
    _auth: Auth,
    container: String,
    delete: Delete,
) -> HttpResponse {
    match dao::storage::delete(db, container, delete.storages.clone()).await {
        Err(e) => {
            log::error!(store, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        },
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
    }
}
