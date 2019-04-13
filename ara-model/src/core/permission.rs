use serde::{Deserialize, Serialize};
use crate::schema::permission;

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
)]
#[table_name = "permission"]
#[serde(rename_all = "camelCase")]
pub struct Permission {
    pub id: i32,
    pub application: String,
    pub authority: String,
    pub description: String,
}
/*
impl Permission {
    pub fn findAll(conn: &Connection) -> Result<Vec<Permission>, Error> {
        //crate::schema::permission::dsl::permission;
        let res =
            permission::select(permission::all_columns)
                .order(permission::application,permission::authority)
                .load::<Permission>(conn)?;
        Ok(res)
    }
}*/