use chrono::Utc;
use nightmare_common::log;
use sea_orm::{DatabaseConnection, EntityTrait, DbErr, QueryFilter, ColumnTrait, Set, QueryTrait, ConnectionTrait};
use uuid::Uuid;

use crate::models::storages;

pub async fn store(
    db: &DatabaseConnection,
    storages: Vec<storages::Model>,
) -> Result<(), DbErr> {
    let query = storages::Entity::insert_many(
        storages.iter()
            .map(|storage| storages::ActiveModel::from(storage.clone()))
            .collect::<Vec<storages::ActiveModel>>()
    );

    log::debug!(store, "{}", query.build(db.get_database_backend()).to_string());

    query.exec(db).await?;

    Ok(())
}

pub async fn delete(
    db: &DatabaseConnection,
    container: String,
    id: Vec<Uuid>,
) -> Result<(), DbErr> {
    let models = storages::Entity::find()
        .filter(storages::Column::Id.is_in(id))
        .filter(storages::Column::Container.eq(container))
        .all(db)
        .await?;

    let models = models.iter()
        .map(|model| {
            let mut model = storages::ActiveModel::from(model.clone());

            model.deleted_at = Set(Some(Utc::now().naive_local()));
            model
        })
        .collect::<Vec<storages::ActiveModel>>();
        
    for model in models {
        let query = storages::Entity::update(model);

        log::debug!(delete, "{}", query.build(db.get_database_backend()).to_string());

        query.exec(db).await?;
    }

    Ok(())
}
