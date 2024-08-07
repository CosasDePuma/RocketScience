use sea_orm::{ActiveValue, entity::prelude::*};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "services")]
pub struct Model {
    #[sea_orm(unique)] // FIXME: This should be primary key but sea-orm doesn't support composite primary keys
    pub id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub product: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub version: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub banner: String,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub first_seen: DateTime,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub last_seen: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: ActiveValue::Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}