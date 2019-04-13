use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "date_format"]
#[serde(rename_all = "camelCase")]
pub struct DateFormat {
    pub id: i32,
    pub c_format: String,
    pub date_picker_format: String,
    pub js_format: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "datetime_format"]
#[serde(rename_all = "camelCase")]
pub struct DatetimeFormat {
    pub id: i32,
    pub c_format: String,
    pub js_format: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "timezone"]
#[serde(rename_all = "camelCase")]
pub struct Timezone {
    pub id: i32,
    pub name: String,
    pub gmt_offset: String,
    pub location: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "language"]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub id: i32,
    pub name: String,
    pub locale: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "country"]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub dial_code: i16,
    pub currency: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "currency"]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub id: i16,
    pub code: String,
    pub symbol: String,
    pub name: String,
}





