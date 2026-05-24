use sea_orm::ColumnTrait;
use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::{QueryFilter, Select};

use crate::api::handlers::task::TaskFilter; // Ajuste conforme seu módulo de entidades

pub trait TaskFilterExt {
    /// Filtra apenas tarefas que não foram deletadas (Soft Delete)
    fn filter_active(self) -> Self;

    /// Filtra por prioridade se o valor existir
    fn filter_by_priority(self, priority: Option<String>) -> Self;

    /// Filtra por usuário se o ID existir
    fn filter_by_user(self, user_id: Option<i32>) -> Self;

    /// Filtra por título (busca parcial)
    fn filter_by_title(self, title: Option<String>) -> Self;

    fn filter_by_descritpion(self, description: Option<String>) -> Self;

    /// Filtra por campo padrão
    fn filter_by_default(self, is_default: Option<bool>) -> Self;

    fn filter_by_fields(self, filter: TaskFilter) -> Self;
}

impl TaskFilterExt for Select<Entity> {
    fn filter_active(self) -> Self {
        self.filter(Column::DeletedAt.is_null())
    }

    fn filter_by_priority(mut self, priority: Option<String>) -> Self {
        if let Some(p) = priority {
            self = self.filter(Column::Priority.eq(p));
        }
        self
    }

    fn filter_by_user(mut self, user_id: Option<i32>) -> Self {
        if let Some(uid) = user_id {
            self = self.filter(Column::UserId.eq(uid));
        }
        self
    }

    fn filter_by_title(mut self, title: Option<String>) -> Self {
        if let Some(t) = title {
            self = self.filter(Column::Title.contains(&t));
        }
        self
    }
    fn filter_by_default(mut self, is_default: Option<bool>) -> Self {
        if let Some(d) = is_default {
            self = self.filter(Column::IsDefault.eq(d));
        }
        self
    }

    fn filter_by_fields(self, filter: TaskFilter) -> Self {
        self.filter_active()
            .filter_by_priority(filter.priority)
            .filter_by_user(filter.user_id)
            .filter_by_title(filter.title)
            .filter_by_default(filter.is_default)
            .filter_by_descritpion(filter.description)
    }

    fn filter_by_descritpion(mut self, description: Option<String>) -> Self {
        if let Some(d) = description {
            self = self.filter(Column::Description.contains(&d));
        }
        self
    }
}
