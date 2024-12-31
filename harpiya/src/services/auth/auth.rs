use harpiya_model::{
    auth_token,
    user::{Column, Entity, Model},
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    database::not_null,
    utils::{Jwt, JwtPayload},
};

pub struct Business<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> Business<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Model, String> {
        let user = Entity::find()
            .filter(Column::Username.eq(username))
            .one(self.db)
            .await
            .map_err(|msg| msg.to_string())?;

        match user {
            Some(user) => Ok(user),
            None => Err("user not found (username)".to_string()),
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Model, String> {
        let user = Entity::find()
            .filter(Column::Email.eq(email))
            .one(self.db)
            .await
            .map_err(|msg| msg.to_string())?;

        match user {
            Some(user) => Ok(user),
            None => Err("user not found (email)".to_string()),
        }
    }

    pub async fn create_new_token_pair(
        &self,
        user_id: String,
        username: &str,
    ) -> Result<String, String> {
        let payload = JwtPayload {
            user_id: user_id.to_string(),
            username: username.to_string(),
        };

        let new_access_token = Jwt::new_access_token(payload.clone())?;
        let new_refresh_token = Jwt::new_refresh_token(payload)?;

        let new_token_model = auth_token::ActiveModel {
            access_token: not_null(new_access_token.0.clone()),
            refresh_token: not_null(new_refresh_token),
            name: not_null(user_id),
            expire_time: not_null(new_access_token.1.timestamp()),
            ..Default::default()
        };

        new_token_model
            .insert(self.db)
            .await
            .map_err(|err| err.to_string())?;

        Ok(new_access_token.0)
    }

    pub async fn create_new_access_token(
        &self,
        user_id: &str,
        username: &str,
    ) -> Result<String, String> {
        let payload = JwtPayload {
            user_id: user_id.to_string(),
            username: username.to_string(),
        };



        let new_access_token = Jwt::new_access_token(payload)?;



        Ok(new_access_token.0)
    }

    async fn get_token_model_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<auth_token::Model, String> {
        let token_record = harpiya_model::auth_token::Entity::find()
            .filter(auth_token::Column::Name.eq(user_id))
            .one(self.db)
            .await
            .map_err(|msg| msg.to_string())?;

        match token_record {
            Some(token) => Ok(token),
            None => Err("token not found".to_string()),
        }
    }
}
