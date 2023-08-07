#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::time::SystemTime;
use postgres_types::FromSql;
use serde::{Serialize, Deserialize};
use serde_json::json;
use chrono::{Local, NaiveTime};
use sqlrs::{Db, FromRow};
use sqlrs_macros::Table;
pub async fn test() {
    test_find_one().await;
    test_macro().await;
}
pub async fn test_insert() {
    let db = Db::get_conn();
    let info = UserInfo {
        name: "寮犱笁".to_string(),
        password: "123456".to_string(),
    };
    let info = ::serde_json::to_value(&info).unwrap();
    let modified = db
        .execute(
            r#"
    insert into users (name, age, info) values ($1, $2, $3)
    "#,
            &[&"寮犱笁", &7, &info],
        )
        .await
        .unwrap();
    match modified {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}] {2} = {3:#?}\n",
                        "mtv-dao\\src\\lib.rs",
                        50u32,
                        "modified",
                        &tmp,
                    ),
                );
            };
            tmp
        }
    };
}
struct UserInfo {
    name: String,
    password: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for UserInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UserInfo",
            "name",
            &self.name,
            "password",
            &&self.password,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for UserInfo {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "UserInfo",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "password",
                &self.password,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for UserInfo {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "name" => _serde::__private::Ok(__Field::__field0),
                        "password" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"name" => _serde::__private::Ok(__Field::__field0),
                        b"password" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<UserInfo>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = UserInfo;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct UserInfo",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct UserInfo with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct UserInfo with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(UserInfo {
                        name: __field0,
                        password: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key)
                        = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "password",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("name")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("password")?
                        }
                    };
                    _serde::__private::Ok(UserInfo {
                        name: __field0,
                        password: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["name", "password"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "UserInfo",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<UserInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::default::Default for UserInfo {
    #[inline]
    fn default() -> UserInfo {
        UserInfo {
            name: ::core::default::Default::default(),
            password: ::core::default::Default::default(),
        }
    }
}
pub struct User {
    id: i32,
    name: String,
    age: i32,
    created_at: chrono::DateTime<Local>,
}
#[automatically_derived]
impl ::core::fmt::Debug for User {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "User",
            "id",
            &self.id,
            "name",
            &self.name,
            "age",
            &self.age,
            "created_at",
            &&self.created_at,
        )
    }
}
impl User {
    pub fn get_columns_vec() -> Vec<&'static str> {
        return <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new(["id", "name", "age", "created_at"]),
        );
    }
    pub fn get_columns() -> String {
        return <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new(["id", "name", "age", "created_at"]),
            )
            .join(",");
    }
}
impl TryFrom<tokio_postgres::Row> for User {
    type Error = tokio_postgres::Error;
    fn try_from(row: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        Ok(User {
            id: row.try_get(0usize)?,
            name: row.try_get(1usize)?,
            age: row.try_get(2usize)?,
            created_at: row.try_get(3usize)?,
        })
    }
}
pub async fn test_macro() {
    let fs = User::get_columns_vec();
    {
        ::std::io::_print(format_args!("get_columns:{0:?}\n", fs));
    };
    let fss = User::get_columns();
    {
        ::std::io::_print(format_args!("get_columns_str:{0}\n", fss));
    };
}
pub async fn test_find_one() {
    let db = Db::get_conn();
    let row = db
        .query_one(
            r#"
    select id, name, age, info, created_at from users where id = $1
    "#,
            &[&1],
        )
        .await
        .unwrap();
    let user: User = row.try_into().unwrap();
    match user {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}] {2} = {3:#?}\n",
                        "mtv-dao\\src\\lib.rs",
                        91u32,
                        "user",
                        &tmp,
                    ),
                );
            };
            tmp
        }
    };
}
pub async fn test_select() {
    let db = Db::get_conn();
    let rows = db
        .query(r#"
    select id, name, age, info, created_at from users
    "#, &[])
        .await
        .unwrap();
    let users: Vec<User> = rows
        .iter()
        .map(|row| User {
            id: row.get(0),
            name: row.get(1),
            age: row.get(2),
            created_at: row.get(4),
        })
        .collect();
    match users {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}] {2} = {3:#?}\n",
                        "mtv-dao\\src\\lib.rs",
                        118u32,
                        "users",
                        &tmp,
                    ),
                );
            };
            tmp
        }
    };
}
pub async fn up() {
    let db = Db::get_conn();
    let modified = db
        .execute(
            r#"
    create table if not exists users (
        id serial primary key,
        name varchar not null,
        info json not null,
        age int not null,
        created_at timestamp with time zone not null default now()
    )
    "#,
            &[],
        )
        .await
        .unwrap();
    match modified {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}] {2} = {3:#?}\n",
                        "mtv-dao\\src\\lib.rs",
                        142u32,
                        "modified",
                        &tmp,
                    ),
                );
            };
            tmp
        }
    };
}
pub async fn down() {
    let db = Db::get_conn();
    let modified = db
        .execute(r#"
    drop table if exists users
    "#, &[])
        .await
        .unwrap();
    match modified {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}] {2} = {3:#?}\n",
                        "mtv-dao\\src\\lib.rs",
                        158u32,
                        "modified",
                        &tmp,
                    ),
                );
            };
            tmp
        }
    };
}
