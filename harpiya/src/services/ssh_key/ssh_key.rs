use std::str::FromStr;

use harpiya_domain::{public::{self, Method}, ssh_key_service, ecode::ECode};
use harpiya_model::{ssh_key::{Model, Entity}, ssh_key::{ActiveModel, Column}};
use sea_orm::{DatabaseConnection, DbErr, ActiveModelTrait, EntityTrait, Set, QueryFilter, ColumnTrait, QueryOrder, PaginatorTrait};

use crate::{utils::current_timestamp, header};

pub struct Business<'a> {
    db: &'a DatabaseConnection,
    header: public::Header
}

impl<'a> Business<'a> {
    pub fn new(db: &'a DatabaseConnection, header: public::Header) -> Self {
        Self { db, header }
    }

    fn decode(&self, dto: ssh_key_service::SshKey) -> Model {
        let current_time = current_timestamp();
        Model { 
            name: dto.name, 
            public_key: dto.public_key, 
            private_key: dto.private_key, 
            owner: "".to_owned(), 
            modified_by: "".to_owned(),
            creation: current_time, 
            modified: current_time,
            deleted: 0
        }
    }

    fn encode(&self, model: Model) -> ssh_key_service::SshKey {
        ssh_key_service::SshKey { 
            name: model.name, 
            public_key: model.public_key, 
            private_key: model.private_key, 
            owner: "".to_owned(),
            modified_by: "".to_owned(), 
            creation: model.creation, 
            modified: model.modified,
            deleted: model.deleted,
        }
    }

    pub async fn list(
        &self,
        filter: Option<ssh_key_service::SshKeyFilter>,
        mut sorts: Vec<public::Sort>,
        pager: Option<public::Pager>
    ) -> ssh_key_service::ListSshKeyReply {
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

        ssh_key_service::ListSshKeyReply { header, pager: Some(pager), data: dtos }
    }

    pub async fn operate(&self, method: Method, data: Option<ssh_key_service::SshKey>) -> ssh_key_service::OperateSshKeyReply {
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
        ssh_key_service::OperateSshKeyReply { header, data }
    }

    pub async fn batch_operate(&self, method: Method, data: Vec<ssh_key_service::SshKey>) -> ssh_key_service::BatchOperateSshKeyReply {
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
        ssh_key_service::BatchOperateSshKeyReply { header }
    }

    async fn insert(&self, dto: ssh_key_service::SshKey) -> Result<Option<ssh_key_service::SshKey>, DbErr> {
        if dto.name.is_empty() {
            Err(DbErr::Custom("Name is required".to_owned()))
        } else if dto.private_key.is_empty() {
            Err(DbErr::Custom("Private key required".to_owned()))
        } else if dto.public_key.is_empty() {
            Err(DbErr::Custom("Public key required".to_owned()))
        } else {
            let active: ActiveModel = self.decode(dto).into();
            let model = active.insert(self.db).await?;
            Ok(Some(self.encode(model)))
        }
    }

    async fn update(&self, dto: ssh_key_service::SshKey) -> Result<Option<ssh_key_service::SshKey>, DbErr> {
        let value: Option<Model> = Entity::find_by_id(&dto.name).one(self.db).await?;
        let mut active: ActiveModel = value.unwrap().into();
        active.name = Set(dto.name);
        //active.modified_by = Set(Some(self.header.operator.clone()));
        active.modified = Set(current_timestamp());
        let model = active.update(self.db).await?;
        Ok(Some(self.encode(model)))
    }

    async fn delete(&self, name: String) -> Result<Option<ssh_key_service::SshKey>, DbErr> {
        Entity::delete_by_id(&name).exec(self.db).await?;
        Ok(None)
    }

    async fn batch_delete(&self, dtos: Vec<ssh_key_service::SshKey>) -> Result<Option<ssh_key_service::SshKey>, DbErr> {
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