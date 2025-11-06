use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use crate::domain::models::user::{self, ActiveModel, Model};

#[derive(Clone)]
pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, username: String) -> Result<Model, sea_orm::DbErr> {
        let new_user = ActiveModel {
            username: Set(username),
            ..Default::default()
        };

        new_user.insert(&self.db).await
    }

    pub async fn get_all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        user::Entity::find().all(&self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, sea_orm::DbErr> {
        user::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<usize, sea_orm::DbErr> {
        if let Some(user_model) = self.find_by_id(id).await? {
            let active: ActiveModel = user_model.into();
            let res = active.delete(&self.db).await?;
            Ok(res.rows_affected.try_into().unwrap())
        } else {
            Ok(0)
        }
    }
}

