use anyhow::{Error, Result};
use diesel::{prelude::*, result::Error as DieselError, serialize::ToSql};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::schema::{
    role_permission as RolePermissionSchema,
    role_permission::dsl::role_permission as role_permissions,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Action {
    Admin,
    Read,
    List,
    Write,
    Delete,
}

impl Action {
    // add a string representation of the action
    pub fn as_string(&self) -> String {
        match self {
            Action::Admin => "admin".to_string(),
            Action::Read => "read".to_string(),
            Action::List => "list".to_string(),
            Action::Write => "write".to_string(),
            Action::Delete => "delete".to_string(),
        }
    }

    // convert a string to an action
    pub fn from_string(action: &str) -> Result<Action> {
        match action {
            "admin" => Ok(Action::Admin),
            "read" => Ok(Action::Read),
            "list" => Ok(Action::List),
            "write" => Ok(Action::Write),
            "delete" => Ok(Action::Delete),
            _ => Err(Error::msg("Invalid action")),
        }
    }
}

impl ToSql<diesel::sql_types::Text, diesel::pg::Pg> for Action {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        todo!()
    }
}

// Meant to only get data from the db
#[derive(Queryable, Serialize, Deserialize, Debug, Selectable)]
#[diesel(table_name = RolePermissionSchema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RolePermission {
    pub id: i32,
    pub role_id: i32,
    pub directory_id: i32,
    pub name: String,
    pub actions: Vec<Option<String>>,
}

#[derive(Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = RolePermissionSchema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ModifiableRolePermission {
    pub name: String,
    pub actions: Vec<Action>,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = RolePermissionSchema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableRolePermission {
    pub role_id: i32,
    pub directory_id: i32,
    pub name: String,
    pub actions: Vec<Action>,
}

pub fn create(role_permission: InsertableRolePermission) -> Result<InsertableRolePermission> {
    if role_permission.name.is_empty() || role_permission.actions.is_empty() {
        return Err(Error::msg("Invalid role_permission"));
    }

    if role_permission.role_id < 1 {
        return Err(Error::msg("Invalid id"));
    }

    Ok(role_permission)
}

// Repo
pub async fn db_create(
    new_role_permission: InsertableRolePermission,
    connection: &mut AsyncPgConnection,
) -> () {
    let result: Result<i32, DieselError> = diesel::insert_into(role_permissions)
        .values(&new_role_permission)
        .returning(RolePermissionSchema::id)
        .get_result(connection)
        .await;
}
