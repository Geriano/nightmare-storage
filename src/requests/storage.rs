use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use utoipa::{ToSchema, IntoParams, openapi::{RefOr, schema::Schema, ObjectBuilder, SchemaFormat, KnownFormat, SchemaType, Array}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(MultipartForm, IntoParams)]
pub struct Upload {
  #[multipart()]
  pub files: Vec<TempFile>,
}

impl<'a> ToSchema<'a> for Upload {
  fn schema() -> (&'a str, RefOr<Schema>) {
    let o: RefOr<Schema> = ObjectBuilder::new()
      .format(Some(
        SchemaFormat::KnownFormat(
          KnownFormat::Binary
        )
      ))
      .schema_type(SchemaType::String)
      .into();
    (
      "Upload",
      ObjectBuilder::new()
        .schema_type(SchemaType::Object)
        .property(
          "files", 
          Schema::Array(Array::new(o))
        )
        .into()
    )
  }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct Delete {
  #[schema()]
  pub storages: Vec<Uuid>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Order {
  Name,
  Mime,
  Extension,
  Path,
  CreatedAt,
}
