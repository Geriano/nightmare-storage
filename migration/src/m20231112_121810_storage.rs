use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Storage::Table)
                .if_not_exists()
                .col(ColumnDef::new(Storage::Id).uuid().extra("default uuid_generate_v4()").primary_key())
                .col(ColumnDef::new(Storage::Container).string().not_null())
                .col(ColumnDef::new(Storage::Name).string().not_null())
                .col(ColumnDef::new(Storage::Mime).string().null().default(None as Option<String>))
                .col(ColumnDef::new(Storage::Extension).string().null().default(None as Option<String>))
                .col(ColumnDef::new(Storage::Path).string().not_null())
                .col(ColumnDef::new(Storage::CreatedAt).timestamp().extra("default now()").not_null())
                .col(ColumnDef::new(Storage::CreatedById).string().null().default(None as Option<String>))
                .col(ColumnDef::new(Storage::UpdatedAt).timestamp().not_null())
                .col(ColumnDef::new(Storage::UpdatedById).string().null().default(None as Option<String>))
                .col(ColumnDef::new(Storage::DeletedAt).timestamp())
                .col(ColumnDef::new(Storage::DeletedById).string().null().default(None as Option<String>))
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(Storage::Table)
                .name("idx_storages_container")
                .col(Storage::Container)
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(Storage::Table)
                .name("idx_storages_name")
                .col(Storage::Name)
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(Storage::Table)
                .name("idx_storages_mime")
                .col(Storage::Mime)
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(Storage::Table)
                .name("idx_storages_extension")
                .col(Storage::Extension)
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(Storage::Table)
                .name("idx_storages_created_at")
                .col(Storage::CreatedAt)
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .table(Storage::Table)
                .name("idx_storages_deleted_at")
                .col(Storage::DeletedAt)
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Storage::Id).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Storage {
    #[sea_orm(iden = "storages")]
    Table,
    Id,
    Container,
    Name,
    Mime,
    Extension,
    Path,
    CreatedAt,
    CreatedById,
    UpdatedAt,
    UpdatedById,
    DeletedAt,
    DeletedById,
}
