use std::str::FromStr;

use harpiya_domain::{public::{self, Method}, ecode::ECode, user_service};
use harpiya_model::{user::{Model, Entity}, user::{ActiveModel, Column}};
use sea_orm::{DatabaseConnection, DbErr, ActiveModelTrait, EntityTrait, Set, QueryFilter, ColumnTrait, QueryOrder, PaginatorTrait};

use crate::{utils::{current_timestamp, i8_to_bool, encrypt_password, Validator}, header};

pub struct Business<'a> {
    db: &'a DatabaseConnection,
    header: public::Header
}

impl<'a> Business<'a> {
    pub fn new(db: &'a DatabaseConnection, header: public::Header) -> Self {
        Self { db, header }
    }

    fn decode(&self, dto: user_service::User) -> Model {
        let current_time = current_timestamp();
        let hashed_password = encrypt_password(&dto.password).unwrap();
        let full_name = format!(
            "{} {} {}", 
            dto.first_name,
            if dto.middle_name.is_some() { dto.middle_name.clone().unwrap() } else { "".to_owned() },
            if dto.last_name.is_some() { dto.last_name.clone().unwrap() } else { "".to_owned() }
        );
        Model { 
            name: dto.name, 
            email: dto.email,
            username: dto.username,
            password: hashed_password,
            enabled: dto.enabled as i8,
            first_name: dto.first_name,
            middle_name: dto.middle_name,
            last_name: dto.last_name,
            full_name,
            language: dto.language,
            time_zone: dto.time_zone,
            send_welcome_email: dto.send_welcome_email as i8,
            unsubscribed: dto.unsubscribed as i8,
            user_image: dto.user_image,
            gender: dto.gender,
            birth_date: dto.birth_date,
            phone: dto.phone,
            location: dto.location,
            bio: dto.bio,
            mobile_no: dto.mobile_no,
            banner_image: dto.banner_image,
            new_password: dto.new_password,
            logout_all_sessions: dto.logout_all_sessions as i8,
            reset_password_key: dto.reset_password_key,
            last_reset_password_key_generated_on: dto.last_reset_password_key_generated_on,
            last_password_reset_date: dto.last_password_reset_date,
            email_signature: dto.email_signature,
            simultaneous_sessions: dto.simultaneous_sessions as i8,
            last_ip: dto.last_ip,
            last_login: dto.last_login,
            login_after: dto.login_after,
            login_before: dto.login_before,
            user_type: dto.user_type,
            last_active: dto.last_active,
            bypass_restrict_ip_check_if_2fa_enabled: dto.bypass_restrict_ip_check_if_2fa_enabled as i8,
            last_known_versions: dto.last_known_versions,
            api_key: dto.api_key,
            api_secret: dto.api_secret,
            owner: "".to_owned(), 
            modified_by: "".to_owned(),
            creation: current_time, 
            modified: current_time,
            deleted: 0,
        }
    }

    fn encode(&self, model: Model) -> user_service::User {
        user_service::User { 
            name: model.name, 
            email: model.email,
            username: model.username,
            password: model.password,
            enabled: i8_to_bool(model.enabled),
            first_name: model.first_name,
            middle_name: model.middle_name,
            last_name: model.last_name,
            full_name: model.full_name,
            language: model.language,
            time_zone: model.time_zone,
            send_welcome_email: i8_to_bool(model.send_welcome_email),
            unsubscribed: i8_to_bool(model.unsubscribed),
            user_image: model.user_image,
            gender: model.gender,
            birth_date: model.birth_date,
            phone: model.phone,
            location: model.location,
            bio: model.bio,
            mobile_no: model.mobile_no,
            banner_image: model.banner_image,
            new_password: model.new_password,
            logout_all_sessions: i8_to_bool(model.logout_all_sessions),
            reset_password_key: model.reset_password_key,
            last_reset_password_key_generated_on: model.last_reset_password_key_generated_on,
            last_password_reset_date: model.last_password_reset_date,
            email_signature: model.email_signature,
            simultaneous_sessions: i8_to_bool(model.simultaneous_sessions),
            last_ip: model.last_ip,
            last_login: model.last_login,
            login_after: model.login_after,
            login_before: model.login_before,
            user_type: model.user_type,
            last_active: model.last_active,
            bypass_restrict_ip_check_if_2fa_enabled: i8_to_bool(model.bypass_restrict_ip_check_if_2fa_enabled),
            last_known_versions: model.last_known_versions,
            api_key: model.api_key,
            api_secret: model.api_secret,
            owner: "".to_owned(),
            modified_by: "".to_owned(), 
            creation: model.creation, 
            modified: model.modified,
            deleted: model.deleted,
        }
    }

    pub async fn list(
        &self,
        filter: Option<user_service::UserFilter>,
        mut sorts: Vec<public::Sort>,
        pager: Option<public::Pager>
    ) -> user_service::ListUserReply {
        let mut query = Entity::find();

        if let Some(filter) = filter {
            query = match filter.deleted() {
                public::BooleanScope::BoolAll => query,
                public::BooleanScope::BoolFalse => query.filter(Column::Deleted.eq(0)),
                public::BooleanScope::BoolTrue => query.filter(Column::Deleted.gt(0)),
            };
            if !filter.names.is_empty() {
                query = query.filter(Column::Name.is_in(filter.names));
            }
            if !filter.owners.is_empty() {
                query = query.filter(Column::Owner.is_in(filter.owners));
            }
            if !filter.modifiers.is_empty() {
                query = query.filter(Column::ModifiedBy.is_in(filter.modifiers));
            }
            if let Some(range) = filter.create_time_range {
                query = query.filter(Column::Creation.between(range.left, range.right))
            }
            if let Some(range) = filter.update_time_range {
                query = query.filter(Column::Modified.between(range.left, range.right))
            }
            if let Some(range) = filter.delete_time_range {
                query = query.filter(Column::Deleted.between(range.left, range.right))
            }
        }

        if sorts.is_empty() {
            sorts.push(public::Sort { 
                field: "id".to_owned(), 
                direction: public::SortDirection::SortAsc.into() 
            });
        }
        for sort in sorts {
            if let Ok(col) = Column::from_str(sort.field.as_str()) {
                query = match sort.direction() {
                    public::SortDirection::SortAsc => query.order_by_asc(col),
                    public::SortDirection::SortDesc => query.order_by_desc(col),
                }
            }
        }
        let mut pager = pager.unwrap_or(public::Pager { disabled: true, ..Default::default() });

        let mut dtos = vec![];

        let header = if pager.disabled {
            match query.all(self.db).await {
                Ok(models) => {
                    for model in models {
                        dtos.push(self.encode(model));
                    };
                    header::reply(self.header.trace_id.clone())
                }
                Err(e) => header::err_reply(
                    self.header.trace_id.clone(),
                    ECode::PublicServiceErrorListLabel,
                    format!("List SSH Key failed: {}", e)
                )
            }
        } else {
            let paginator = query.paginate(self.db, pager.size);
            pager.count = paginator.num_items().await.unwrap_or_default();
            for model in paginator.fetch_page(pager.index - 1).await.unwrap_or_default() {
                dtos.push(self.encode(model))
            };
            header::reply(self.header.trace_id.clone())
        };

        user_service::ListUserReply { header, pager: Some(pager), data: dtos }
    }

    pub async fn operate(&self, method: Method, data: Option<user_service::User>) -> user_service::OperateUserReply {
        let (header, data) = match data {
            None => {
                (header::err_reply(
                    self.header.trace_id.clone(), 
                    ECode::PublicServiceErrorOperateLabel,
                "Missing method data".to_string()), None)
            }
            Some(data) => {
                let result = match method {
                    Method::Create => self.insert(data).await,
                    Method::Update => self.update(data).await,
                    Method::Delete => self.delete(data.name).await,
                    Method::Upsert => Ok(None),
                };
                match result {
                    Ok(data) => {
                        let header = header::reply(self.header.trace_id.clone());
                        (header, data)
                    }
                    Err(e) => {
                        let header = header::err_reply(
                            self.header.trace_id.clone(),
                            ECode::PublicServiceErrorOperateLabel,
                            format!("{} access failed: {}", method.as_str_name(), e)
                        );
                        (header, None)
                    }
                }
            }
        };
        user_service::OperateUserReply { header, data }
    }

    pub async fn batch_operate(&self, method: Method, data: Vec<user_service::User>) -> user_service::BatchOperateUserReply {
        let header = if data.is_empty() {
            header::err_reply(
                self.header.trace_id.clone(), 
                ECode::PublicServiceErrorOperateLabel,
                "Missing method data".to_string()
            )
        } else {
            let result = match method {
                Method::Upsert => Ok(None),
                Method::Delete => self.batch_delete(data).await,
                _ => Ok(None)
            };
            match result {
                Ok(_) => header::reply(self.header.trace_id.clone()),
                Err(e) => header::err_reply(
                    self.header.trace_id.clone(),
                    ECode::PublicServiceErrorBatchOperateLabel, 
                    format!("Batch {} access failed: {}", method.as_str_name(), e))
            }
        };
        user_service::BatchOperateUserReply { header }
    }

    async fn insert(&self, dto: user_service::User) -> Result<Option<user_service::User>, DbErr> {
        if dto.name.is_empty() {
            Err(DbErr::Custom("Name is required".to_owned()))
        } else if Validator::username(&dto.username).is_err() {
            Err(DbErr::Custom("Username is required".to_owned()))
        } else if Validator::password(&dto.password).is_err() {
            Err(DbErr::Custom("Password is required".to_owned()))
        } else if Validator::email(&dto.email).is_err() {
            Err(DbErr::Custom("Email is required".to_owned()))
        } else if dto.first_name.is_empty() {
            Err(DbErr::Custom("First name is required".to_owned()))
        } else {
            let active: ActiveModel = self.decode(dto).into();
            let model = active.insert(self.db).await?;
            Ok(Some(self.encode(model)))
        }
    }

    async fn update(&self, dto: user_service::User) -> Result<Option<user_service::User>, DbErr> {
        let value: Option<Model> = Entity::find_by_id(&dto.name).one(self.db).await?;
        let mut active: ActiveModel = value.unwrap().into();
        active.name = Set(dto.name);
        //active.modified_by = Set(Some(self.header.operator.clone()));
        active.modified = Set(current_timestamp());
        let model = active.update(self.db).await?;
        Ok(Some(self.encode(model)))
    }

    async fn delete(&self, name: String) -> Result<Option<user_service::User>, DbErr> {
        Entity::delete_by_id(&name).exec(self.db).await?;
        Ok(None)
    }

    async fn batch_delete(&self, dtos: Vec<user_service::User>) -> Result<Option<user_service::User>, DbErr> {
        let mut names = vec![];
        for dto in dtos {
            names.push(dto.name);
        }
        Entity::delete_many()
            .filter(Column::Name.is_in(names))
            .exec(self.db)
            .await?;
        Ok(None)
    }
}