mod user {
    use crate::core::role::RoleRecord;
    use crate::db::Connection;
    use crate::error::DbError;
    use crate::schema::{app_user, user_credential, user_role};
    use chrono::{DateTime, Utc};
    use diesel::associations::HasTable;
    use diesel::prelude::*;
    use diesel::query_builder::AsChangeset;
    use diesel::{delete, insert_into, update};
    use failure::{ensure, Error};
    use serde::{Deserialize, Serialize};
    use validator::{Validate, ValidationError};
    #[table_name = "app_user"]
    #[serde(rename_all = "camelCase")]
    #[structural_match]
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
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_queryable_for_userrecord() {
        extern crate std;
        use diesel;
        use diesel::Queryable;
        impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for UserRecord
        where
            (
                i64,
                String,
                String,
                String,
                String,
                Option<String>,
                bool,
                DateTime<Utc>,
                String,
                DateTime<Utc>,
                String,
                i32,
            ): Queryable<__ST, __DB>,
        {
            type Row = <(
                i64,
                String,
                String,
                String,
                String,
                Option<String>,
                bool,
                DateTime<Utc>,
                String,
                DateTime<Utc>,
                String,
                i32,
            ) as Queryable<__ST, __DB>>::Row;
            fn build(row: Self::Row) -> Self {
                let row: (
                    i64,
                    String,
                    String,
                    String,
                    String,
                    Option<String>,
                    bool,
                    DateTime<Utc>,
                    String,
                    DateTime<Utc>,
                    String,
                    i32,
                ) = Queryable::build(row);
                Self {
                    id: (row.0.into()),
                    first_name: (row.1.into()),
                    last_name: (row.2.into()),
                    username: (row.3.into()),
                    email: (row.4.into()),
                    phone_number: (row.5.into()),
                    active: (row.6.into()),
                    created_at: (row.7.into()),
                    created_by: (row.8.into()),
                    updated_at: (row.9.into()),
                    updated_by: (row.10.into()),
                    version: (row.11.into()),
                }
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_identifiable_for_userrecord() {
        extern crate std;
        use diesel;
        use diesel::associations::{HasTable, Identifiable};
        impl HasTable for UserRecord {
            type Table = app_user::table;
            fn table() -> Self::Table {
                app_user::table
            }
        }
        impl<'ident> Identifiable for &'ident UserRecord {
            type Id = (&'ident i64);
            fn id(self) -> Self::Id {
                (&self.id)
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_as_changeset_for_userrecord() {
        extern crate std;
        use diesel;
        use diesel::prelude::*;
        use diesel::query_builder::AsChangeset;
        impl<'update> AsChangeset for &'update UserRecord {
            type Target = app_user::table;
            type Changeset = <(
                diesel::dsl::Eq<app_user::first_name, &'update String>,
                diesel::dsl::Eq<app_user::last_name, &'update String>,
                diesel::dsl::Eq<app_user::username, &'update String>,
                diesel::dsl::Eq<app_user::email, &'update String>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, &'update String>>,
                diesel::dsl::Eq<app_user::active, &'update bool>,
                diesel::dsl::Eq<app_user::created_at, &'update DateTime<Utc>>,
                diesel::dsl::Eq<app_user::created_by, &'update String>,
                diesel::dsl::Eq<app_user::updated_at, &'update DateTime<Utc>>,
                diesel::dsl::Eq<app_user::updated_by, &'update String>,
                diesel::dsl::Eq<app_user::version, &'update i32>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    app_user::first_name.eq(&self.first_name),
                    app_user::last_name.eq(&self.last_name),
                    app_user::username.eq(&self.username),
                    app_user::email.eq(&self.email),
                    self.phone_number
                        .as_ref()
                        .map(|x| app_user::phone_number.eq(x)),
                    app_user::active.eq(&self.active),
                    app_user::created_at.eq(&self.created_at),
                    app_user::created_by.eq(&self.created_by),
                    app_user::updated_at.eq(&self.updated_at),
                    app_user::updated_by.eq(&self.updated_by),
                    app_user::version.eq(&self.version),
                )
                    .as_changeset()
            }
        }
        impl<'update> AsChangeset for UserRecord {
            type Target = app_user::table;
            type Changeset = <(
                diesel::dsl::Eq<app_user::first_name, String>,
                diesel::dsl::Eq<app_user::last_name, String>,
                diesel::dsl::Eq<app_user::username, String>,
                diesel::dsl::Eq<app_user::email, String>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, String>>,
                diesel::dsl::Eq<app_user::active, bool>,
                diesel::dsl::Eq<app_user::created_at, DateTime<Utc>>,
                diesel::dsl::Eq<app_user::created_by, String>,
                diesel::dsl::Eq<app_user::updated_at, DateTime<Utc>>,
                diesel::dsl::Eq<app_user::updated_by, String>,
                diesel::dsl::Eq<app_user::version, i32>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    app_user::first_name.eq(self.first_name),
                    app_user::last_name.eq(self.last_name),
                    app_user::username.eq(self.username),
                    app_user::email.eq(self.email),
                    self.phone_number.map(|x| app_user::phone_number.eq(x)),
                    app_user::active.eq(self.active),
                    app_user::created_at.eq(self.created_at),
                    app_user::created_by.eq(self.created_by),
                    app_user::updated_at.eq(self.updated_at),
                    app_user::updated_by.eq(self.updated_by),
                    app_user::version.eq(self.version),
                )
                    .as_changeset()
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_associations_for_userrecord() {
        extern crate std;
        use diesel;
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UserRecord {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UserRecord {
                    id: ref __self_0_0,
                    first_name: ref __self_0_1,
                    last_name: ref __self_0_2,
                    username: ref __self_0_3,
                    email: ref __self_0_4,
                    phone_number: ref __self_0_5,
                    active: ref __self_0_6,
                    created_at: ref __self_0_7,
                    created_by: ref __self_0_8,
                    updated_at: ref __self_0_9,
                    updated_by: ref __self_0_10,
                    version: ref __self_0_11,
                } => {
                    let mut debug_trait_builder = f.debug_struct("UserRecord");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("username", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("email", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("phone_number", &&(*__self_0_5));
                    let _ = debug_trait_builder.field("active", &&(*__self_0_6));
                    let _ = debug_trait_builder.field("created_at", &&(*__self_0_7));
                    let _ = debug_trait_builder.field("created_by", &&(*__self_0_8));
                    let _ = debug_trait_builder.field("updated_at", &&(*__self_0_9));
                    let _ = debug_trait_builder.field("updated_by", &&(*__self_0_10));
                    let _ = debug_trait_builder.field("version", &&(*__self_0_11));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UserRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UserRecord {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UserRecord",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "username",
                    &self.username,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "email",
                    &self.email,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phoneNumber",
                    &self.phone_number,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "active",
                    &self.active,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "createdAt",
                    &self.created_at,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "createdBy",
                    &self.created_by,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updatedAt",
                    &self.updated_at,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updatedBy",
                    &self.updated_by,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UserRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UserRecord {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            8u64 => _serde::export::Ok(__Field::__field8),
                            9u64 => _serde::export::Ok(__Field::__field9),
                            10u64 => _serde::export::Ok(__Field::__field10),
                            11u64 => _serde::export::Ok(__Field::__field11),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 12",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::export::Ok(__Field::__field0),
                            "firstName" => _serde::export::Ok(__Field::__field1),
                            "lastName" => _serde::export::Ok(__Field::__field2),
                            "username" => _serde::export::Ok(__Field::__field3),
                            "email" => _serde::export::Ok(__Field::__field4),
                            "phoneNumber" => _serde::export::Ok(__Field::__field5),
                            "active" => _serde::export::Ok(__Field::__field6),
                            "createdAt" => _serde::export::Ok(__Field::__field7),
                            "createdBy" => _serde::export::Ok(__Field::__field8),
                            "updatedAt" => _serde::export::Ok(__Field::__field9),
                            "updatedBy" => _serde::export::Ok(__Field::__field10),
                            "version" => _serde::export::Ok(__Field::__field11),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::export::Ok(__Field::__field0),
                            b"firstName" => _serde::export::Ok(__Field::__field1),
                            b"lastName" => _serde::export::Ok(__Field::__field2),
                            b"username" => _serde::export::Ok(__Field::__field3),
                            b"email" => _serde::export::Ok(__Field::__field4),
                            b"phoneNumber" => _serde::export::Ok(__Field::__field5),
                            b"active" => _serde::export::Ok(__Field::__field6),
                            b"createdAt" => _serde::export::Ok(__Field::__field7),
                            b"createdBy" => _serde::export::Ok(__Field::__field8),
                            b"updatedAt" => _serde::export::Ok(__Field::__field9),
                            b"updatedBy" => _serde::export::Ok(__Field::__field10),
                            b"version" => _serde::export::Ok(__Field::__field11),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UserRecord>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UserRecord;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct UserRecord")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct UserRecord with 12 elements",
                                ));
                            }
                        };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field7 = match match _serde::de::SeqAccess::next_element::<
                            DateTime<Utc>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    7usize,
                                    &"struct UserRecord with 12 elements",
                                ));
                            }
                        };
                        let __field8 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        8usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field9 = match match _serde::de::SeqAccess::next_element::<
                            DateTime<Utc>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    9usize,
                                    &"struct UserRecord with 12 elements",
                                ));
                            }
                        };
                        let __field10 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        10usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field11 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        11usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(UserRecord {
                            id: __field0,
                            first_name: __field1,
                            last_name: __field2,
                            username: __field3,
                            email: __field4,
                            phone_number: __field5,
                            active: __field6,
                            created_at: __field7,
                            created_by: __field8,
                            updated_at: __field9,
                            updated_by: __field10,
                            version: __field11,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<i64> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<String> = _serde::export::None;
                        let mut __field3: _serde::export::Option<String> = _serde::export::None;
                        let mut __field4: _serde::export::Option<String> = _serde::export::None;
                        let mut __field5: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field6: _serde::export::Option<bool> = _serde::export::None;
                        let mut __field7: _serde::export::Option<DateTime<Utc>> =
                            _serde::export::None;
                        let mut __field8: _serde::export::Option<String> = _serde::export::None;
                        let mut __field9: _serde::export::Option<DateTime<Utc>> =
                            _serde::export::None;
                        let mut __field10: _serde::export::Option<String> = _serde::export::None;
                        let mut __field11: _serde::export::Option<i32> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "username",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "email",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "phoneNumber",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "active",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "createdAt",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<DateTime<Utc>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::export::Option::is_some(&__field8) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "createdBy",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::export::Option::is_some(&__field9) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "updatedAt",
                                            ),
                                        );
                                    }
                                    __field9 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<DateTime<Utc>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field10 => {
                                    if _serde::export::Option::is_some(&__field10) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "updatedBy",
                                            ),
                                        );
                                    }
                                    __field10 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field11 => {
                                    if _serde::export::Option::is_some(&__field11) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "version",
                                            ),
                                        );
                                    }
                                    __field11 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("id") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("username") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("email") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("phoneNumber") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::export::Some(__field6) => __field6,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("active") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::export::Some(__field7) => __field7,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("createdAt") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::export::Some(__field8) => __field8,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("createdBy") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::export::Some(__field9) => __field9,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("updatedAt") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field10 = match __field10 {
                            _serde::export::Some(__field10) => __field10,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("updatedBy") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field11 = match __field11 {
                            _serde::export::Some(__field11) => __field11,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("version") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(UserRecord {
                            id: __field0,
                            first_name: __field1,
                            last_name: __field2,
                            username: __field3,
                            email: __field4,
                            phone_number: __field5,
                            active: __field6,
                            created_at: __field7,
                            created_by: __field8,
                            updated_at: __field9,
                            updated_by: __field10,
                            version: __field11,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "firstName",
                    "lastName",
                    "username",
                    "email",
                    "phoneNumber",
                    "active",
                    "createdAt",
                    "createdBy",
                    "updatedAt",
                    "updatedBy",
                    "version",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UserRecord",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<UserRecord>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UserRecord {
        #[inline]
        fn clone(&self) -> UserRecord {
            match *self {
                UserRecord {
                    id: ref __self_0_0,
                    first_name: ref __self_0_1,
                    last_name: ref __self_0_2,
                    username: ref __self_0_3,
                    email: ref __self_0_4,
                    phone_number: ref __self_0_5,
                    active: ref __self_0_6,
                    created_at: ref __self_0_7,
                    created_by: ref __self_0_8,
                    updated_at: ref __self_0_9,
                    updated_by: ref __self_0_10,
                    version: ref __self_0_11,
                } => UserRecord {
                    id: ::std::clone::Clone::clone(&(*__self_0_0)),
                    first_name: ::std::clone::Clone::clone(&(*__self_0_1)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_2)),
                    username: ::std::clone::Clone::clone(&(*__self_0_3)),
                    email: ::std::clone::Clone::clone(&(*__self_0_4)),
                    phone_number: ::std::clone::Clone::clone(&(*__self_0_5)),
                    active: ::std::clone::Clone::clone(&(*__self_0_6)),
                    created_at: ::std::clone::Clone::clone(&(*__self_0_7)),
                    created_by: ::std::clone::Clone::clone(&(*__self_0_8)),
                    updated_at: ::std::clone::Clone::clone(&(*__self_0_9)),
                    updated_by: ::std::clone::Clone::clone(&(*__self_0_10)),
                    version: ::std::clone::Clone::clone(&(*__self_0_11)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for UserRecord {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<i64>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<Option<String>>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
                let _: ::std::cmp::AssertParamIsEq<DateTime<Utc>>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<DateTime<Utc>>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<i32>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for UserRecord {
        #[inline]
        fn eq(&self, other: &UserRecord) -> bool {
            match *other {
                UserRecord {
                    id: ref __self_1_0,
                    first_name: ref __self_1_1,
                    last_name: ref __self_1_2,
                    username: ref __self_1_3,
                    email: ref __self_1_4,
                    phone_number: ref __self_1_5,
                    active: ref __self_1_6,
                    created_at: ref __self_1_7,
                    created_by: ref __self_1_8,
                    updated_at: ref __self_1_9,
                    updated_by: ref __self_1_10,
                    version: ref __self_1_11,
                } => match *self {
                    UserRecord {
                        id: ref __self_0_0,
                        first_name: ref __self_0_1,
                        last_name: ref __self_0_2,
                        username: ref __self_0_3,
                        email: ref __self_0_4,
                        phone_number: ref __self_0_5,
                        active: ref __self_0_6,
                        created_at: ref __self_0_7,
                        created_by: ref __self_0_8,
                        updated_at: ref __self_0_9,
                        updated_by: ref __self_0_10,
                        version: ref __self_0_11,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                            && (*__self_0_6) == (*__self_1_6)
                            && (*__self_0_7) == (*__self_1_7)
                            && (*__self_0_8) == (*__self_1_8)
                            && (*__self_0_9) == (*__self_1_9)
                            && (*__self_0_10) == (*__self_1_10)
                            && (*__self_0_11) == (*__self_1_11)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &UserRecord) -> bool {
            match *other {
                UserRecord {
                    id: ref __self_1_0,
                    first_name: ref __self_1_1,
                    last_name: ref __self_1_2,
                    username: ref __self_1_3,
                    email: ref __self_1_4,
                    phone_number: ref __self_1_5,
                    active: ref __self_1_6,
                    created_at: ref __self_1_7,
                    created_by: ref __self_1_8,
                    updated_at: ref __self_1_9,
                    updated_by: ref __self_1_10,
                    version: ref __self_1_11,
                } => match *self {
                    UserRecord {
                        id: ref __self_0_0,
                        first_name: ref __self_0_1,
                        last_name: ref __self_0_2,
                        username: ref __self_0_3,
                        email: ref __self_0_4,
                        phone_number: ref __self_0_5,
                        active: ref __self_0_6,
                        created_at: ref __self_0_7,
                        created_by: ref __self_0_8,
                        updated_at: ref __self_0_9,
                        updated_by: ref __self_0_10,
                        version: ref __self_0_11,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                            || (*__self_0_6) != (*__self_1_6)
                            || (*__self_0_7) != (*__self_1_7)
                            || (*__self_0_8) != (*__self_1_8)
                            || (*__self_0_9) != (*__self_1_9)
                            || (*__self_0_10) != (*__self_1_10)
                            || (*__self_0_11) != (*__self_1_11)
                    }
                },
            }
        }
    }
    impl Validate for UserRecord {
        #[allow(unused_mut)]
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            let mut errors = ::validator::ValidationErrors::new();
            if !::validator::validate_length(
                ::validator::Validator::Length {
                    min: ::std::option::Option::Some(2u64),
                    max: ::std::option::Option::None,
                    equal: ::std::option::Option::None,
                },
                &self.username,
            ) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &2u64);
                err.add_param(::std::borrow::Cow::from("value"), &&self.username);
                errors.add("username", err);
            }
            if !::validator::validate_email(&self.email) {
                let mut err = ::validator::ValidationError::new("email");
                err.add_param(::std::borrow::Cow::from("value"), &&self.email);
                errors.add("email", err);
            }
            let mut result = if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            };
            result
        }
    }
    #[table_name = "app_user"]
    #[serde(rename_all = "camelCase")]
    #[structural_match]
    pub struct UserUpdate {
        pub id: i64,
        pub first_name: String,
        pub last_name: String,
        pub phone_number: Option<String>,
        pub active: bool,
        pub version: i32,
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_queryable_for_userupdate() {
        extern crate std;
        use diesel;
        use diesel::Queryable;
        impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for UserUpdate
        where
            (i64, String, String, Option<String>, bool, i32): Queryable<__ST, __DB>,
        {
            type Row =
                <(i64, String, String, Option<String>, bool, i32) as Queryable<__ST, __DB>>::Row;
            fn build(row: Self::Row) -> Self {
                let row: (i64, String, String, Option<String>, bool, i32) = Queryable::build(row);
                Self {
                    id: (row.0.into()),
                    first_name: (row.1.into()),
                    last_name: (row.2.into()),
                    phone_number: (row.3.into()),
                    active: (row.4.into()),
                    version: (row.5.into()),
                }
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_identifiable_for_userupdate() {
        extern crate std;
        use diesel;
        use diesel::associations::{HasTable, Identifiable};
        impl HasTable for UserUpdate {
            type Table = app_user::table;
            fn table() -> Self::Table {
                app_user::table
            }
        }
        impl<'ident> Identifiable for &'ident UserUpdate {
            type Id = (&'ident i64);
            fn id(self) -> Self::Id {
                (&self.id)
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_as_changeset_for_userupdate() {
        extern crate std;
        use diesel;
        use diesel::prelude::*;
        use diesel::query_builder::AsChangeset;
        impl<'update> AsChangeset for &'update UserUpdate {
            type Target = app_user::table;
            type Changeset = <(
                diesel::dsl::Eq<app_user::first_name, &'update String>,
                diesel::dsl::Eq<app_user::last_name, &'update String>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, &'update String>>,
                diesel::dsl::Eq<app_user::active, &'update bool>,
                diesel::dsl::Eq<app_user::version, &'update i32>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    app_user::first_name.eq(&self.first_name),
                    app_user::last_name.eq(&self.last_name),
                    self.phone_number
                        .as_ref()
                        .map(|x| app_user::phone_number.eq(x)),
                    app_user::active.eq(&self.active),
                    app_user::version.eq(&self.version),
                )
                    .as_changeset()
            }
        }
        impl<'update> AsChangeset for UserUpdate {
            type Target = app_user::table;
            type Changeset = <(
                diesel::dsl::Eq<app_user::first_name, String>,
                diesel::dsl::Eq<app_user::last_name, String>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, String>>,
                diesel::dsl::Eq<app_user::active, bool>,
                diesel::dsl::Eq<app_user::version, i32>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    app_user::first_name.eq(self.first_name),
                    app_user::last_name.eq(self.last_name),
                    self.phone_number.map(|x| app_user::phone_number.eq(x)),
                    app_user::active.eq(self.active),
                    app_user::version.eq(self.version),
                )
                    .as_changeset()
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_associations_for_userupdate() {
        extern crate std;
        use diesel;
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UserUpdate {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UserUpdate {
                    id: ref __self_0_0,
                    first_name: ref __self_0_1,
                    last_name: ref __self_0_2,
                    phone_number: ref __self_0_3,
                    active: ref __self_0_4,
                    version: ref __self_0_5,
                } => {
                    let mut debug_trait_builder = f.debug_struct("UserUpdate");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("phone_number", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("active", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("version", &&(*__self_0_5));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UserUpdate: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UserUpdate {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UserUpdate",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phoneNumber",
                    &self.phone_number,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "active",
                    &self.active,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UserUpdate: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UserUpdate {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 6",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::export::Ok(__Field::__field0),
                            "firstName" => _serde::export::Ok(__Field::__field1),
                            "lastName" => _serde::export::Ok(__Field::__field2),
                            "phoneNumber" => _serde::export::Ok(__Field::__field3),
                            "active" => _serde::export::Ok(__Field::__field4),
                            "version" => _serde::export::Ok(__Field::__field5),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::export::Ok(__Field::__field0),
                            b"firstName" => _serde::export::Ok(__Field::__field1),
                            b"lastName" => _serde::export::Ok(__Field::__field2),
                            b"phoneNumber" => _serde::export::Ok(__Field::__field3),
                            b"active" => _serde::export::Ok(__Field::__field4),
                            b"version" => _serde::export::Ok(__Field::__field5),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UserUpdate>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UserUpdate;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct UserUpdate")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UserUpdate with 6 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UserUpdate with 6 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UserUpdate with 6 elements",
                                    ));
                                }
                            };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct UserUpdate with 6 elements",
                                ));
                            }
                        };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct UserUpdate with 6 elements",
                                    ));
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct UserUpdate with 6 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(UserUpdate {
                            id: __field0,
                            first_name: __field1,
                            last_name: __field2,
                            phone_number: __field3,
                            active: __field4,
                            version: __field5,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<i64> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<String> = _serde::export::None;
                        let mut __field3: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field4: _serde::export::Option<bool> = _serde::export::None;
                        let mut __field5: _serde::export::Option<i32> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "phoneNumber",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "active",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "version",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("id") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("phoneNumber") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("active") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("version") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(UserUpdate {
                            id: __field0,
                            first_name: __field1,
                            last_name: __field2,
                            phone_number: __field3,
                            active: __field4,
                            version: __field5,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "firstName",
                    "lastName",
                    "phoneNumber",
                    "active",
                    "version",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UserUpdate",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<UserUpdate>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UserUpdate {
        #[inline]
        fn clone(&self) -> UserUpdate {
            match *self {
                UserUpdate {
                    id: ref __self_0_0,
                    first_name: ref __self_0_1,
                    last_name: ref __self_0_2,
                    phone_number: ref __self_0_3,
                    active: ref __self_0_4,
                    version: ref __self_0_5,
                } => UserUpdate {
                    id: ::std::clone::Clone::clone(&(*__self_0_0)),
                    first_name: ::std::clone::Clone::clone(&(*__self_0_1)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_2)),
                    phone_number: ::std::clone::Clone::clone(&(*__self_0_3)),
                    active: ::std::clone::Clone::clone(&(*__self_0_4)),
                    version: ::std::clone::Clone::clone(&(*__self_0_5)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for UserUpdate {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<i64>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<Option<String>>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
                let _: ::std::cmp::AssertParamIsEq<i32>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for UserUpdate {
        #[inline]
        fn eq(&self, other: &UserUpdate) -> bool {
            match *other {
                UserUpdate {
                    id: ref __self_1_0,
                    first_name: ref __self_1_1,
                    last_name: ref __self_1_2,
                    phone_number: ref __self_1_3,
                    active: ref __self_1_4,
                    version: ref __self_1_5,
                } => match *self {
                    UserUpdate {
                        id: ref __self_0_0,
                        first_name: ref __self_0_1,
                        last_name: ref __self_0_2,
                        phone_number: ref __self_0_3,
                        active: ref __self_0_4,
                        version: ref __self_0_5,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &UserUpdate) -> bool {
            match *other {
                UserUpdate {
                    id: ref __self_1_0,
                    first_name: ref __self_1_1,
                    last_name: ref __self_1_2,
                    phone_number: ref __self_1_3,
                    active: ref __self_1_4,
                    version: ref __self_1_5,
                } => match *self {
                    UserUpdate {
                        id: ref __self_0_0,
                        first_name: ref __self_0_1,
                        last_name: ref __self_0_2,
                        phone_number: ref __self_0_3,
                        active: ref __self_0_4,
                        version: ref __self_0_5,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                    }
                },
            }
        }
    }
    impl Validate for UserUpdate {
        #[allow(unused_mut)]
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            let mut errors = ::validator::ValidationErrors::new();
            let mut result = if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            };
            result
        }
    }
    #[serde(rename_all = "camelCase")]
    #[structural_match]
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
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for User {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                User {
                    id: ref __self_0_0,
                    username: ref __self_0_1,
                    first_name: ref __self_0_2,
                    last_name: ref __self_0_3,
                    email: ref __self_0_4,
                    phone_number: ref __self_0_5,
                    active: ref __self_0_6,
                    version: ref __self_0_7,
                } => {
                    let mut debug_trait_builder = f.debug_struct("User");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("username", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("email", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("phone_number", &&(*__self_0_5));
                    let _ = debug_trait_builder.field("active", &&(*__self_0_6));
                    let _ = debug_trait_builder.field("version", &&(*__self_0_7));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_User: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for User {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "User",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "username",
                    &self.username,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "email",
                    &self.email,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phoneNumber",
                    &self.phone_number,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "active",
                    &self.active,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_User: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for User {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 8",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::export::Ok(__Field::__field0),
                            "username" => _serde::export::Ok(__Field::__field1),
                            "firstName" => _serde::export::Ok(__Field::__field2),
                            "lastName" => _serde::export::Ok(__Field::__field3),
                            "email" => _serde::export::Ok(__Field::__field4),
                            "phoneNumber" => _serde::export::Ok(__Field::__field5),
                            "active" => _serde::export::Ok(__Field::__field6),
                            "version" => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::export::Ok(__Field::__field0),
                            b"username" => _serde::export::Ok(__Field::__field1),
                            b"firstName" => _serde::export::Ok(__Field::__field2),
                            b"lastName" => _serde::export::Ok(__Field::__field3),
                            b"email" => _serde::export::Ok(__Field::__field4),
                            b"phoneNumber" => _serde::export::Ok(__Field::__field5),
                            b"active" => _serde::export::Ok(__Field::__field6),
                            b"version" => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<User>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = User;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct User")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct User with 8 elements",
                                ));
                            }
                        };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field7 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(User {
                            id: __field0,
                            username: __field1,
                            first_name: __field2,
                            last_name: __field3,
                            email: __field4,
                            phone_number: __field5,
                            active: __field6,
                            version: __field7,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<i64> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<String> = _serde::export::None;
                        let mut __field3: _serde::export::Option<String> = _serde::export::None;
                        let mut __field4: _serde::export::Option<String> = _serde::export::None;
                        let mut __field5: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field6: _serde::export::Option<bool> = _serde::export::None;
                        let mut __field7: _serde::export::Option<i32> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "username",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "email",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "phoneNumber",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "active",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "version",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("id") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("username") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("email") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("phoneNumber") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::export::Some(__field6) => __field6,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("active") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::export::Some(__field7) => __field7,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("version") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(User {
                            id: __field0,
                            username: __field1,
                            first_name: __field2,
                            last_name: __field3,
                            email: __field4,
                            phone_number: __field5,
                            active: __field6,
                            version: __field7,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "username",
                    "firstName",
                    "lastName",
                    "email",
                    "phoneNumber",
                    "active",
                    "version",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "User",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<User>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for User {
        #[inline]
        fn clone(&self) -> User {
            match *self {
                User {
                    id: ref __self_0_0,
                    username: ref __self_0_1,
                    first_name: ref __self_0_2,
                    last_name: ref __self_0_3,
                    email: ref __self_0_4,
                    phone_number: ref __self_0_5,
                    active: ref __self_0_6,
                    version: ref __self_0_7,
                } => User {
                    id: ::std::clone::Clone::clone(&(*__self_0_0)),
                    username: ::std::clone::Clone::clone(&(*__self_0_1)),
                    first_name: ::std::clone::Clone::clone(&(*__self_0_2)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_3)),
                    email: ::std::clone::Clone::clone(&(*__self_0_4)),
                    phone_number: ::std::clone::Clone::clone(&(*__self_0_5)),
                    active: ::std::clone::Clone::clone(&(*__self_0_6)),
                    version: ::std::clone::Clone::clone(&(*__self_0_7)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for User {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<i64>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<Option<String>>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
                let _: ::std::cmp::AssertParamIsEq<i32>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for User {
        #[inline]
        fn eq(&self, other: &User) -> bool {
            match *other {
                User {
                    id: ref __self_1_0,
                    username: ref __self_1_1,
                    first_name: ref __self_1_2,
                    last_name: ref __self_1_3,
                    email: ref __self_1_4,
                    phone_number: ref __self_1_5,
                    active: ref __self_1_6,
                    version: ref __self_1_7,
                } => match *self {
                    User {
                        id: ref __self_0_0,
                        username: ref __self_0_1,
                        first_name: ref __self_0_2,
                        last_name: ref __self_0_3,
                        email: ref __self_0_4,
                        phone_number: ref __self_0_5,
                        active: ref __self_0_6,
                        version: ref __self_0_7,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                            && (*__self_0_6) == (*__self_1_6)
                            && (*__self_0_7) == (*__self_1_7)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &User) -> bool {
            match *other {
                User {
                    id: ref __self_1_0,
                    username: ref __self_1_1,
                    first_name: ref __self_1_2,
                    last_name: ref __self_1_3,
                    email: ref __self_1_4,
                    phone_number: ref __self_1_5,
                    active: ref __self_1_6,
                    version: ref __self_1_7,
                } => match *self {
                    User {
                        id: ref __self_0_0,
                        username: ref __self_0_1,
                        first_name: ref __self_0_2,
                        last_name: ref __self_0_3,
                        email: ref __self_0_4,
                        phone_number: ref __self_0_5,
                        active: ref __self_0_6,
                        version: ref __self_0_7,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                            || (*__self_0_6) != (*__self_1_6)
                            || (*__self_0_7) != (*__self_1_7)
                    }
                },
            }
        }
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
    #[table_name = "app_user"]
    #[serde(rename_all = "camelCase")]
    pub struct NewUserRecord<'a> {
        pub first_name: &'a str,
        pub last_name: &'a str,
        pub username: &'a str,
        pub email: &'a str,
        pub phone_number: Option<&'a str>,
        pub created_by: &'a str,
        pub updated_by: &'a str,
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_insertable_for_newuserrecord() {
        extern crate std;
        use diesel;
        use diesel::insertable::Insertable;
        use diesel::prelude::*;
        use diesel::query_builder::UndecoratedInsertRecord;
        impl<'a, 'insert> Insertable<app_user::table> for NewUserRecord<'a> {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<app_user::first_name, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::last_name, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::username, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::email, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::created_by, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::updated_by, &'a str>>,
            ) as Insertable<app_user::table>>::Values;
            fn values(self) -> Self::Values {
                (
                    std::option::Option::Some(app_user::first_name.eq(self.first_name)),
                    std::option::Option::Some(app_user::last_name.eq(self.last_name)),
                    std::option::Option::Some(app_user::username.eq(self.username)),
                    std::option::Option::Some(app_user::email.eq(self.email)),
                    self.phone_number.map(|x| app_user::phone_number.eq(x)),
                    std::option::Option::Some(app_user::created_by.eq(self.created_by)),
                    std::option::Option::Some(app_user::updated_by.eq(self.updated_by)),
                )
                    .values()
            }
        }
        impl<'a, 'insert> Insertable<app_user::table> for &'insert NewUserRecord<'a> {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<app_user::first_name, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::last_name, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::username, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::email, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::created_by, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::updated_by, &'insert &'a str>>,
            ) as Insertable<app_user::table>>::Values;
            fn values(self) -> Self::Values {
                (
                    std::option::Option::Some(app_user::first_name.eq(&self.first_name)),
                    std::option::Option::Some(app_user::last_name.eq(&self.last_name)),
                    std::option::Option::Some(app_user::username.eq(&self.username)),
                    std::option::Option::Some(app_user::email.eq(&self.email)),
                    self.phone_number
                        .as_ref()
                        .map(|x| app_user::phone_number.eq(x)),
                    std::option::Option::Some(app_user::created_by.eq(&self.created_by)),
                    std::option::Option::Some(app_user::updated_by.eq(&self.updated_by)),
                )
                    .values()
            }
        }
        impl<'a, 'insert> UndecoratedInsertRecord<app_user::table> for NewUserRecord<'a> {}
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::fmt::Debug for NewUserRecord<'a> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                NewUserRecord {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    username: ref __self_0_2,
                    email: ref __self_0_3,
                    phone_number: ref __self_0_4,
                    created_by: ref __self_0_5,
                    updated_by: ref __self_0_6,
                } => {
                    let mut debug_trait_builder = f.debug_struct("NewUserRecord");
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("username", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("email", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("phone_number", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("created_by", &&(*__self_0_5));
                    let _ = debug_trait_builder.field("updated_by", &&(*__self_0_6));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_NewUserRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'a> _serde::Serialize for NewUserRecord<'a> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "NewUserRecord",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "username",
                    &self.username,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "email",
                    &self.email,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phoneNumber",
                    &self.phone_number,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "createdBy",
                    &self.created_by,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updatedBy",
                    &self.updated_by,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_NewUserRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de: 'a, 'a> _serde::Deserialize<'de> for NewUserRecord<'a> {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 7",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "firstName" => _serde::export::Ok(__Field::__field0),
                            "lastName" => _serde::export::Ok(__Field::__field1),
                            "username" => _serde::export::Ok(__Field::__field2),
                            "email" => _serde::export::Ok(__Field::__field3),
                            "phoneNumber" => _serde::export::Ok(__Field::__field4),
                            "createdBy" => _serde::export::Ok(__Field::__field5),
                            "updatedBy" => _serde::export::Ok(__Field::__field6),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"firstName" => _serde::export::Ok(__Field::__field0),
                            b"lastName" => _serde::export::Ok(__Field::__field1),
                            b"username" => _serde::export::Ok(__Field::__field2),
                            b"email" => _serde::export::Ok(__Field::__field3),
                            b"phoneNumber" => _serde::export::Ok(__Field::__field4),
                            b"createdBy" => _serde::export::Ok(__Field::__field5),
                            b"updatedBy" => _serde::export::Ok(__Field::__field6),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de: 'a, 'a> {
                    marker: _serde::export::PhantomData<NewUserRecord<'a>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                    type Value = NewUserRecord<'a>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct NewUserRecord")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<&'a str>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(NewUserRecord {
                            first_name: __field0,
                            last_name: __field1,
                            username: __field2,
                            email: __field3,
                            phone_number: __field4,
                            created_by: __field5,
                            updated_by: __field6,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field1: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field2: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field3: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field4: _serde::export::Option<Option<&'a str>> =
                            _serde::export::None;
                        let mut __field5: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field6: _serde::export::Option<&'a str> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "username",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "email",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "phoneNumber",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<&'a str>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "createdBy",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "updatedBy",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("username") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("email") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("phoneNumber") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("createdBy") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::export::Some(__field6) => __field6,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("updatedBy") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(NewUserRecord {
                            first_name: __field0,
                            last_name: __field1,
                            username: __field2,
                            email: __field3,
                            phone_number: __field4,
                            created_by: __field5,
                            updated_by: __field6,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "firstName",
                    "lastName",
                    "username",
                    "email",
                    "phoneNumber",
                    "createdBy",
                    "updatedBy",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "NewUserRecord",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<NewUserRecord<'a>>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::clone::Clone for NewUserRecord<'a> {
        #[inline]
        fn clone(&self) -> NewUserRecord<'a> {
            match *self {
                NewUserRecord {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    username: ref __self_0_2,
                    email: ref __self_0_3,
                    phone_number: ref __self_0_4,
                    created_by: ref __self_0_5,
                    updated_by: ref __self_0_6,
                } => NewUserRecord {
                    first_name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_1)),
                    username: ::std::clone::Clone::clone(&(*__self_0_2)),
                    email: ::std::clone::Clone::clone(&(*__self_0_3)),
                    phone_number: ::std::clone::Clone::clone(&(*__self_0_4)),
                    created_by: ::std::clone::Clone::clone(&(*__self_0_5)),
                    updated_by: ::std::clone::Clone::clone(&(*__self_0_6)),
                },
            }
        }
    }
    #[serde(rename_all = "camelCase")]
    pub struct NewUser<'a> {
        pub first_name: &'a str,
        pub last_name: &'a str,
        pub email: &'a str,
        pub phone_number: Option<&'a str>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::fmt::Debug for NewUser<'a> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                NewUser {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    email: ref __self_0_2,
                    phone_number: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("NewUser");
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("email", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("phone_number", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_NewUser: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'a> _serde::Serialize for NewUser<'a> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "NewUser",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "email",
                    &self.email,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phoneNumber",
                    &self.phone_number,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_NewUser: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de: 'a, 'a> _serde::Deserialize<'de> for NewUser<'a> {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 4",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "firstName" => _serde::export::Ok(__Field::__field0),
                            "lastName" => _serde::export::Ok(__Field::__field1),
                            "email" => _serde::export::Ok(__Field::__field2),
                            "phoneNumber" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"firstName" => _serde::export::Ok(__Field::__field0),
                            b"lastName" => _serde::export::Ok(__Field::__field1),
                            b"email" => _serde::export::Ok(__Field::__field2),
                            b"phoneNumber" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de: 'a, 'a> {
                    marker: _serde::export::PhantomData<NewUser<'a>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                    type Value = NewUser<'a>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct NewUser")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct NewUser with 4 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct NewUser with 4 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct NewUser with 4 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<&'a str>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct NewUser with 4 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(NewUser {
                            first_name: __field0,
                            last_name: __field1,
                            email: __field2,
                            phone_number: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field1: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field2: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field3: _serde::export::Option<Option<&'a str>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "email",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "phoneNumber",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<&'a str>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("email") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("phoneNumber") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(NewUser {
                            first_name: __field0,
                            last_name: __field1,
                            email: __field2,
                            phone_number: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["firstName", "lastName", "email", "phoneNumber"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "NewUser",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<NewUser<'a>>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::clone::Clone for NewUser<'a> {
        #[inline]
        fn clone(&self) -> NewUser<'a> {
            match *self {
                NewUser {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    email: ref __self_0_2,
                    phone_number: ref __self_0_3,
                } => NewUser {
                    first_name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_1)),
                    email: ::std::clone::Clone::clone(&(*__self_0_2)),
                    phone_number: ::std::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[serde(rename_all = "camelCase")]
    pub struct UserSignUp {
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub password: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UserSignUp {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UserSignUp {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    email: ref __self_0_2,
                    password: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("UserSignUp");
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("email", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("password", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UserSignUp: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UserSignUp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UserSignUp",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "email",
                    &self.email,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "password",
                    &self.password,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UserSignUp: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UserSignUp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 4",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "firstName" => _serde::export::Ok(__Field::__field0),
                            "lastName" => _serde::export::Ok(__Field::__field1),
                            "email" => _serde::export::Ok(__Field::__field2),
                            "password" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"firstName" => _serde::export::Ok(__Field::__field0),
                            b"lastName" => _serde::export::Ok(__Field::__field1),
                            b"email" => _serde::export::Ok(__Field::__field2),
                            b"password" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UserSignUp>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UserSignUp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct UserSignUp")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UserSignUp with 4 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UserSignUp with 4 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UserSignUp with 4 elements",
                                    ));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct UserSignUp with 4 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(UserSignUp {
                            first_name: __field0,
                            last_name: __field1,
                            email: __field2,
                            password: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<String> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<String> = _serde::export::None;
                        let mut __field3: _serde::export::Option<String> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "email",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "password",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("email") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("password") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(UserSignUp {
                            first_name: __field0,
                            last_name: __field1,
                            email: __field2,
                            password: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["firstName", "lastName", "email", "password"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UserSignUp",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<UserSignUp>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UserSignUp {
        #[inline]
        fn clone(&self) -> UserSignUp {
            match *self {
                UserSignUp {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    email: ref __self_0_2,
                    password: ref __self_0_3,
                } => UserSignUp {
                    first_name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_1)),
                    email: ::std::clone::Clone::clone(&(*__self_0_2)),
                    password: ::std::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[serde(rename_all = "camelCase")]
    pub struct UserLogin {
        pub username: String,
        pub password: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UserLogin {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UserLogin {
                    username: ref __self_0_0,
                    password: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("UserLogin");
                    let _ = debug_trait_builder.field("username", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("password", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UserLogin: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UserLogin {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UserLogin",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "username",
                    &self.username,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "password",
                    &self.password,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UserLogin: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UserLogin {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "username" => _serde::export::Ok(__Field::__field0),
                            "password" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"username" => _serde::export::Ok(__Field::__field0),
                            b"password" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UserLogin>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UserLogin;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct UserLogin")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UserLogin with 2 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UserLogin with 2 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(UserLogin {
                            username: __field0,
                            password: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<String> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "username",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "password",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("username") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("password") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(UserLogin {
                            username: __field0,
                            password: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["username", "password"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UserLogin",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<UserLogin>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UserLogin {
        #[inline]
        fn clone(&self) -> UserLogin {
            match *self {
                UserLogin {
                    username: ref __self_0_0,
                    password: ref __self_0_1,
                } => UserLogin {
                    username: ::std::clone::Clone::clone(&(*__self_0_0)),
                    password: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[belongs_to(UserRecord, foreign_key = "user_id")]
    #[belongs_to(RoleRecord, foreign_key = "role_id")]
    #[table_name = "user_role"]
    #[primary_key(user_id, role_id)]
    #[serde(rename_all = "camelCase")]
    pub struct UserRoleRecord {
        pub user_id: i64,
        pub role_id: i32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UserRoleRecord {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UserRoleRecord {
                    user_id: ref __self_0_0,
                    role_id: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("UserRoleRecord");
                    let _ = debug_trait_builder.field("user_id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("role_id", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UserRoleRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UserRoleRecord {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UserRoleRecord",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "userId",
                    &self.user_id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "roleId",
                    &self.role_id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UserRoleRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UserRoleRecord {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "userId" => _serde::export::Ok(__Field::__field0),
                            "roleId" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"userId" => _serde::export::Ok(__Field::__field0),
                            b"roleId" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UserRoleRecord>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UserRoleRecord;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct UserRoleRecord")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UserRoleRecord with 2 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UserRoleRecord with 2 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(UserRoleRecord {
                            user_id: __field0,
                            role_id: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<i64> = _serde::export::None;
                        let mut __field1: _serde::export::Option<i32> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "userId",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "roleId",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("userId") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("roleId") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(UserRoleRecord {
                            user_id: __field0,
                            role_id: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["userId", "roleId"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UserRoleRecord",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<UserRoleRecord>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_identifiable_for_userrolerecord() {
        extern crate std;
        use diesel;
        use diesel::associations::{HasTable, Identifiable};
        impl HasTable for UserRoleRecord {
            type Table = user_role::table;
            fn table() -> Self::Table {
                user_role::table
            }
        }
        impl<'ident> Identifiable for &'ident UserRoleRecord {
            type Id = (&'ident i64, &'ident i32);
            fn id(self) -> Self::Id {
                (&self.user_id, &self.role_id)
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_queryable_for_userrolerecord() {
        extern crate std;
        use diesel;
        use diesel::Queryable;
        impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for UserRoleRecord
        where
            (i64, i32): Queryable<__ST, __DB>,
        {
            type Row = <(i64, i32) as Queryable<__ST, __DB>>::Row;
            fn build(row: Self::Row) -> Self {
                let row: (i64, i32) = Queryable::build(row);
                Self {
                    user_id: (row.0.into()),
                    role_id: (row.1.into()),
                }
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_associations_for_userrolerecord() {
        extern crate std;
        use diesel;
        impl<__FK> diesel::associations::BelongsTo<UserRecord> for UserRoleRecord
        where
            __FK: std::hash::Hash + std::cmp::Eq,
            for<'__a> &'__a i64: std::convert::Into<::std::option::Option<&'__a __FK>>,
            for<'__a> &'__a UserRecord: diesel::associations::Identifiable<Id = &'__a __FK>,
        {
            type ForeignKey = __FK;
            type ForeignKeyColumn = user_role::user_id;
            fn foreign_key(&self) -> std::option::Option<&Self::ForeignKey> {
                std::convert::Into::into(&self.user_id)
            }
            fn foreign_key_column() -> Self::ForeignKeyColumn {
                user_role::user_id
            }
        }
        impl<__FK> diesel::associations::BelongsTo<RoleRecord> for UserRoleRecord
        where
            __FK: std::hash::Hash + std::cmp::Eq,
            for<'__a> &'__a i32: std::convert::Into<::std::option::Option<&'__a __FK>>,
            for<'__a> &'__a RoleRecord: diesel::associations::Identifiable<Id = &'__a __FK>,
        {
            type ForeignKey = __FK;
            type ForeignKeyColumn = user_role::role_id;
            fn foreign_key(&self) -> std::option::Option<&Self::ForeignKey> {
                std::convert::Into::into(&self.role_id)
            }
            fn foreign_key_column() -> Self::ForeignKeyColumn {
                user_role::role_id
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_insertable_for_userrolerecord() {
        extern crate std;
        use diesel;
        use diesel::insertable::Insertable;
        use diesel::prelude::*;
        use diesel::query_builder::UndecoratedInsertRecord;
        impl<'insert> Insertable<user_role::table> for UserRoleRecord {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<user_role::user_id, i64>>,
                std::option::Option<diesel::dsl::Eq<user_role::role_id, i32>>,
            ) as Insertable<user_role::table>>::Values;
            fn values(self) -> Self::Values {
                (
                    std::option::Option::Some(user_role::user_id.eq(self.user_id)),
                    std::option::Option::Some(user_role::role_id.eq(self.role_id)),
                )
                    .values()
            }
        }
        impl<'insert> Insertable<user_role::table> for &'insert UserRoleRecord {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<user_role::user_id, &'insert i64>>,
                std::option::Option<diesel::dsl::Eq<user_role::role_id, &'insert i32>>,
            ) as Insertable<user_role::table>>::Values;
            fn values(self) -> Self::Values {
                (
                    std::option::Option::Some(user_role::user_id.eq(&self.user_id)),
                    std::option::Option::Some(user_role::role_id.eq(&self.role_id)),
                )
                    .values()
            }
        }
        impl<'insert> UndecoratedInsertRecord<user_role::table> for UserRoleRecord {}
    }
    impl User {
        pub fn insert(
            conn: &Connection,
            new_user: &WithAudit<'_, NewUser<'_>>,
        ) -> Result<UserRecord, Error> {
            let res = insert_into(app_user::table)
                .values(NewUserRecord::from(new_user))
                .get_result::<UserRecord>(conn)?;
            Ok(res)
        }
        pub fn update(
            conn: &Connection,
            update_user: &WithAudit<'_, UserUpdate>,
        ) -> Result<UserRecord, Error> {
            use crate::schema::app_user::columns::version;
            let target = app_user::table.filter(
                app_user::id
                    .eq(update_user.id)
                    .and(version.eq(update_user.version)),
            );
            let res = update(target)
                .set(update_user)
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
            if !(res == 1) {
                return Err(::failure::err_msg(DbError::IncorrectResultSize(1, res)));
            };
            Ok(())
        }
        pub fn delete(conn: &Connection, user_id: i64) -> Result<(), Error> {
            delete(app_user::table)
                .filter(app_user::id.eq(user_id).and(app_user::active.eq(false)))
                .execute(conn)?;
            Ok(())
        }
        pub fn find_by_id(conn: &Connection, id: i64) -> Result<Option<UserRecord>, Error> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::std::fmt::Arguments::new_v1(
                            &["Finding user by id "],
                            &match (&id,) {
                                (arg0,) => {
                                    [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)]
                                }
                            },
                        ),
                        lvl,
                        &(
                            "ara_model::core::user",
                            "ara_model::core::user",
                            "ara-model/src/core/user.rs",
                            235u32,
                        ),
                    );
                }
            };
            let res = app_user::table.find(id).first(conn).optional()?;
            Ok(res)
        }
        pub fn validate_username(conn: &Connection, username_i: &str) -> Result<(), Error> {
            use crate::schema::app_user::dsl::*;
            use diesel::expression::dsl::exists;
            use diesel::select;
            let exists: bool =
                select(exists(app_user.filter(username.eq(username_i)))).get_result(conn)?;
            if !(!exists) {
                return Err(::failure::err_msg(ValidationError::new(
                    "Email already exists in DB",
                )));
            };
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
    pub struct Insert<'a, T> {
        audit_user: &'a str,
        record: &'a T,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a, T: ::std::fmt::Debug> ::std::fmt::Debug for Insert<'a, T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Insert {
                    audit_user: ref __self_0_0,
                    record: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Insert");
                    let _ = debug_trait_builder.field("audit_user", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("record", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a, T: ::std::clone::Clone> ::std::clone::Clone for Insert<'a, T> {
        #[inline]
        fn clone(&self) -> Insert<'a, T> {
            match *self {
                Insert {
                    audit_user: ref __self_0_0,
                    record: ref __self_0_1,
                } => Insert {
                    audit_user: ::std::clone::Clone::clone(&(*__self_0_0)),
                    record: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_as_changeset_for_insert() {
        extern crate std;
        use diesel;
        use diesel::prelude::*;
        use diesel::query_builder::AsChangeset;
        impl<'a, 'update, T> AsChangeset for &'update Insert<'a, T> {
            type Target = inserts::table;
            type Changeset = <(
                diesel::dsl::Eq<inserts::audit_user, &'update &'a str>,
                diesel::dsl::Eq<inserts::record, &'update &'a T>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    inserts::audit_user.eq(&self.audit_user),
                    inserts::record.eq(&self.record),
                )
                    .as_changeset()
            }
        }
        impl<'a, 'update, T> AsChangeset for Insert<'a, T> {
            type Target = inserts::table;
            type Changeset = <(
                diesel::dsl::Eq<inserts::audit_user, &'a str>,
                diesel::dsl::Eq<inserts::record, &'a T>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    inserts::audit_user.eq(self.audit_user),
                    inserts::record.eq(self.record),
                )
                    .as_changeset()
            }
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
    impl<'a, 'b: 'a> From<&'b WithAudit<'a, NewUser<'a>>> for NewUserRecord<'a> {
        fn from(n: &'b WithAudit<'a, NewUser<'a>>) -> Self {
            NewUserRecord {
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
}
mod user {
    use crate::core::role::RoleRecord;
    use crate::db::Connection;
    use crate::error::DbError;
    use crate::schema::{app_user, user_credential, user_role};
    use chrono::{DateTime, Utc};
    use diesel::associations::HasTable;
    use diesel::prelude::*;
    use diesel::query_builder::AsChangeset;
    use diesel::{delete, insert_into, update};
    use failure::{ensure, Error};
    use serde::{Deserialize, Serialize};
    use validator::{Validate, ValidationError};
    #[table_name = "app_user"]
    #[serde(rename_all = "camelCase")]
    #[structural_match]
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
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_queryable_for_userrecord() {
        extern crate std;
        use diesel;
        use diesel::Queryable;
        impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for UserRecord
        where
            (
                i64,
                String,
                String,
                String,
                String,
                Option<String>,
                bool,
                DateTime<Utc>,
                String,
                DateTime<Utc>,
                String,
                i32,
            ): Queryable<__ST, __DB>,
        {
            type Row = <(
                i64,
                String,
                String,
                String,
                String,
                Option<String>,
                bool,
                DateTime<Utc>,
                String,
                DateTime<Utc>,
                String,
                i32,
            ) as Queryable<__ST, __DB>>::Row;
            fn build(row: Self::Row) -> Self {
                let row: (
                    i64,
                    String,
                    String,
                    String,
                    String,
                    Option<String>,
                    bool,
                    DateTime<Utc>,
                    String,
                    DateTime<Utc>,
                    String,
                    i32,
                ) = Queryable::build(row);
                Self {
                    id: (row.0.into()),
                    first_name: (row.1.into()),
                    last_name: (row.2.into()),
                    username: (row.3.into()),
                    email: (row.4.into()),
                    phone_number: (row.5.into()),
                    active: (row.6.into()),
                    created_at: (row.7.into()),
                    created_by: (row.8.into()),
                    updated_at: (row.9.into()),
                    updated_by: (row.10.into()),
                    version: (row.11.into()),
                }
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_identifiable_for_userrecord() {
        extern crate std;
        use diesel;
        use diesel::associations::{HasTable, Identifiable};
        impl HasTable for UserRecord {
            type Table = app_user::table;
            fn table() -> Self::Table {
                app_user::table
            }
        }
        impl<'ident> Identifiable for &'ident UserRecord {
            type Id = (&'ident i64);
            fn id(self) -> Self::Id {
                (&self.id)
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_as_changeset_for_userrecord() {
        extern crate std;
        use diesel;
        use diesel::prelude::*;
        use diesel::query_builder::AsChangeset;
        impl<'update> AsChangeset for &'update UserRecord {
            type Target = app_user::table;
            type Changeset = <(
                diesel::dsl::Eq<app_user::first_name, &'update String>,
                diesel::dsl::Eq<app_user::last_name, &'update String>,
                diesel::dsl::Eq<app_user::username, &'update String>,
                diesel::dsl::Eq<app_user::email, &'update String>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, &'update String>>,
                diesel::dsl::Eq<app_user::active, &'update bool>,
                diesel::dsl::Eq<app_user::created_at, &'update DateTime<Utc>>,
                diesel::dsl::Eq<app_user::created_by, &'update String>,
                diesel::dsl::Eq<app_user::updated_at, &'update DateTime<Utc>>,
                diesel::dsl::Eq<app_user::updated_by, &'update String>,
                diesel::dsl::Eq<app_user::version, &'update i32>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    app_user::first_name.eq(&self.first_name),
                    app_user::last_name.eq(&self.last_name),
                    app_user::username.eq(&self.username),
                    app_user::email.eq(&self.email),
                    self.phone_number
                        .as_ref()
                        .map(|x| app_user::phone_number.eq(x)),
                    app_user::active.eq(&self.active),
                    app_user::created_at.eq(&self.created_at),
                    app_user::created_by.eq(&self.created_by),
                    app_user::updated_at.eq(&self.updated_at),
                    app_user::updated_by.eq(&self.updated_by),
                    app_user::version.eq(&self.version),
                )
                    .as_changeset()
            }
        }
        impl<'update> AsChangeset for UserRecord {
            type Target = app_user::table;
            type Changeset = <(
                diesel::dsl::Eq<app_user::first_name, String>,
                diesel::dsl::Eq<app_user::last_name, String>,
                diesel::dsl::Eq<app_user::username, String>,
                diesel::dsl::Eq<app_user::email, String>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, String>>,
                diesel::dsl::Eq<app_user::active, bool>,
                diesel::dsl::Eq<app_user::created_at, DateTime<Utc>>,
                diesel::dsl::Eq<app_user::created_by, String>,
                diesel::dsl::Eq<app_user::updated_at, DateTime<Utc>>,
                diesel::dsl::Eq<app_user::updated_by, String>,
                diesel::dsl::Eq<app_user::version, i32>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    app_user::first_name.eq(self.first_name),
                    app_user::last_name.eq(self.last_name),
                    app_user::username.eq(self.username),
                    app_user::email.eq(self.email),
                    self.phone_number.map(|x| app_user::phone_number.eq(x)),
                    app_user::active.eq(self.active),
                    app_user::created_at.eq(self.created_at),
                    app_user::created_by.eq(self.created_by),
                    app_user::updated_at.eq(self.updated_at),
                    app_user::updated_by.eq(self.updated_by),
                    app_user::version.eq(self.version),
                )
                    .as_changeset()
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_associations_for_userrecord() {
        extern crate std;
        use diesel;
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UserRecord {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UserRecord {
                    id: ref __self_0_0,
                    first_name: ref __self_0_1,
                    last_name: ref __self_0_2,
                    username: ref __self_0_3,
                    email: ref __self_0_4,
                    phone_number: ref __self_0_5,
                    active: ref __self_0_6,
                    created_at: ref __self_0_7,
                    created_by: ref __self_0_8,
                    updated_at: ref __self_0_9,
                    updated_by: ref __self_0_10,
                    version: ref __self_0_11,
                } => {
                    let mut debug_trait_builder = f.debug_struct("UserRecord");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("username", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("email", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("phone_number", &&(*__self_0_5));
                    let _ = debug_trait_builder.field("active", &&(*__self_0_6));
                    let _ = debug_trait_builder.field("created_at", &&(*__self_0_7));
                    let _ = debug_trait_builder.field("created_by", &&(*__self_0_8));
                    let _ = debug_trait_builder.field("updated_at", &&(*__self_0_9));
                    let _ = debug_trait_builder.field("updated_by", &&(*__self_0_10));
                    let _ = debug_trait_builder.field("version", &&(*__self_0_11));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UserRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UserRecord {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UserRecord",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "username",
                    &self.username,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "email",
                    &self.email,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phoneNumber",
                    &self.phone_number,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "active",
                    &self.active,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "createdAt",
                    &self.created_at,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "createdBy",
                    &self.created_by,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updatedAt",
                    &self.updated_at,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updatedBy",
                    &self.updated_by,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UserRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UserRecord {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            8u64 => _serde::export::Ok(__Field::__field8),
                            9u64 => _serde::export::Ok(__Field::__field9),
                            10u64 => _serde::export::Ok(__Field::__field10),
                            11u64 => _serde::export::Ok(__Field::__field11),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 12",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::export::Ok(__Field::__field0),
                            "firstName" => _serde::export::Ok(__Field::__field1),
                            "lastName" => _serde::export::Ok(__Field::__field2),
                            "username" => _serde::export::Ok(__Field::__field3),
                            "email" => _serde::export::Ok(__Field::__field4),
                            "phoneNumber" => _serde::export::Ok(__Field::__field5),
                            "active" => _serde::export::Ok(__Field::__field6),
                            "createdAt" => _serde::export::Ok(__Field::__field7),
                            "createdBy" => _serde::export::Ok(__Field::__field8),
                            "updatedAt" => _serde::export::Ok(__Field::__field9),
                            "updatedBy" => _serde::export::Ok(__Field::__field10),
                            "version" => _serde::export::Ok(__Field::__field11),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::export::Ok(__Field::__field0),
                            b"firstName" => _serde::export::Ok(__Field::__field1),
                            b"lastName" => _serde::export::Ok(__Field::__field2),
                            b"username" => _serde::export::Ok(__Field::__field3),
                            b"email" => _serde::export::Ok(__Field::__field4),
                            b"phoneNumber" => _serde::export::Ok(__Field::__field5),
                            b"active" => _serde::export::Ok(__Field::__field6),
                            b"createdAt" => _serde::export::Ok(__Field::__field7),
                            b"createdBy" => _serde::export::Ok(__Field::__field8),
                            b"updatedAt" => _serde::export::Ok(__Field::__field9),
                            b"updatedBy" => _serde::export::Ok(__Field::__field10),
                            b"version" => _serde::export::Ok(__Field::__field11),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UserRecord>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UserRecord;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct UserRecord")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct UserRecord with 12 elements",
                                ));
                            }
                        };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field7 = match match _serde::de::SeqAccess::next_element::<
                            DateTime<Utc>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    7usize,
                                    &"struct UserRecord with 12 elements",
                                ));
                            }
                        };
                        let __field8 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        8usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field9 = match match _serde::de::SeqAccess::next_element::<
                            DateTime<Utc>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    9usize,
                                    &"struct UserRecord with 12 elements",
                                ));
                            }
                        };
                        let __field10 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        10usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        let __field11 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        11usize,
                                        &"struct UserRecord with 12 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(UserRecord {
                            id: __field0,
                            first_name: __field1,
                            last_name: __field2,
                            username: __field3,
                            email: __field4,
                            phone_number: __field5,
                            active: __field6,
                            created_at: __field7,
                            created_by: __field8,
                            updated_at: __field9,
                            updated_by: __field10,
                            version: __field11,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<i64> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<String> = _serde::export::None;
                        let mut __field3: _serde::export::Option<String> = _serde::export::None;
                        let mut __field4: _serde::export::Option<String> = _serde::export::None;
                        let mut __field5: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field6: _serde::export::Option<bool> = _serde::export::None;
                        let mut __field7: _serde::export::Option<DateTime<Utc>> =
                            _serde::export::None;
                        let mut __field8: _serde::export::Option<String> = _serde::export::None;
                        let mut __field9: _serde::export::Option<DateTime<Utc>> =
                            _serde::export::None;
                        let mut __field10: _serde::export::Option<String> = _serde::export::None;
                        let mut __field11: _serde::export::Option<i32> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "username",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "email",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "phoneNumber",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "active",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "createdAt",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<DateTime<Utc>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::export::Option::is_some(&__field8) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "createdBy",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::export::Option::is_some(&__field9) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "updatedAt",
                                            ),
                                        );
                                    }
                                    __field9 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<DateTime<Utc>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field10 => {
                                    if _serde::export::Option::is_some(&__field10) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "updatedBy",
                                            ),
                                        );
                                    }
                                    __field10 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field11 => {
                                    if _serde::export::Option::is_some(&__field11) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "version",
                                            ),
                                        );
                                    }
                                    __field11 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("id") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("username") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("email") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("phoneNumber") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::export::Some(__field6) => __field6,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("active") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::export::Some(__field7) => __field7,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("createdAt") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::export::Some(__field8) => __field8,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("createdBy") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::export::Some(__field9) => __field9,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("updatedAt") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field10 = match __field10 {
                            _serde::export::Some(__field10) => __field10,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("updatedBy") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field11 = match __field11 {
                            _serde::export::Some(__field11) => __field11,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("version") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(UserRecord {
                            id: __field0,
                            first_name: __field1,
                            last_name: __field2,
                            username: __field3,
                            email: __field4,
                            phone_number: __field5,
                            active: __field6,
                            created_at: __field7,
                            created_by: __field8,
                            updated_at: __field9,
                            updated_by: __field10,
                            version: __field11,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "firstName",
                    "lastName",
                    "username",
                    "email",
                    "phoneNumber",
                    "active",
                    "createdAt",
                    "createdBy",
                    "updatedAt",
                    "updatedBy",
                    "version",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UserRecord",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<UserRecord>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UserRecord {
        #[inline]
        fn clone(&self) -> UserRecord {
            match *self {
                UserRecord {
                    id: ref __self_0_0,
                    first_name: ref __self_0_1,
                    last_name: ref __self_0_2,
                    username: ref __self_0_3,
                    email: ref __self_0_4,
                    phone_number: ref __self_0_5,
                    active: ref __self_0_6,
                    created_at: ref __self_0_7,
                    created_by: ref __self_0_8,
                    updated_at: ref __self_0_9,
                    updated_by: ref __self_0_10,
                    version: ref __self_0_11,
                } => UserRecord {
                    id: ::std::clone::Clone::clone(&(*__self_0_0)),
                    first_name: ::std::clone::Clone::clone(&(*__self_0_1)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_2)),
                    username: ::std::clone::Clone::clone(&(*__self_0_3)),
                    email: ::std::clone::Clone::clone(&(*__self_0_4)),
                    phone_number: ::std::clone::Clone::clone(&(*__self_0_5)),
                    active: ::std::clone::Clone::clone(&(*__self_0_6)),
                    created_at: ::std::clone::Clone::clone(&(*__self_0_7)),
                    created_by: ::std::clone::Clone::clone(&(*__self_0_8)),
                    updated_at: ::std::clone::Clone::clone(&(*__self_0_9)),
                    updated_by: ::std::clone::Clone::clone(&(*__self_0_10)),
                    version: ::std::clone::Clone::clone(&(*__self_0_11)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for UserRecord {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<i64>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<Option<String>>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
                let _: ::std::cmp::AssertParamIsEq<DateTime<Utc>>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<DateTime<Utc>>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<i32>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for UserRecord {
        #[inline]
        fn eq(&self, other: &UserRecord) -> bool {
            match *other {
                UserRecord {
                    id: ref __self_1_0,
                    first_name: ref __self_1_1,
                    last_name: ref __self_1_2,
                    username: ref __self_1_3,
                    email: ref __self_1_4,
                    phone_number: ref __self_1_5,
                    active: ref __self_1_6,
                    created_at: ref __self_1_7,
                    created_by: ref __self_1_8,
                    updated_at: ref __self_1_9,
                    updated_by: ref __self_1_10,
                    version: ref __self_1_11,
                } => match *self {
                    UserRecord {
                        id: ref __self_0_0,
                        first_name: ref __self_0_1,
                        last_name: ref __self_0_2,
                        username: ref __self_0_3,
                        email: ref __self_0_4,
                        phone_number: ref __self_0_5,
                        active: ref __self_0_6,
                        created_at: ref __self_0_7,
                        created_by: ref __self_0_8,
                        updated_at: ref __self_0_9,
                        updated_by: ref __self_0_10,
                        version: ref __self_0_11,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                            && (*__self_0_6) == (*__self_1_6)
                            && (*__self_0_7) == (*__self_1_7)
                            && (*__self_0_8) == (*__self_1_8)
                            && (*__self_0_9) == (*__self_1_9)
                            && (*__self_0_10) == (*__self_1_10)
                            && (*__self_0_11) == (*__self_1_11)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &UserRecord) -> bool {
            match *other {
                UserRecord {
                    id: ref __self_1_0,
                    first_name: ref __self_1_1,
                    last_name: ref __self_1_2,
                    username: ref __self_1_3,
                    email: ref __self_1_4,
                    phone_number: ref __self_1_5,
                    active: ref __self_1_6,
                    created_at: ref __self_1_7,
                    created_by: ref __self_1_8,
                    updated_at: ref __self_1_9,
                    updated_by: ref __self_1_10,
                    version: ref __self_1_11,
                } => match *self {
                    UserRecord {
                        id: ref __self_0_0,
                        first_name: ref __self_0_1,
                        last_name: ref __self_0_2,
                        username: ref __self_0_3,
                        email: ref __self_0_4,
                        phone_number: ref __self_0_5,
                        active: ref __self_0_6,
                        created_at: ref __self_0_7,
                        created_by: ref __self_0_8,
                        updated_at: ref __self_0_9,
                        updated_by: ref __self_0_10,
                        version: ref __self_0_11,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                            || (*__self_0_6) != (*__self_1_6)
                            || (*__self_0_7) != (*__self_1_7)
                            || (*__self_0_8) != (*__self_1_8)
                            || (*__self_0_9) != (*__self_1_9)
                            || (*__self_0_10) != (*__self_1_10)
                            || (*__self_0_11) != (*__self_1_11)
                    }
                },
            }
        }
    }
    impl Validate for UserRecord {
        #[allow(unused_mut)]
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            let mut errors = ::validator::ValidationErrors::new();
            if !::validator::validate_length(
                ::validator::Validator::Length {
                    min: ::std::option::Option::Some(2u64),
                    max: ::std::option::Option::None,
                    equal: ::std::option::Option::None,
                },
                &self.username,
            ) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &2u64);
                err.add_param(::std::borrow::Cow::from("value"), &&self.username);
                errors.add("username", err);
            }
            if !::validator::validate_email(&self.email) {
                let mut err = ::validator::ValidationError::new("email");
                err.add_param(::std::borrow::Cow::from("value"), &&self.email);
                errors.add("email", err);
            }
            let mut result = if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            };
            result
        }
    }
    #[table_name = "app_user"]
    #[serde(rename_all = "camelCase")]
    #[structural_match]
    pub struct UserUpdate {
        pub id: i64,
        pub first_name: String,
        pub last_name: String,
        pub phone_number: Option<String>,
        pub active: bool,
        pub version: i32,
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_queryable_for_userupdate() {
        extern crate std;
        use diesel;
        use diesel::Queryable;
        impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for UserUpdate
        where
            (i64, String, String, Option<String>, bool, i32): Queryable<__ST, __DB>,
        {
            type Row =
                <(i64, String, String, Option<String>, bool, i32) as Queryable<__ST, __DB>>::Row;
            fn build(row: Self::Row) -> Self {
                let row: (i64, String, String, Option<String>, bool, i32) = Queryable::build(row);
                Self {
                    id: (row.0.into()),
                    first_name: (row.1.into()),
                    last_name: (row.2.into()),
                    phone_number: (row.3.into()),
                    active: (row.4.into()),
                    version: (row.5.into()),
                }
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_identifiable_for_userupdate() {
        extern crate std;
        use diesel;
        use diesel::associations::{HasTable, Identifiable};
        impl HasTable for UserUpdate {
            type Table = app_user::table;
            fn table() -> Self::Table {
                app_user::table
            }
        }
        impl<'ident> Identifiable for &'ident UserUpdate {
            type Id = (&'ident i64);
            fn id(self) -> Self::Id {
                (&self.id)
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_as_changeset_for_userupdate() {
        extern crate std;
        use diesel;
        use diesel::prelude::*;
        use diesel::query_builder::AsChangeset;
        impl<'update> AsChangeset for &'update UserUpdate {
            type Target = app_user::table;
            type Changeset = <(
                diesel::dsl::Eq<app_user::first_name, &'update String>,
                diesel::dsl::Eq<app_user::last_name, &'update String>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, &'update String>>,
                diesel::dsl::Eq<app_user::active, &'update bool>,
                diesel::dsl::Eq<app_user::version, &'update i32>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    app_user::first_name.eq(&self.first_name),
                    app_user::last_name.eq(&self.last_name),
                    self.phone_number
                        .as_ref()
                        .map(|x| app_user::phone_number.eq(x)),
                    app_user::active.eq(&self.active),
                    app_user::version.eq(&self.version),
                )
                    .as_changeset()
            }
        }
        impl<'update> AsChangeset for UserUpdate {
            type Target = app_user::table;
            type Changeset = <(
                diesel::dsl::Eq<app_user::first_name, String>,
                diesel::dsl::Eq<app_user::last_name, String>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, String>>,
                diesel::dsl::Eq<app_user::active, bool>,
                diesel::dsl::Eq<app_user::version, i32>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    app_user::first_name.eq(self.first_name),
                    app_user::last_name.eq(self.last_name),
                    self.phone_number.map(|x| app_user::phone_number.eq(x)),
                    app_user::active.eq(self.active),
                    app_user::version.eq(self.version),
                )
                    .as_changeset()
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_associations_for_userupdate() {
        extern crate std;
        use diesel;
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UserUpdate {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UserUpdate {
                    id: ref __self_0_0,
                    first_name: ref __self_0_1,
                    last_name: ref __self_0_2,
                    phone_number: ref __self_0_3,
                    active: ref __self_0_4,
                    version: ref __self_0_5,
                } => {
                    let mut debug_trait_builder = f.debug_struct("UserUpdate");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("phone_number", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("active", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("version", &&(*__self_0_5));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UserUpdate: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UserUpdate {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UserUpdate",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phoneNumber",
                    &self.phone_number,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "active",
                    &self.active,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UserUpdate: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UserUpdate {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 6",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::export::Ok(__Field::__field0),
                            "firstName" => _serde::export::Ok(__Field::__field1),
                            "lastName" => _serde::export::Ok(__Field::__field2),
                            "phoneNumber" => _serde::export::Ok(__Field::__field3),
                            "active" => _serde::export::Ok(__Field::__field4),
                            "version" => _serde::export::Ok(__Field::__field5),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::export::Ok(__Field::__field0),
                            b"firstName" => _serde::export::Ok(__Field::__field1),
                            b"lastName" => _serde::export::Ok(__Field::__field2),
                            b"phoneNumber" => _serde::export::Ok(__Field::__field3),
                            b"active" => _serde::export::Ok(__Field::__field4),
                            b"version" => _serde::export::Ok(__Field::__field5),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UserUpdate>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UserUpdate;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct UserUpdate")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UserUpdate with 6 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UserUpdate with 6 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UserUpdate with 6 elements",
                                    ));
                                }
                            };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct UserUpdate with 6 elements",
                                ));
                            }
                        };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct UserUpdate with 6 elements",
                                    ));
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct UserUpdate with 6 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(UserUpdate {
                            id: __field0,
                            first_name: __field1,
                            last_name: __field2,
                            phone_number: __field3,
                            active: __field4,
                            version: __field5,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<i64> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<String> = _serde::export::None;
                        let mut __field3: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field4: _serde::export::Option<bool> = _serde::export::None;
                        let mut __field5: _serde::export::Option<i32> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "phoneNumber",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "active",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "version",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("id") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("phoneNumber") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("active") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("version") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(UserUpdate {
                            id: __field0,
                            first_name: __field1,
                            last_name: __field2,
                            phone_number: __field3,
                            active: __field4,
                            version: __field5,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "firstName",
                    "lastName",
                    "phoneNumber",
                    "active",
                    "version",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UserUpdate",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<UserUpdate>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UserUpdate {
        #[inline]
        fn clone(&self) -> UserUpdate {
            match *self {
                UserUpdate {
                    id: ref __self_0_0,
                    first_name: ref __self_0_1,
                    last_name: ref __self_0_2,
                    phone_number: ref __self_0_3,
                    active: ref __self_0_4,
                    version: ref __self_0_5,
                } => UserUpdate {
                    id: ::std::clone::Clone::clone(&(*__self_0_0)),
                    first_name: ::std::clone::Clone::clone(&(*__self_0_1)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_2)),
                    phone_number: ::std::clone::Clone::clone(&(*__self_0_3)),
                    active: ::std::clone::Clone::clone(&(*__self_0_4)),
                    version: ::std::clone::Clone::clone(&(*__self_0_5)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for UserUpdate {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<i64>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<Option<String>>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
                let _: ::std::cmp::AssertParamIsEq<i32>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for UserUpdate {
        #[inline]
        fn eq(&self, other: &UserUpdate) -> bool {
            match *other {
                UserUpdate {
                    id: ref __self_1_0,
                    first_name: ref __self_1_1,
                    last_name: ref __self_1_2,
                    phone_number: ref __self_1_3,
                    active: ref __self_1_4,
                    version: ref __self_1_5,
                } => match *self {
                    UserUpdate {
                        id: ref __self_0_0,
                        first_name: ref __self_0_1,
                        last_name: ref __self_0_2,
                        phone_number: ref __self_0_3,
                        active: ref __self_0_4,
                        version: ref __self_0_5,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &UserUpdate) -> bool {
            match *other {
                UserUpdate {
                    id: ref __self_1_0,
                    first_name: ref __self_1_1,
                    last_name: ref __self_1_2,
                    phone_number: ref __self_1_3,
                    active: ref __self_1_4,
                    version: ref __self_1_5,
                } => match *self {
                    UserUpdate {
                        id: ref __self_0_0,
                        first_name: ref __self_0_1,
                        last_name: ref __self_0_2,
                        phone_number: ref __self_0_3,
                        active: ref __self_0_4,
                        version: ref __self_0_5,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                    }
                },
            }
        }
    }
    impl Validate for UserUpdate {
        #[allow(unused_mut)]
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            let mut errors = ::validator::ValidationErrors::new();
            let mut result = if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            };
            result
        }
    }
    #[serde(rename_all = "camelCase")]
    #[structural_match]
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
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for User {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                User {
                    id: ref __self_0_0,
                    username: ref __self_0_1,
                    first_name: ref __self_0_2,
                    last_name: ref __self_0_3,
                    email: ref __self_0_4,
                    phone_number: ref __self_0_5,
                    active: ref __self_0_6,
                    version: ref __self_0_7,
                } => {
                    let mut debug_trait_builder = f.debug_struct("User");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("username", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("email", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("phone_number", &&(*__self_0_5));
                    let _ = debug_trait_builder.field("active", &&(*__self_0_6));
                    let _ = debug_trait_builder.field("version", &&(*__self_0_7));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_User: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for User {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "User",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "username",
                    &self.username,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "email",
                    &self.email,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phoneNumber",
                    &self.phone_number,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "active",
                    &self.active,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_User: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for User {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 8",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::export::Ok(__Field::__field0),
                            "username" => _serde::export::Ok(__Field::__field1),
                            "firstName" => _serde::export::Ok(__Field::__field2),
                            "lastName" => _serde::export::Ok(__Field::__field3),
                            "email" => _serde::export::Ok(__Field::__field4),
                            "phoneNumber" => _serde::export::Ok(__Field::__field5),
                            "active" => _serde::export::Ok(__Field::__field6),
                            "version" => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::export::Ok(__Field::__field0),
                            b"username" => _serde::export::Ok(__Field::__field1),
                            b"firstName" => _serde::export::Ok(__Field::__field2),
                            b"lastName" => _serde::export::Ok(__Field::__field3),
                            b"email" => _serde::export::Ok(__Field::__field4),
                            b"phoneNumber" => _serde::export::Ok(__Field::__field5),
                            b"active" => _serde::export::Ok(__Field::__field6),
                            b"version" => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<User>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = User;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct User")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct User with 8 elements",
                                ));
                            }
                        };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        let __field7 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct User with 8 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(User {
                            id: __field0,
                            username: __field1,
                            first_name: __field2,
                            last_name: __field3,
                            email: __field4,
                            phone_number: __field5,
                            active: __field6,
                            version: __field7,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<i64> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<String> = _serde::export::None;
                        let mut __field3: _serde::export::Option<String> = _serde::export::None;
                        let mut __field4: _serde::export::Option<String> = _serde::export::None;
                        let mut __field5: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field6: _serde::export::Option<bool> = _serde::export::None;
                        let mut __field7: _serde::export::Option<i32> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "username",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "email",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "phoneNumber",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "active",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "version",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("id") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("username") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("email") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("phoneNumber") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::export::Some(__field6) => __field6,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("active") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::export::Some(__field7) => __field7,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("version") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(User {
                            id: __field0,
                            username: __field1,
                            first_name: __field2,
                            last_name: __field3,
                            email: __field4,
                            phone_number: __field5,
                            active: __field6,
                            version: __field7,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "username",
                    "firstName",
                    "lastName",
                    "email",
                    "phoneNumber",
                    "active",
                    "version",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "User",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<User>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for User {
        #[inline]
        fn clone(&self) -> User {
            match *self {
                User {
                    id: ref __self_0_0,
                    username: ref __self_0_1,
                    first_name: ref __self_0_2,
                    last_name: ref __self_0_3,
                    email: ref __self_0_4,
                    phone_number: ref __self_0_5,
                    active: ref __self_0_6,
                    version: ref __self_0_7,
                } => User {
                    id: ::std::clone::Clone::clone(&(*__self_0_0)),
                    username: ::std::clone::Clone::clone(&(*__self_0_1)),
                    first_name: ::std::clone::Clone::clone(&(*__self_0_2)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_3)),
                    email: ::std::clone::Clone::clone(&(*__self_0_4)),
                    phone_number: ::std::clone::Clone::clone(&(*__self_0_5)),
                    active: ::std::clone::Clone::clone(&(*__self_0_6)),
                    version: ::std::clone::Clone::clone(&(*__self_0_7)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for User {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<i64>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<String>;
                let _: ::std::cmp::AssertParamIsEq<Option<String>>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
                let _: ::std::cmp::AssertParamIsEq<i32>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for User {
        #[inline]
        fn eq(&self, other: &User) -> bool {
            match *other {
                User {
                    id: ref __self_1_0,
                    username: ref __self_1_1,
                    first_name: ref __self_1_2,
                    last_name: ref __self_1_3,
                    email: ref __self_1_4,
                    phone_number: ref __self_1_5,
                    active: ref __self_1_6,
                    version: ref __self_1_7,
                } => match *self {
                    User {
                        id: ref __self_0_0,
                        username: ref __self_0_1,
                        first_name: ref __self_0_2,
                        last_name: ref __self_0_3,
                        email: ref __self_0_4,
                        phone_number: ref __self_0_5,
                        active: ref __self_0_6,
                        version: ref __self_0_7,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                            && (*__self_0_6) == (*__self_1_6)
                            && (*__self_0_7) == (*__self_1_7)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &User) -> bool {
            match *other {
                User {
                    id: ref __self_1_0,
                    username: ref __self_1_1,
                    first_name: ref __self_1_2,
                    last_name: ref __self_1_3,
                    email: ref __self_1_4,
                    phone_number: ref __self_1_5,
                    active: ref __self_1_6,
                    version: ref __self_1_7,
                } => match *self {
                    User {
                        id: ref __self_0_0,
                        username: ref __self_0_1,
                        first_name: ref __self_0_2,
                        last_name: ref __self_0_3,
                        email: ref __self_0_4,
                        phone_number: ref __self_0_5,
                        active: ref __self_0_6,
                        version: ref __self_0_7,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                            || (*__self_0_6) != (*__self_1_6)
                            || (*__self_0_7) != (*__self_1_7)
                    }
                },
            }
        }
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
    #[table_name = "app_user"]
    #[serde(rename_all = "camelCase")]
    pub struct NewUserRecord<'a> {
        pub first_name: &'a str,
        pub last_name: &'a str,
        pub username: &'a str,
        pub email: &'a str,
        pub phone_number: Option<&'a str>,
        pub created_by: &'a str,
        pub updated_by: &'a str,
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_insertable_for_newuserrecord() {
        extern crate std;
        use diesel;
        use diesel::insertable::Insertable;
        use diesel::prelude::*;
        use diesel::query_builder::UndecoratedInsertRecord;
        impl<'a, 'insert> Insertable<app_user::table> for NewUserRecord<'a> {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<app_user::first_name, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::last_name, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::username, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::email, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::created_by, &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::updated_by, &'a str>>,
            ) as Insertable<app_user::table>>::Values;
            fn values(self) -> Self::Values {
                (
                    std::option::Option::Some(app_user::first_name.eq(self.first_name)),
                    std::option::Option::Some(app_user::last_name.eq(self.last_name)),
                    std::option::Option::Some(app_user::username.eq(self.username)),
                    std::option::Option::Some(app_user::email.eq(self.email)),
                    self.phone_number.map(|x| app_user::phone_number.eq(x)),
                    std::option::Option::Some(app_user::created_by.eq(self.created_by)),
                    std::option::Option::Some(app_user::updated_by.eq(self.updated_by)),
                )
                    .values()
            }
        }
        impl<'a, 'insert> Insertable<app_user::table> for &'insert NewUserRecord<'a> {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<app_user::first_name, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::last_name, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::username, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::email, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::phone_number, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::created_by, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<app_user::updated_by, &'insert &'a str>>,
            ) as Insertable<app_user::table>>::Values;
            fn values(self) -> Self::Values {
                (
                    std::option::Option::Some(app_user::first_name.eq(&self.first_name)),
                    std::option::Option::Some(app_user::last_name.eq(&self.last_name)),
                    std::option::Option::Some(app_user::username.eq(&self.username)),
                    std::option::Option::Some(app_user::email.eq(&self.email)),
                    self.phone_number
                        .as_ref()
                        .map(|x| app_user::phone_number.eq(x)),
                    std::option::Option::Some(app_user::created_by.eq(&self.created_by)),
                    std::option::Option::Some(app_user::updated_by.eq(&self.updated_by)),
                )
                    .values()
            }
        }
        impl<'a, 'insert> UndecoratedInsertRecord<app_user::table> for NewUserRecord<'a> {}
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::fmt::Debug for NewUserRecord<'a> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                NewUserRecord {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    username: ref __self_0_2,
                    email: ref __self_0_3,
                    phone_number: ref __self_0_4,
                    created_by: ref __self_0_5,
                    updated_by: ref __self_0_6,
                } => {
                    let mut debug_trait_builder = f.debug_struct("NewUserRecord");
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("username", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("email", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("phone_number", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("created_by", &&(*__self_0_5));
                    let _ = debug_trait_builder.field("updated_by", &&(*__self_0_6));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_NewUserRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'a> _serde::Serialize for NewUserRecord<'a> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "NewUserRecord",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "username",
                    &self.username,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "email",
                    &self.email,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phoneNumber",
                    &self.phone_number,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "createdBy",
                    &self.created_by,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updatedBy",
                    &self.updated_by,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_NewUserRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de: 'a, 'a> _serde::Deserialize<'de> for NewUserRecord<'a> {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 7",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "firstName" => _serde::export::Ok(__Field::__field0),
                            "lastName" => _serde::export::Ok(__Field::__field1),
                            "username" => _serde::export::Ok(__Field::__field2),
                            "email" => _serde::export::Ok(__Field::__field3),
                            "phoneNumber" => _serde::export::Ok(__Field::__field4),
                            "createdBy" => _serde::export::Ok(__Field::__field5),
                            "updatedBy" => _serde::export::Ok(__Field::__field6),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"firstName" => _serde::export::Ok(__Field::__field0),
                            b"lastName" => _serde::export::Ok(__Field::__field1),
                            b"username" => _serde::export::Ok(__Field::__field2),
                            b"email" => _serde::export::Ok(__Field::__field3),
                            b"phoneNumber" => _serde::export::Ok(__Field::__field4),
                            b"createdBy" => _serde::export::Ok(__Field::__field5),
                            b"updatedBy" => _serde::export::Ok(__Field::__field6),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de: 'a, 'a> {
                    marker: _serde::export::PhantomData<NewUserRecord<'a>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                    type Value = NewUserRecord<'a>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct NewUserRecord")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<&'a str>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct NewUserRecord with 7 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(NewUserRecord {
                            first_name: __field0,
                            last_name: __field1,
                            username: __field2,
                            email: __field3,
                            phone_number: __field4,
                            created_by: __field5,
                            updated_by: __field6,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field1: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field2: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field3: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field4: _serde::export::Option<Option<&'a str>> =
                            _serde::export::None;
                        let mut __field5: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field6: _serde::export::Option<&'a str> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "username",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "email",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "phoneNumber",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<&'a str>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "createdBy",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "updatedBy",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("username") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("email") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("phoneNumber") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("createdBy") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::export::Some(__field6) => __field6,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("updatedBy") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(NewUserRecord {
                            first_name: __field0,
                            last_name: __field1,
                            username: __field2,
                            email: __field3,
                            phone_number: __field4,
                            created_by: __field5,
                            updated_by: __field6,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "firstName",
                    "lastName",
                    "username",
                    "email",
                    "phoneNumber",
                    "createdBy",
                    "updatedBy",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "NewUserRecord",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<NewUserRecord<'a>>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::clone::Clone for NewUserRecord<'a> {
        #[inline]
        fn clone(&self) -> NewUserRecord<'a> {
            match *self {
                NewUserRecord {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    username: ref __self_0_2,
                    email: ref __self_0_3,
                    phone_number: ref __self_0_4,
                    created_by: ref __self_0_5,
                    updated_by: ref __self_0_6,
                } => NewUserRecord {
                    first_name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_1)),
                    username: ::std::clone::Clone::clone(&(*__self_0_2)),
                    email: ::std::clone::Clone::clone(&(*__self_0_3)),
                    phone_number: ::std::clone::Clone::clone(&(*__self_0_4)),
                    created_by: ::std::clone::Clone::clone(&(*__self_0_5)),
                    updated_by: ::std::clone::Clone::clone(&(*__self_0_6)),
                },
            }
        }
    }
    #[serde(rename_all = "camelCase")]
    pub struct NewUser<'a> {
        pub first_name: &'a str,
        pub last_name: &'a str,
        pub email: &'a str,
        pub phone_number: Option<&'a str>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::fmt::Debug for NewUser<'a> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                NewUser {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    email: ref __self_0_2,
                    phone_number: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("NewUser");
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("email", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("phone_number", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_NewUser: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'a> _serde::Serialize for NewUser<'a> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "NewUser",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "email",
                    &self.email,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phoneNumber",
                    &self.phone_number,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_NewUser: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de: 'a, 'a> _serde::Deserialize<'de> for NewUser<'a> {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 4",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "firstName" => _serde::export::Ok(__Field::__field0),
                            "lastName" => _serde::export::Ok(__Field::__field1),
                            "email" => _serde::export::Ok(__Field::__field2),
                            "phoneNumber" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"firstName" => _serde::export::Ok(__Field::__field0),
                            b"lastName" => _serde::export::Ok(__Field::__field1),
                            b"email" => _serde::export::Ok(__Field::__field2),
                            b"phoneNumber" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de: 'a, 'a> {
                    marker: _serde::export::PhantomData<NewUser<'a>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                    type Value = NewUser<'a>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct NewUser")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct NewUser with 4 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct NewUser with 4 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct NewUser with 4 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<&'a str>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct NewUser with 4 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(NewUser {
                            first_name: __field0,
                            last_name: __field1,
                            email: __field2,
                            phone_number: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field1: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field2: _serde::export::Option<&'a str> = _serde::export::None;
                        let mut __field3: _serde::export::Option<Option<&'a str>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "email",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "phoneNumber",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<&'a str>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("email") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("phoneNumber") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(NewUser {
                            first_name: __field0,
                            last_name: __field1,
                            email: __field2,
                            phone_number: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["firstName", "lastName", "email", "phoneNumber"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "NewUser",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<NewUser<'a>>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::std::clone::Clone for NewUser<'a> {
        #[inline]
        fn clone(&self) -> NewUser<'a> {
            match *self {
                NewUser {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    email: ref __self_0_2,
                    phone_number: ref __self_0_3,
                } => NewUser {
                    first_name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_1)),
                    email: ::std::clone::Clone::clone(&(*__self_0_2)),
                    phone_number: ::std::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[serde(rename_all = "camelCase")]
    pub struct UserSignUp {
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub password: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UserSignUp {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UserSignUp {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    email: ref __self_0_2,
                    password: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("UserSignUp");
                    let _ = debug_trait_builder.field("first_name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("last_name", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("email", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("password", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UserSignUp: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UserSignUp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UserSignUp",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "firstName",
                    &self.first_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lastName",
                    &self.last_name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "email",
                    &self.email,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "password",
                    &self.password,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UserSignUp: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UserSignUp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 4",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "firstName" => _serde::export::Ok(__Field::__field0),
                            "lastName" => _serde::export::Ok(__Field::__field1),
                            "email" => _serde::export::Ok(__Field::__field2),
                            "password" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"firstName" => _serde::export::Ok(__Field::__field0),
                            b"lastName" => _serde::export::Ok(__Field::__field1),
                            b"email" => _serde::export::Ok(__Field::__field2),
                            b"password" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UserSignUp>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UserSignUp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct UserSignUp")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UserSignUp with 4 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UserSignUp with 4 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UserSignUp with 4 elements",
                                    ));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct UserSignUp with 4 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(UserSignUp {
                            first_name: __field0,
                            last_name: __field1,
                            email: __field2,
                            password: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<String> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<String> = _serde::export::None;
                        let mut __field3: _serde::export::Option<String> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "firstName",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastName",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "email",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "password",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("firstName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("lastName") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("email") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("password") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(UserSignUp {
                            first_name: __field0,
                            last_name: __field1,
                            email: __field2,
                            password: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["firstName", "lastName", "email", "password"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UserSignUp",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<UserSignUp>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UserSignUp {
        #[inline]
        fn clone(&self) -> UserSignUp {
            match *self {
                UserSignUp {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    email: ref __self_0_2,
                    password: ref __self_0_3,
                } => UserSignUp {
                    first_name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    last_name: ::std::clone::Clone::clone(&(*__self_0_1)),
                    email: ::std::clone::Clone::clone(&(*__self_0_2)),
                    password: ::std::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[serde(rename_all = "camelCase")]
    pub struct UserLogin {
        pub username: String,
        pub password: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UserLogin {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UserLogin {
                    username: ref __self_0_0,
                    password: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("UserLogin");
                    let _ = debug_trait_builder.field("username", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("password", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UserLogin: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UserLogin {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UserLogin",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "username",
                    &self.username,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "password",
                    &self.password,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UserLogin: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UserLogin {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "username" => _serde::export::Ok(__Field::__field0),
                            "password" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"username" => _serde::export::Ok(__Field::__field0),
                            b"password" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UserLogin>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UserLogin;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct UserLogin")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UserLogin with 2 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UserLogin with 2 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(UserLogin {
                            username: __field0,
                            password: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<String> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "username",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "password",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("username") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("password") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(UserLogin {
                            username: __field0,
                            password: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["username", "password"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UserLogin",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<UserLogin>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UserLogin {
        #[inline]
        fn clone(&self) -> UserLogin {
            match *self {
                UserLogin {
                    username: ref __self_0_0,
                    password: ref __self_0_1,
                } => UserLogin {
                    username: ::std::clone::Clone::clone(&(*__self_0_0)),
                    password: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[belongs_to(UserRecord, foreign_key = "user_id")]
    #[belongs_to(RoleRecord, foreign_key = "role_id")]
    #[table_name = "user_role"]
    #[primary_key(user_id, role_id)]
    #[serde(rename_all = "camelCase")]
    pub struct UserRoleRecord {
        pub user_id: i64,
        pub role_id: i32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UserRoleRecord {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UserRoleRecord {
                    user_id: ref __self_0_0,
                    role_id: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("UserRoleRecord");
                    let _ = debug_trait_builder.field("user_id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("role_id", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UserRoleRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UserRoleRecord {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UserRoleRecord",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "userId",
                    &self.user_id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "roleId",
                    &self.role_id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UserRoleRecord: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UserRoleRecord {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "userId" => _serde::export::Ok(__Field::__field0),
                            "roleId" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"userId" => _serde::export::Ok(__Field::__field0),
                            b"roleId" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UserRoleRecord>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UserRoleRecord;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct UserRoleRecord")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UserRoleRecord with 2 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UserRoleRecord with 2 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(UserRoleRecord {
                            user_id: __field0,
                            role_id: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<i64> = _serde::export::None;
                        let mut __field1: _serde::export::Option<i32> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "userId",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "roleId",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("userId") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("roleId") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(UserRoleRecord {
                            user_id: __field0,
                            role_id: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["userId", "roleId"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UserRoleRecord",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<UserRoleRecord>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_identifiable_for_userrolerecord() {
        extern crate std;
        use diesel;
        use diesel::associations::{HasTable, Identifiable};
        impl HasTable for UserRoleRecord {
            type Table = user_role::table;
            fn table() -> Self::Table {
                user_role::table
            }
        }
        impl<'ident> Identifiable for &'ident UserRoleRecord {
            type Id = (&'ident i64, &'ident i32);
            fn id(self) -> Self::Id {
                (&self.user_id, &self.role_id)
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_queryable_for_userrolerecord() {
        extern crate std;
        use diesel;
        use diesel::Queryable;
        impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for UserRoleRecord
        where
            (i64, i32): Queryable<__ST, __DB>,
        {
            type Row = <(i64, i32) as Queryable<__ST, __DB>>::Row;
            fn build(row: Self::Row) -> Self {
                let row: (i64, i32) = Queryable::build(row);
                Self {
                    user_id: (row.0.into()),
                    role_id: (row.1.into()),
                }
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_associations_for_userrolerecord() {
        extern crate std;
        use diesel;
        impl<__FK> diesel::associations::BelongsTo<UserRecord> for UserRoleRecord
        where
            __FK: std::hash::Hash + std::cmp::Eq,
            for<'__a> &'__a i64: std::convert::Into<::std::option::Option<&'__a __FK>>,
            for<'__a> &'__a UserRecord: diesel::associations::Identifiable<Id = &'__a __FK>,
        {
            type ForeignKey = __FK;
            type ForeignKeyColumn = user_role::user_id;
            fn foreign_key(&self) -> std::option::Option<&Self::ForeignKey> {
                std::convert::Into::into(&self.user_id)
            }
            fn foreign_key_column() -> Self::ForeignKeyColumn {
                user_role::user_id
            }
        }
        impl<__FK> diesel::associations::BelongsTo<RoleRecord> for UserRoleRecord
        where
            __FK: std::hash::Hash + std::cmp::Eq,
            for<'__a> &'__a i32: std::convert::Into<::std::option::Option<&'__a __FK>>,
            for<'__a> &'__a RoleRecord: diesel::associations::Identifiable<Id = &'__a __FK>,
        {
            type ForeignKey = __FK;
            type ForeignKeyColumn = user_role::role_id;
            fn foreign_key(&self) -> std::option::Option<&Self::ForeignKey> {
                std::convert::Into::into(&self.role_id)
            }
            fn foreign_key_column() -> Self::ForeignKeyColumn {
                user_role::role_id
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_insertable_for_userrolerecord() {
        extern crate std;
        use diesel;
        use diesel::insertable::Insertable;
        use diesel::prelude::*;
        use diesel::query_builder::UndecoratedInsertRecord;
        impl<'insert> Insertable<user_role::table> for UserRoleRecord {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<user_role::user_id, i64>>,
                std::option::Option<diesel::dsl::Eq<user_role::role_id, i32>>,
            ) as Insertable<user_role::table>>::Values;
            fn values(self) -> Self::Values {
                (
                    std::option::Option::Some(user_role::user_id.eq(self.user_id)),
                    std::option::Option::Some(user_role::role_id.eq(self.role_id)),
                )
                    .values()
            }
        }
        impl<'insert> Insertable<user_role::table> for &'insert UserRoleRecord {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<user_role::user_id, &'insert i64>>,
                std::option::Option<diesel::dsl::Eq<user_role::role_id, &'insert i32>>,
            ) as Insertable<user_role::table>>::Values;
            fn values(self) -> Self::Values {
                (
                    std::option::Option::Some(user_role::user_id.eq(&self.user_id)),
                    std::option::Option::Some(user_role::role_id.eq(&self.role_id)),
                )
                    .values()
            }
        }
        impl<'insert> UndecoratedInsertRecord<user_role::table> for UserRoleRecord {}
    }
    impl User {
        pub fn insert(
            conn: &Connection,
            new_user: &WithAudit<'_, NewUser<'_>>,
        ) -> Result<UserRecord, Error> {
            let res = insert_into(app_user::table)
                .values(NewUserRecord::from(new_user))
                .get_result::<UserRecord>(conn)?;
            Ok(res)
        }
        pub fn update(conn: &Connection, update_user: &UserUpdate) -> Result<UserRecord, Error> {
            use crate::schema::app_user::columns::version;
            let target = app_user::table.filter(
                app_user::id
                    .eq(update_user.id)
                    .and(version.eq(update_user.version)),
            );
            let res = update(target)
                .set(update_user)
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
            if !(res == 1) {
                return Err(::failure::err_msg(DbError::IncorrectResultSize(1, res)));
            };
            Ok(())
        }
        pub fn delete(conn: &Connection, user_id: i64) -> Result<(), Error> {
            delete(app_user::table)
                .filter(app_user::id.eq(user_id).and(app_user::active.eq(false)))
                .execute(conn)?;
            Ok(())
        }
        pub fn find_by_id(conn: &Connection, id: i64) -> Result<Option<UserRecord>, Error> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::std::fmt::Arguments::new_v1(
                            &["Finding user by id "],
                            &match (&id,) {
                                (arg0,) => {
                                    [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)]
                                }
                            },
                        ),
                        lvl,
                        &(
                            "ara_model::core::user",
                            "ara_model::core::user",
                            "ara-model/src/core/user.rs",
                            235u32,
                        ),
                    );
                }
            };
            let res = app_user::table.find(id).first(conn).optional()?;
            Ok(res)
        }
        pub fn validate_username(conn: &Connection, username_i: &str) -> Result<(), Error> {
            use crate::schema::app_user::dsl::*;
            use diesel::expression::dsl::exists;
            use diesel::select;
            let exists: bool =
                select(exists(app_user.filter(username.eq(username_i)))).get_result(conn)?;
            if !(!exists) {
                return Err(::failure::err_msg(ValidationError::new(
                    "Email already exists in DB",
                )));
            };
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
    pub struct Insert<'a, T> {
        audit_user: &'a str,
        #[diesel(embed)]
        record: T,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a, T: ::std::fmt::Debug> ::std::fmt::Debug for Insert<'a, T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Insert {
                    audit_user: ref __self_0_0,
                    record: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Insert");
                    let _ = debug_trait_builder.field("audit_user", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("record", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a, T: ::std::clone::Clone> ::std::clone::Clone for Insert<'a, T> {
        #[inline]
        fn clone(&self) -> Insert<'a, T> {
            match *self {
                Insert {
                    audit_user: ref __self_0_0,
                    record: ref __self_0_1,
                } => Insert {
                    audit_user: ::std::clone::Clone::clone(&(*__self_0_0)),
                    record: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_as_changeset_for_insert() {
        extern crate std;
        use diesel;
        use diesel::prelude::*;
        use diesel::query_builder::AsChangeset;
        impl<'a, 'update, T> AsChangeset for &'update Insert<'a, T> {
            type Target = inserts::table;
            type Changeset = <(
                diesel::dsl::Eq<inserts::audit_user, &'update &'a str>,
                diesel::dsl::Eq<inserts::record, &'update T>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    inserts::audit_user.eq(&self.audit_user),
                    inserts::record.eq(&self.record),
                )
                    .as_changeset()
            }
        }
        impl<'a, 'update, T> AsChangeset for Insert<'a, T> {
            type Target = inserts::table;
            type Changeset = <(
                diesel::dsl::Eq<inserts::audit_user, &'a str>,
                diesel::dsl::Eq<inserts::record, T>,
            ) as AsChangeset>::Changeset;
            fn as_changeset(self) -> Self::Changeset {
                (
                    inserts::audit_user.eq(self.audit_user),
                    inserts::record.eq(self.record),
                )
                    .as_changeset()
            }
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_insertable_for_insert() {
        extern crate std;
        use diesel;
        use diesel::insertable::Insertable;
        use diesel::prelude::*;
        use diesel::query_builder::UndecoratedInsertRecord;
        impl<'a, 'insert, T> Insertable<inserts::table> for Insert<'a, T> {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<inserts::audit_user, &'a str>>,
                T,
            ) as Insertable<inserts::table>>::Values;
            fn values(self) -> Self::Values {
                (
                    std::option::Option::Some(inserts::audit_user.eq(self.audit_user)),
                    self.record,
                )
                    .values()
            }
        }
        impl<'a, 'insert, T> Insertable<inserts::table> for &'insert Insert<'a, T> {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<inserts::audit_user, &'insert &'a str>>,
                &'insert T,
            ) as Insertable<inserts::table>>::Values;
            fn values(self) -> Self::Values {
                (
                    std::option::Option::Some(inserts::audit_user.eq(&self.audit_user)),
                    &self.record,
                )
                    .values()
            }
        }
        impl<'a, 'insert, T> UndecoratedInsertRecord<inserts::table> for Insert<'a, T> {}
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
    impl<'a, 'b: 'a> From<&'b WithAudit<'a, NewUser<'a>>> for NewUserRecord<'a> {
        fn from(n: &'b WithAudit<'a, NewUser<'a>>) -> Self {
            NewUserRecord {
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
}
