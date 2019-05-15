use failure::{ensure, Error};
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use validator::{Validate, ValidationError};

use crate::core::role::RoleRecord;
use crate::db::Connection;
use crate::error::DbError;
use crate::schema::{app_user, user_credential, user_role};
use chrono::{DateTime, Utc};

#[derive(
    Queryable,
    Identifiable,
    Associations,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Eq,
    PartialEq,
    Validate,
)]
#[table_name = "app_user"]
#[serde(rename_all = "camelCase")]
pub struct UserRecord {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    #[validate(length(min = "2"))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub phone_number: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: DateTime<Utc>,
    pub updated_by: String,
    pub version: i32,
}

#[derive(
    Queryable,
    Identifiable,
    AsChangeset,
    Associations,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Eq,
    PartialEq,
    Validate,
)]
#[table_name = "app_user"]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateRecord<'a> {
    pub(crate) id: i64,
    pub(crate) first_name: Option<&'a str>,
    pub(crate) last_name: Option<&'a str>,
    #[validate(length(min = "2"))]
    pub(crate) username: Option<&'a str>,
    #[validate(email)]
    pub(crate) email: Option<&'a str>,
    pub(crate) phone_number: Option<Option<&'a str>>,
    pub(crate) active: Option<bool>,
    pub(crate) updated_at: DateTime<Utc>,
    pub(crate) updated_by: &'a str,
    pub(crate) version: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserUpdate<'a> {
    pub id: i64,
    pub first_name: ColumnValue<&'a str>,
    pub last_name: ColumnValue<&'a str>,
    pub username: ColumnValue<&'a str>,
    pub email: ColumnValue<&'a str>,
    pub phone_number: ColumnValue<Option<&'a str>>,
    pub active: ColumnValue<bool>,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub active: bool,
    pub version: i32,
}

impl From<UserRecord> for User {
    fn from(user_record: UserRecord) -> Self {
        User {
            id: user_record.id,
            username: user_record.username,
            first_name: user_record.first_name,
            last_name: user_record.last_name,
            email: user_record.email,
            phone_number: user_record.phone_number,
            active: user_record.active,
            version: user_record.version,
        }
    }
}

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "app_user"]
#[serde(rename_all = "camelCase")]
pub(crate) struct UserNewRecord<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub phone_number: Option<&'a str>,
    pub created_by: &'a str,
    pub updated_by: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub phone_number: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserSignUp {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations, Insertable)]
#[belongs_to(UserRecord, foreign_key = "user_id")]
#[belongs_to(RoleRecord, foreign_key = "role_id")]
#[table_name = "user_role"]
#[primary_key(user_id, role_id)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleRecord {
    pub user_id: i64,
    pub role_id: i32,
}

impl User {
    pub fn insert(
        conn: &Connection,
        new_user: WithAudit<'_, NewUser<'_>>,
    ) -> Result<UserRecord, Error> {
        let res = insert_into(app_user::table)
            .values(UserNewRecord::from(new_user))
            .get_result::<UserRecord>(conn)?;
        Ok(res)
    }

    pub fn update<'a>(
        conn: &Connection,
        update_user: impl Into<UserUpdateRecord<'a>>,
    ) -> Result<UserRecord, Error> {
        use crate::schema::app_user::columns::version;
        let update_record: UserUpdateRecord<'a> = update_user.into();
        let target = app_user::table.filter(
            app_user::id
                .eq(update_record.id)
                .and(version.eq(update_record.version)),
        );
        let res = update(target)
            .set(update_record)
            .get_result::<UserRecord>(conn)?;
        Ok(res)
    }

    pub fn activate(conn: &Connection, user_id: i64) -> Result<(), Error> {
        use crate::schema::app_user::dsl::*;
        update(app_user)
            .filter(id.eq(user_id))
            .set(active.eq(true))
            .execute(conn)?;

        Ok(())
    }

    pub fn reset_failed_login_count(conn: &Connection, user_id: i64) -> Result<(), Error> {
        use crate::schema::user_credential::dsl::*;

        update(user_credential)
            .filter(id.eq(user_id))
            .set(invalid_attempts.eq(0))
            .execute(conn)?;
        Ok(())
    }

    pub fn update_password_hash(
        conn: &Connection,
        user_id: i64,
        new_hash: &str,
    ) -> Result<(), Error> {
        use crate::schema::user_credential::dsl::*;

        update(user_credential)
            .filter(id.eq(user_id))
            .set(password_hash.eq(new_hash))
            .execute(conn)?;
        Ok(())
    }

    pub fn increment_failed_login_count(conn: &Connection, user_id: i64) -> Result<(), Error> {
        use crate::schema::user_credential::dsl::*;

        update(user_credential)
            .filter(id.eq(user_id))
            .set(invalid_attempts.eq(invalid_attempts + 1))
            .execute(conn)?;
        Ok(())
    }

    pub fn set_password(conn: &Connection, user_id: i64, hash: &str) -> Result<(), Error> {
        let target = user_credential::table.filter(user_credential::id.eq(user_id));
        let res = diesel::update(target)
            .set(user_credential::password_hash.eq(hash))
            .execute(conn)?;

        ensure!(res == 1, DbError::IncorrectResultSize(1, res));
        Ok(())
    }

    pub fn delete(conn: &Connection, user_id: i64) -> Result<(), Error> {
        delete(app_user::table)
            .filter(app_user::id.eq(user_id).and(app_user::active.eq(false))) //Only unactivated accounts can be deleted
            .execute(conn)?;
        Ok(())
    }

    pub fn find_by_id(conn: &Connection, id: i64) -> Result<Option<UserRecord>, Error> {
        debug!("Finding user by id {}", id);
        let res = app_user::table.find(id).first(conn).optional()?;
        Ok(res)
    }

    pub fn validate_username(conn: &Connection, username_i: &str) -> Result<(), Error> {
        use crate::schema::app_user::dsl::*;
        use diesel::expression::dsl::exists;
        use diesel::select;

        let exists: bool =
            select(exists(app_user.filter(username.eq(username_i)))).get_result(conn)?;

        ensure!(!exists, ValidationError::new("Email already exists in DB"));

        Ok(())
    }

    pub fn exists_by_email(conn: &Connection, email_id: &str) -> Result<bool, Error> {
        use crate::schema::app_user::dsl::*;
        use diesel::expression::dsl::exists;
        use diesel::select;

        let result =
            select(exists(app_user.filter(email.eq(email_id)))).get_result::<bool>(conn)?;
        Ok(result)
    }

    #[allow(dead_code)]
    pub fn exists_by_username(conn: &Connection, uname: &str) -> Result<bool, Error> {
        use crate::schema::app_user::dsl::*;
        use diesel::expression::dsl::exists;
        use diesel::select;

        let result =
            select(exists(app_user.filter(username.eq(uname)))).get_result::<bool>(conn)?;
        Ok(result)
    }

    pub fn find_by_username(
        conn: &Connection,
        username: &str,
    ) -> Result<Option<UserRecord>, Error> {
        let res = app_user::table
            .filter(app_user::username.eq(username))
            .select(app_user::all_columns)
            .first::<UserRecord>(conn)
            .optional()?;
        Ok(res)
    }
}

pub struct WithAudit<'a, T> {
    audit_user: &'a str,
    record: &'a T,
}

impl<'a, T> WithAudit<'a, T> {
    pub fn new(audit_user: &'a str, record: &'a T) -> WithAudit<'a, T> {
        Self { audit_user, record }
    }
}

impl<'a, 'b: 'a> From<WithAudit<'a, NewUser<'a>>> for UserNewRecord<'a> {
    fn from(n: WithAudit<'a, NewUser<'a>>) -> Self {
        UserNewRecord {
            first_name: n.record.first_name,
            last_name: n.record.last_name,
            username: n.record.email,
            email: n.record.email,
            phone_number: n.record.phone_number,
            created_by: n.audit_user,
            updated_by: n.audit_user,
        }
    }
}

pub trait AuditExt: Sized {
    fn with_audit<'a>(&'a self, username: &'a str) -> WithAudit<'a, Self>;
}

impl<T> AuditExt for T where T: Sized {
    fn with_audit<'a>(&'a self, username: &'a str) -> WithAudit<'a, T> {
        WithAudit::new(username, self)
    }
}


impl<'a, T> From<WithAudit<'a, T>> for UserUpdateRecord<'a>
where
    UserUpdate<'a>: From<&'a T>,
{
    fn from(n: WithAudit<'a, T>) -> Self {
        let update: UserUpdate<'a> = n.record.into();
        UserUpdateRecord {
            id: update.id,
            first_name: update.first_name.into_option(),
            last_name: update.last_name.into_option(),
            username: update.username.into_option(),
            email: update.email.into_option(),
            phone_number: update.phone_number.into_option(),
            active: update.active.into_option(),
            updated_by: n.audit_user.as_ref(),
            updated_at: Utc::now(),
            version: update.version,
        }
    }
}

trait Auditable {
    type CreatedBy;
    type CreatedAt;
    type UpdatedBy;
    type UpdatedAt;

    fn created_by() -> Self::CreatedBy;
    fn created_at() -> Self::CreatedAt;
    fn updated_by() -> Self::UpdatedBy;
    fn updated_at() -> Self::UpdatedAt;
}

impl<'a> Auditable for app_user::table {
    type CreatedBy = app_user::created_by;
    type CreatedAt = app_user::created_at;
    type UpdatedBy = app_user::updated_by;
    type UpdatedAt = app_user::updated_at;

    fn created_by() -> Self::CreatedBy {
        app_user::columns::created_by
    }

    fn created_at() -> Self::CreatedAt {
        app_user::columns::created_at
    }

    fn updated_by() -> Self::UpdatedBy {
        app_user::columns::updated_by
    }

    fn updated_at() -> Self::UpdatedAt {
        app_user::columns::updated_at
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum ColumnValue<T> {
    Changed(T),
    Unchanged,
}

impl<T> Default for ColumnValue<T> {
    #[inline]
    fn default() -> ColumnValue<T> {
        ColumnValue::Unchanged
    }
}

impl<T> From<T> for ColumnValue<T> {
    fn from(val: T) -> ColumnValue<T> {
        ColumnValue::Changed(val)
    }
}

impl<T> ColumnValue<T> {
    fn into_option(self) -> Option<T> {
        match self {
            ColumnValue::Changed(value) => Some(value),
            ColumnValue::Unchanged => None,
        }
    }
}

impl<T> Into<Option<T>> for ColumnValue<T> {
    fn into(self) -> Option<T> {
        match self {
            ColumnValue::Changed(value) => Some(value),
            ColumnValue::Unchanged => None,
        }
    }
}

impl<'a> From<&'a User> for UserUpdate<'a> {
    fn from(user: &'a User) -> Self {
        UserUpdate {
            id: user.id,
            first_name: ColumnValue::Changed(user.first_name.as_ref()),
            last_name: ColumnValue::Changed(user.last_name.as_ref()),
            username: ColumnValue::Changed(user.username.as_str()),
            email: ColumnValue::Changed(user.email.as_str()),
            phone_number: ColumnValue::Changed(user.phone_number.as_ref().map(String::as_ref)),
            active: ColumnValue::Changed(user.active),
            version: user.version,
        }
    }
}
