use std::io::Write;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use diesel::deserialize;
use diesel::insert_into;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::serialize::{self, Output};
use diesel::sql_types::Varchar;
use diesel::types::{FromSql, IsNull, ToSql};

use crate::db::Connection;
use crate::schema::*;
use failure::{bail, Error};

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

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
)]
#[table_name = "notification"]
pub struct NotificationRecord {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub subject: String,
    pub notification_type: NotificationType,
    pub from_address: Option<String>,
    pub body_html: Option<String>,
    pub body_plain_text: Option<String>,
}

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "notification"]
pub struct NewNotificationRecord<'a> {
    pub notification_type: NotificationType,
    pub from_address: Option<&'a str>,
    pub subject: &'a str,
    pub body_html: Option<&'a str>,
    pub body_plain_text: Option<&'a str>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Eq,
    PartialEq,
    AsExpression,
    FromSqlRow,
    EnumString,
    Display,
)]
#[sql_type = "Varchar"]
pub enum NotificationType {
    Email,
}

impl ToSql<Varchar, Pg> for NotificationType {
    fn to_sql<W: Write>(&self, out: &mut Output<'_, W, Pg>) -> serialize::Result {
        let str_res = format!("{}", self);
        out.write_all(str_res.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Varchar, Pg> for NotificationType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = String::from_utf8_lossy(not_none!(bytes));
        let parse_res = NotificationType::from_str(bytes.as_ref());
        parse_res.map_err(|_e| "Unrecognized enum variant".into())
    }
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
)]
#[table_name = "notification_recipient"]
#[belongs_to(NotificationRecord, foreign_key = "notification_id")]
pub struct NotificationRecipientRecord {
    pub id: i64,
    pub notification_id: i64,
    pub recipient_type: RecipientType,
    pub name: Option<String>,
    pub address: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "notification_recipient"]
pub struct NewNotificationRecipientRecord<'a> {
    pub notification_id: i64,
    pub recipient_type: RecipientType,
    pub name: Option<&'a str>,
    pub address: &'a str,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Eq,
    PartialEq,
    AsExpression,
    FromSqlRow,
    EnumString,
    Display,
)]
#[sql_type = "Varchar"]
pub enum RecipientType {
    To,
    Cc,
    Bcc,
}

impl ToSql<Varchar, Pg> for RecipientType {
    fn to_sql<W: Write>(&self, out: &mut Output<'_, W, Pg>) -> serialize::Result {
        let str_res = format!("{}", self);
        out.write_all(str_res.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Varchar, Pg> for RecipientType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = String::from_utf8_lossy(not_none!(bytes));
        let parse_res = RecipientType::from_str(bytes.as_ref());
        parse_res.map_err(|_e| "Unrecognized enum variant".into())
    }
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
)]
#[table_name = "notification_attachment"]
#[belongs_to(NotificationRecord, foreign_key = "notification_id")]
pub struct NotificationAttachmentRecord {
    pub id: i64,
    pub notification_id: i64,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "notification_attachment"]
pub struct NewNotificationAttachmentRecord {
    pub id: i64,
    pub notification_id: i64,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewNotification {
    pub notification_type: NotificationType,
    pub from: Option<String>,
    pub subject: String,
    pub body: Body,
    pub to: Vec<Address>,
    pub cc: Vec<Address>,
    pub bcc: Vec<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// The given name for this address
    pub name: Option<String>,
    /// The mailbox address
    pub address: String,
}

impl<'a> From<&'a str> for Address {
    fn from(mailbox: &'a str) -> Address {
        Address {
            name: None,
            address: mailbox.into(),
        }
    }
}

impl From<String> for Address {
    fn from(mailbox: String) -> Address {
        Address {
            name: None,
            address: mailbox,
        }
    }
}

impl From<NotificationRecipientRecord> for Address {
    fn from(nr: NotificationRecipientRecord) -> Self {
        Self {
            name: nr.name,
            address: nr.address,
        }
    }
}

impl<S: Into<String>, T: Into<String>> From<(S, T)> for Address {
    fn from(header: (S, T)) -> Address {
        let (address, alias) = header;
        Address {
            name: Some(alias.into()),
            address: address.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Body {
    Html(String),
    PlainText(String),
    HtmlAndPlainText(String, String),
}

impl Body {
    pub fn html(&self) -> Option<&str> {
        match self {
            Body::Html(s) => Some(s.as_str()),
            Body::HtmlAndPlainText(h, _) => Some(h.as_str()),
            _ => None,
        }
    }

    pub fn plain_text(&self) -> Option<&str> {
        match self {
            Body::PlainText(s) => Some(s.as_str()),
            Body::HtmlAndPlainText(_, p) => Some(p.as_str()),
            _ => None,
        }
    }
}

impl TryFrom<(Option<String>, Option<String>)> for Body {
    type Error = failure::Error;

    fn try_from((html, plaintext): (Option<String>, Option<String>)) -> Result<Self, Self::Error> {
        match (html, plaintext) {
            (Some(h), Some(p)) => Ok(Body::HtmlAndPlainText(h, p)),
            (Some(h), None) => Ok(Body::Html(h)),
            (None, Some(p)) => Ok(Body::PlainText(p)),
            (_, _) => bail!("At least html or plain text body must be specified"),
        }
    }
}

pub fn create_notification(conn: &Connection, notification: &NewNotification) -> Result<(), Error> {
    let new_notf_rec = NewNotificationRecord {
        notification_type: notification.notification_type,
        from_address: notification.from.deref(),
        subject: notification.subject.as_ref(),
        body_html: notification.body.html(),
        body_plain_text: notification.body.plain_text(),
    };

    let res = insert_into(notification::table)
        .values(new_notf_rec)
        .get_result::<NotificationRecord>(conn)?;

    let nrs = notification
        .to
        .iter()
        .map(|a| new_notification_recipient_record(a, res.id, RecipientType::To))
        .chain(
            notification
                .cc
                .iter()
                .map(|a| new_notification_recipient_record(a, res.id, RecipientType::Cc)),
        )
        .chain(
            notification
                .bcc
                .iter()
                .map(|a| new_notification_recipient_record(a, res.id, RecipientType::Bcc)),
        );
    for nr in nrs {
        insert_into(notification_recipient::table)
            .values(nr)
            .execute(conn)?;
    }
    Ok(())
}

pub struct Notification {
    pub id: i64,
    pub notification_type: NotificationType,
    pub from: Option<String>,
    pub subject: String,
    pub body: Body,
    pub to: Vec<Address>,
    pub cc: Vec<Address>,
    pub bcc: Vec<Address>,
}

impl TryFrom<(NotificationRecord, Vec<NotificationRecipientRecord>)> for Notification {
    type Error = failure::Error;

    fn try_from(
        (n, mut nrs): (NotificationRecord, Vec<NotificationRecipientRecord>),
    ) -> Result<Self, Error> {
        let to = nrs
            .drain_filter(|nr| nr.recipient_type == RecipientType::To)
            .into_iter()
            .map(Address::from)
            .collect::<Vec<_>>();
        let cc = nrs
            .drain_filter(|nr| nr.recipient_type == RecipientType::Cc)
            .into_iter()
            .map(Address::from)
            .collect::<Vec<_>>();

        let bcc = nrs.into_iter().map(Address::from).collect::<Vec<_>>();
        Ok(Self {
            id: n.id,
            notification_type: n.notification_type,
            from: n.from_address,
            subject: n.subject,
            body: Body::try_from((n.body_html, n.body_plain_text))?,
            to,
            cc,
            bcc,
        })
    }
}

pub fn read_notification(conn: &Connection, notification_id: i64) -> Result<Notification, Error> {
    let nr: NotificationRecord = notification::table.find(notification_id).first(conn)?;
    let notf_addr: Vec<NotificationRecipientRecord> =
        NotificationRecipientRecord::belonging_to(&nr)
            .select(notification_recipient::all_columns)
            .load::<NotificationRecipientRecord>(conn)?;

    Notification::try_from((nr, notf_addr))
}

fn new_notification_recipient_record(
    address: &Address,
    id: i64,
    rt: RecipientType,
) -> NewNotificationRecipientRecord<'_> {
    NewNotificationRecipientRecord {
        notification_id: id,
        recipient_type: rt,
        name: address.name.deref(),
        address: address.address.as_ref(),
    }
}
