use serde::{Deserialize, Serialize};
use validator::Validate;

use failure::{ensure, Error};

use diesel::insert_into;
use diesel::prelude::*;

use crate::core::user::UserRecord;
use crate::db::Connection;
use crate::schema::user_credential;
use chrono::{DateTime, Utc};

#[derive(
    Queryable,
    Insertable,
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
#[table_name = "user_credential"]
#[serde(rename_all = "camelCase")]
pub struct UserCredential {
    pub id: i64,
    pub password_hash: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub invalid_attempts: i32,
    pub locked: bool,
    pub activation_key: Option<String>,
    pub activation_key_expires_at: Option<DateTime<Utc>>,
    pub activated: bool,
    pub reset_key: Option<String>,
    pub reset_key_expires_at: Option<DateTime<Utc>>,
    pub reset_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
}

impl UserCredential {
    pub fn find_by_id(conn: &Connection, user_id: i64) -> Result<Option<UserCredential>, Error> {
        let res = user_credential::table
            .filter(user_credential::id.eq(user_id))
            .select(user_credential::all_columns)
            .first::<UserCredential>(conn)
            .optional()?;
        Ok(res)
    }

    pub fn create(&self, conn: &Connection) -> Result<(), Error> {
        let res = insert_into(user_credential::table)
            .values(self)
            .execute(conn)?;
        ensure!(res == 1, "Insert to user_credential failed");
        Ok(())
    }

    pub fn find_by_activation_key(
        conn: &Connection,
        activation_key: &str,
    ) -> Result<Option<(UserRecord, UserCredential)>, Error> {
        use crate::schema::app_user;
        debug!("Finding by activation key {}", activation_key);
        let res = user_credential::table
            .filter(user_credential::activation_key.eq(activation_key))
            .inner_join(app_user::table)
            .select((app_user::all_columns, user_credential::all_columns))
            .first::<(UserRecord, UserCredential)>(conn)
            .optional()?;
        Ok(res)
    }

    pub fn find_by_reset_key(
        conn: &Connection,
        reset_key: &str,
    ) -> Result<Option<(UserRecord, UserCredential)>, Error> {
        use crate::schema::app_user;
        debug!("Finding user with reset key {}", reset_key);
        let res = user_credential::table
            .filter(user_credential::reset_key.eq(reset_key))
            .inner_join(app_user::table)
            .select((app_user::all_columns, user_credential::all_columns))
            .first::<(UserRecord, UserCredential)>(conn)
            .optional()?;
        Ok(res)
    }

    pub fn update_reset_key(
        conn: &Connection,
        user_id: i64,
        reset_key: &str,
        reset_expiry: DateTime<Utc>,
    ) -> Result<(), Error> {
        let target = user_credential::table.filter(user_credential::id.eq(user_id));
        let res = diesel::update(target)
            .set((
                user_credential::reset_key.eq(reset_key),
                user_credential::reset_key_expires_at.eq(reset_expiry),
            ))
            .execute(conn)?;
        ensure!(res == 1, "Update failed");
        Ok(())
    }

    pub fn set_activated(conn: &Connection, user_id: i64) -> Result<(), Error> {
        let target = user_credential::table.filter(user_credential::id.eq(user_id));
        let res = diesel::update(target)
            .set((
                user_credential::activation_key.eq(None as Option<String>),
                user_credential::activation_key_expires_at.eq(None as Option<DateTime<Utc>>),
                user_credential::activated.eq(true),
            ))
            .execute(conn)?;
        ensure!(res == 1, "Update failed");
        Ok(())
    }
}
