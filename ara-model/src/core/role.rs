use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use failure::{ensure, Error, ResultExt};

use crate::core::permission::{
    Permission
};
use crate::db::Connection;
use crate::schema::{role, role_permission, user_role, permission};
use diesel::expression::exists::exists;
use diesel::select;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewRole<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoleUpdate {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub version: i32,
    pub permissions: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[table_name = "role"]
#[serde(rename_all = "camelCase")]
pub(crate) struct RoleRecord {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: DateTime<Utc>,
    pub updated_by: String,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "role"]
#[serde(rename_all = "camelCase")]
pub(crate) struct NewRoleRecord<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Identifiable, Queryable, Associations)]
#[table_name = "role_permission"]
#[belongs_to(RoleRecord, foreign_key = "role_id")]
#[belongs_to(Permission, foreign_key = "permission_id")]
#[primary_key(role_id, permission_id)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RolePermissionRecord {
    pub(crate) role_id: i32,
    pub(crate) permission_id: i32,
}

impl From<(RoleRecord, Vec<Permission>)> for Role {
    fn from(role: (RoleRecord, Vec<Permission>)) -> Self {
        Role {
            id: role.0.id,
            name: role.0.name,
            description: role.0.description,
            created_at: role.0.created_at,
            updated_at: role.0.updated_at,
            version: role.0.version,
            permissions: role.1,
        }
    }
}


impl Role {
    pub fn find(conn: &Connection, role_id: i32) -> Result<Role, Error> {
        let role_record = role::table
            .find(role_id)
            .get_result::<RoleRecord>(conn)
            .context("Failed to find role")?;

        let permissions = RolePermissionRecord::belonging_to(&role_record)
            .inner_join(permission::table)
            .select(permission::all_columns)
            .load::<Permission>(conn)?;

        Ok((role_record, permissions).into())
    }

    pub fn create(conn: &Connection, new_role: &NewRole<'_>) -> Result<Role, Error> {
        let new_role_row = NewRoleRecord {
            name: new_role.name,
            description: new_role.description,
            created_by: "system".to_owned(),
            updated_by: "system".to_owned(),
        };

        let role_record: RoleRecord = insert_into(role::table)
            .values(&new_role_row)
            .get_result(conn)
            .context("Failed to insert user into DB")?;

        let role_perms = new_role
            .permissions
            .iter()
            .map(|p| RolePermissionRecord {
                role_id: role_record.id,
                permission_id: p.id,
            })
            .collect::<Vec<_>>();

        insert_into(role_permission::table)
            .values(role_perms.as_slice())
            .execute(conn)?;

        let permissions = RolePermissionRecord::belonging_to(&role_record)
            .inner_join(permission::table)
            .select(permission::all_columns)
            .load::<Permission>(conn)?;

        Ok((role_record, permissions).into())
    }

    pub fn update(conn: &Connection, role_update: &RoleUpdate) -> Result<Role, Error> {
        delete(role_permission::table.filter(role_permission::role_id.eq(role_update.id)))
            .execute(conn)?;

        let role_perms = role_update
            .permissions
            .iter()
            .map(|p| RolePermissionRecord {
                role_id: role_update.id,
                permission_id: p.to_owned(),
            })
            .collect::<Vec<_>>();

        insert_into(role_permission::table)
            .values(role_perms.as_slice())
            .execute(conn)?;

        let role_record = update(
            role::table.filter(
                role::id
                    .eq(role_update.id)
                    .and(role::version.eq(role_update.version)),
            ),
        )
        .set(&NewRoleRecord {
            name: role_update.name.as_str(),
            description: role_update.description.as_str(),
            created_by: "system".to_owned(),
            updated_by: "system".to_owned(),
        })
        .get_result::<RoleRecord>(conn)?;

        let permissions = RolePermissionRecord::belonging_to(&role_record)
            .inner_join(permission::table)
            .select(permission::all_columns)
            .load::<Permission>(conn)?;

        Ok((role_record, permissions).into())
    }

    pub fn delete(conn: &Connection, role_id: i32) -> Result<(), Error> {
        let exists: bool = select(exists(
            user_role::table.filter(user_role::role_id.eq(role_id)),
        ))
        .get_result(conn)?;
        ensure!(!exists, "Role is currently associated with users");
        let update_count = delete(role::table.filter(role::id.eq(role_id))).execute(conn)?;
        ensure!(
            update_count == 1,
            format!("Expected to delete {} records, found {}", 1, update_count)
        );
        Ok(())
    }
}
