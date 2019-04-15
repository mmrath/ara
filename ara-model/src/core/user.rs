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
pub struct NewUserRecord<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub email: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub roles: Option<Vec<i32>>,
}

impl<'a, 'b: 'a> From<&'b NewUser<'a>> for NewUserRecord<'a> {
    fn from(n: &'b NewUser<'a>) -> Self {
        NewUserRecord {
            first_name: n.first_name,
            last_name: n.last_name,
            username: n.email,
            email: n.email,
        }
    }
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
    pub fn insert(conn: &Connection, new_user: &NewUserRecord<'_>) -> Result<User, Error> {
        let res = insert_into(app_user::table)
            .values(new_user)
            .get_result::<UserRecord>(conn)?;
        Ok(res.into())
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

    pub fn find_by_id(conn: &Connection, id: i64) -> Result<UserRecord, Error> {
        debug!("Finding user by id {}", id);
        let res = app_user::table.find(id).first(conn)?;
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
        use crate::schema::app_user;

        let res = app_user::table
            .filter(app_user::username.eq(username))
            .select(app_user::all_columns)
            .first::<UserRecord>(conn)
            .optional()?;
        Ok(res)
    }
}
