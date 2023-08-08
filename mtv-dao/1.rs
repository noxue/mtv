#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod user {
    use chrono::Local;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use sqlrs::{Conn, Db, Table};
    pub struct User {
        pub id: i32,
        pub nickname: Option<String>,
        pub avatar: Option<String>,
        pub score: i32,
        pub vip: i32,
        pub vip_expire_time: chrono::DateTime<Local>,
        #[sql_json]
        pub auth: Auth,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for User {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "nickname",
                "avatar",
                "score",
                "vip",
                "vip_expire_time",
                "auth",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.nickname,
                &self.avatar,
                &self.score,
                &self.vip,
                &self.vip_expire_time,
                &&self.auth,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "User", names, values)
        }
    }
    impl User {
        pub fn get_columns_vec() -> Vec<&'static str> {
            return <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    "id",
                    "nickname",
                    "avatar",
                    "score",
                    "vip",
                    "vip_expire_time",
                    "auth",
                ]),
            );
        }
        pub fn get_columns() -> String {
            return <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        "id",
                        "nickname",
                        "avatar",
                        "score",
                        "vip",
                        "vip_expire_time",
                        "auth",
                    ]),
                )
                .join(",");
        }
    }
    impl TryFrom<tokio_postgres::Row> for User {
        type Error = tokio_postgres::Error;
        fn try_from(row: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
            Ok(User {
                id: row.try_get(0usize)?,
                nickname: row.try_get(1usize)?,
                avatar: row.try_get(2usize)?,
                score: row.try_get(3usize)?,
                vip: row.try_get(4usize)?,
                vip_expire_time: row.try_get(5usize)?,
                auth: serde_json::from_value(row.try_get(6usize)?).unwrap(),
            })
        }
    }
    impl<'a> TryFrom<&tokio_postgres::Row> for User {
        type Error = tokio_postgres::Error;
        fn try_from(
            row: &tokio_postgres::Row,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(User {
                id: row.try_get(0usize)?,
                nickname: row.try_get(1usize)?,
                avatar: row.try_get(2usize)?,
                score: row.try_get(3usize)?,
                vip: row.try_get(4usize)?,
                vip_expire_time: row.try_get(5usize)?,
                auth: serde_json::from_value(row.try_get(6usize)?).unwrap(),
            })
        }
    }
    pub struct Auth {
        pub phone: Option<String>,
        pub password: Option<String>,
        pub wechat_unionid: Option<String>,
        pub wechat_openid: Option<String>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Auth {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Auth",
                "phone",
                &self.phone,
                "password",
                &self.password,
                "wechat_unionid",
                &self.wechat_unionid,
                "wechat_openid",
                &&self.wechat_openid,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Auth {
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
                    __field2,
                    __field3,
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
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
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
                            "phone" => _serde::__private::Ok(__Field::__field0),
                            "password" => _serde::__private::Ok(__Field::__field1),
                            "wechat_unionid" => _serde::__private::Ok(__Field::__field2),
                            "wechat_openid" => _serde::__private::Ok(__Field::__field3),
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
                            b"phone" => _serde::__private::Ok(__Field::__field0),
                            b"password" => _serde::__private::Ok(__Field::__field1),
                            b"wechat_unionid" => _serde::__private::Ok(__Field::__field2),
                            b"wechat_openid" => _serde::__private::Ok(__Field::__field3),
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
                    marker: _serde::__private::PhantomData<Auth>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Auth;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Auth",
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
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Auth with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Auth with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Auth with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Auth with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Auth {
                            phone: __field0,
                            password: __field1,
                            wechat_unionid: __field2,
                            wechat_openid: __field3,
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
                        let mut __field0: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("phone"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
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
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "wechat_unionid",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "wechat_openid",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
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
                                _serde::__private::de::missing_field("phone")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("password")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("wechat_unionid")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("wechat_openid")?
                            }
                        };
                        _serde::__private::Ok(Auth {
                            phone: __field0,
                            password: __field1,
                            wechat_unionid: __field2,
                            wechat_openid: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "phone",
                    "password",
                    "wechat_unionid",
                    "wechat_openid",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Auth",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Auth>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Auth {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Auth",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phone",
                    &self.phone,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "password",
                    &self.password,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "wechat_unionid",
                    &self.wechat_unionid,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "wechat_openid",
                    &self.wechat_openid,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    /// 鏍规嵁unionid鍒涘缓鐢ㄦ埛
    pub async fn create_user_by_unionid(
        conn: &Conn,
        unionid: &str,
    ) -> anyhow::Result<User> {
        let auth = Auth {
            phone: None,
            password: None,
            wechat_unionid: Some(unionid.to_string()),
            wechat_openid: None,
        };
        let auth = ::serde_json::to_value(&auth).unwrap();
        let row = conn
            .query_one(
                r#"
    insert into users (auth) values ($1) returning *
    "#,
                &[&auth],
            )
            .await?;
        let user: User = row.try_into()?;
        Ok(user)
    }
    /// 鏍规嵁userid 鏇存柊淇℃伅
    pub async fn update(
        conn: &Conn,
        userid: i32,
        nickname: &str,
        avatar: &str,
    ) -> anyhow::Result<()> {
        let row = conn
            .execute(
                r#"
    update users set nickname = $1, avatar = $2 where id = $3
    "#,
                &[&nickname, &avatar, &userid],
            )
            .await?;
        if row == 0 {
            return ::anyhow::__private::Err({
                let error = ::anyhow::__private::format_err(
                    format_args!("鏈壘鍒扮敤鎴?),
                );
                error
            });
        }
        Ok(())
    }
    /// 鏍规嵁userid 璁剧疆鐢ㄦ埛瀵嗙爜
    pub async fn set_password(
        conn: &Conn,
        userid: i32,
        password: &str,
    ) -> anyhow::Result<()> {
        let row = conn
            .execute(
                r#"
    update users set auth = jsonb_set(auth, '{password}', $1::jsonb) where id = $2
    "#,
                &[&::serde_json::to_value(&password).unwrap(), &userid],
            )
            .await?;
        if row == 0 {
            return ::anyhow::__private::Err({
                let error = ::anyhow::__private::format_err(
                    format_args!("鏈壘鍒扮敤鎴?),
                );
                error
            });
        }
        Ok(())
    }
    /// 璁剧疆鎵嬫満鍜屽瘑鐮?    pub async fn set_phone_and_password(
        conn: &Conn,
        userid: i32,
        phone: &str,
        password: &str,
    ) -> anyhow::Result<()> {
        let row = conn
            .execute(
                r#"
    update users set auth = jsonb_set(auth, '{phone}', $1::jsonb, '{password}', $2::jsonb) where id = $3
    "#,
                &[
                    &::serde_json::to_value(&phone).unwrap(),
                    &::serde_json::to_value(&password).unwrap(),
                    &userid,
                ],
            )
            .await?;
        if row == 0 {
            return ::anyhow::__private::Err({
                let error = ::anyhow::__private::format_err(
                    format_args!("鏈壘鍒扮敤鎴?),
                );
                error
            });
        }
        Ok(())
    }
    /// 鏍规嵁id鑾峰彇鐢ㄦ埛
    pub async fn get(conn: &Conn, userid: i32) -> anyhow::Result<User> {
        let row = conn
            .query_one(r#"
    select * from users where id = $1
    "#, &[&userid])
            .await?;
        let user: User = row.try_into()?;
        Ok(user)
    }
    /// 鏍规嵁union鑾峰彇鐢ㄦ埛
    pub async fn get_by_unionid(conn: &Conn, unionid: &str) -> anyhow::Result<User> {
        let row = conn
            .query_one(
                r#"
    select * from users where auth->>'wechat_unionid' = $1
    "#,
                &[&unionid],
            )
            .await?;
        let user: User = row.try_into()?;
        Ok(user)
    }
    /// 鏍规嵁id鍒犻櫎鐢ㄦ埛
    pub async fn delete(conn: &Conn, userid: i32) -> anyhow::Result<()> {
        let row = conn
            .execute(r#"
    delete from users where id = $1
    "#, &[&userid])
            .await?;
        if row == 0 {
            return ::anyhow::__private::Err({
                let error = ::anyhow::__private::format_err(
                    format_args!("鏈壘鍒扮敤鎴?),
                );
                error
            });
        }
        Ok(())
    }
}
use chrono::{Local, NaiveTime};
use postgres_types::FromSql;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlrs::{transaction, Db, Table};
use std::time::SystemTime;
use crate::user::set_password;
pub async fn test() {
    let mut db = Db::get_conn();
    db.begin().await.unwrap();
    set_password(&db, 1, "admin1").await.unwrap();
    let user = user::get(&db, 1).await.unwrap();
    db.commit().await.unwrap();
}
pub async fn test_insert() {
    let db = Db::get_conn();
    let auth = Auth {
        phone: None,
        password: None,
        wechat_unionid: Some("xxxxxxxxxxx".to_string()),
        wechat_openid: None,
    };
    let auth = ::serde_json::to_value(&auth).unwrap();
    let modified = db
        .execute(
            r#"
    insert into users (nickname, auth) values ($1, $2)
    "#,
            &[&"寮犱笁", &auth],
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
                        63u32,
                        "modified",
                        &tmp,
                    ),
                );
            };
            tmp
        }
    };
}
pub struct User {
    id: i32,
    nickname: Option<String>,
    avatar: Option<String>,
    score: i32,
    vip: i32,
    vip_expire_time: chrono::DateTime<Local>,
    #[sql_json]
    auth: Auth,
    create_time: chrono::DateTime<Local>,
    update_time: chrono::DateTime<Local>,
}
#[automatically_derived]
impl ::core::fmt::Debug for User {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "id",
            "nickname",
            "avatar",
            "score",
            "vip",
            "vip_expire_time",
            "auth",
            "create_time",
            "update_time",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.id,
            &self.nickname,
            &self.avatar,
            &self.score,
            &self.vip,
            &self.vip_expire_time,
            &self.auth,
            &self.create_time,
            &&self.update_time,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "User", names, values)
    }
}
impl User {
    pub fn get_columns_vec() -> Vec<&'static str> {
        return <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                "id",
                "nickname",
                "avatar",
                "score",
                "vip",
                "vip_expire_time",
                "auth",
                "create_time",
                "update_time",
            ]),
        );
    }
    pub fn get_columns() -> String {
        return <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    "id",
                    "nickname",
                    "avatar",
                    "score",
                    "vip",
                    "vip_expire_time",
                    "auth",
                    "create_time",
                    "update_time",
                ]),
            )
            .join(",");
    }
}
impl TryFrom<tokio_postgres::Row> for User {
    type Error = tokio_postgres::Error;
    fn try_from(row: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        Ok(User {
            id: row.try_get(0usize)?,
            nickname: row.try_get(1usize)?,
            avatar: row.try_get(2usize)?,
            score: row.try_get(3usize)?,
            vip: row.try_get(4usize)?,
            vip_expire_time: row.try_get(5usize)?,
            auth: serde_json::from_value(row.try_get(6usize)?).unwrap(),
            create_time: row.try_get(7usize)?,
            update_time: row.try_get(8usize)?,
        })
    }
}
impl<'a> TryFrom<&tokio_postgres::Row> for User {
    type Error = tokio_postgres::Error;
    fn try_from(row: &tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        Ok(User {
            id: row.try_get(0usize)?,
            nickname: row.try_get(1usize)?,
            avatar: row.try_get(2usize)?,
            score: row.try_get(3usize)?,
            vip: row.try_get(4usize)?,
            vip_expire_time: row.try_get(5usize)?,
            auth: serde_json::from_value(row.try_get(6usize)?).unwrap(),
            create_time: row.try_get(7usize)?,
            update_time: row.try_get(8usize)?,
        })
    }
}
pub struct Auth {
    phone: Option<String>,
    password: Option<String>,
    wechat_unionid: Option<String>,
    wechat_openid: Option<String>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Auth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Auth",
            "phone",
            &self.phone,
            "password",
            &self.password,
            "wechat_unionid",
            &self.wechat_unionid,
            "wechat_openid",
            &&self.wechat_openid,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Auth {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "Auth",
                false as usize + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "phone",
                &self.phone,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "password",
                &self.password,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "wechat_unionid",
                &self.wechat_unionid,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "wechat_openid",
                &self.wechat_openid,
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
    impl<'de> _serde::Deserialize<'de> for Auth {
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
                __field2,
                __field3,
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
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
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
                        "phone" => _serde::__private::Ok(__Field::__field0),
                        "password" => _serde::__private::Ok(__Field::__field1),
                        "wechat_unionid" => _serde::__private::Ok(__Field::__field2),
                        "wechat_openid" => _serde::__private::Ok(__Field::__field3),
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
                        b"phone" => _serde::__private::Ok(__Field::__field0),
                        b"password" => _serde::__private::Ok(__Field::__field1),
                        b"wechat_unionid" => _serde::__private::Ok(__Field::__field2),
                        b"wechat_openid" => _serde::__private::Ok(__Field::__field3),
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
                marker: _serde::__private::PhantomData<Auth>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Auth;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Auth")
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
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Auth with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Auth with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Auth with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct Auth with 4 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(Auth {
                        phone: __field0,
                        password: __field1,
                        wechat_unionid: __field2,
                        wechat_openid: __field3,
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
                    let mut __field0: _serde::__private::Option<Option<String>> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Option<String>> = _serde::__private::None;
                    let mut __field3: _serde::__private::Option<Option<String>> = _serde::__private::None;
                    while let _serde::__private::Some(__key)
                        = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("phone"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
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
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wechat_unionid",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wechat_openid",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
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
                            _serde::__private::de::missing_field("phone")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("password")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("wechat_unionid")?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("wechat_openid")?
                        }
                    };
                    _serde::__private::Ok(Auth {
                        phone: __field0,
                        password: __field1,
                        wechat_unionid: __field2,
                        wechat_openid: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "phone",
                "password",
                "wechat_unionid",
                "wechat_openid",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Auth",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Auth>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
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
    let sql = {
        let res = ::alloc::fmt::format(
            format_args!("select {0} from users where id = $1", User::get_columns()),
        );
        res
    };
    let row = db
        .query_one(&sql, &[&1])
        .await
        .unwrap_or_else(|_| ::core::panicking::panic_fmt(
            format_args!("鏈壘鍒扮敤鎴?),
        ));
    let user: User = row.try_into().unwrap();
    match user {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}] {2} = {3:#?}\n",
                        "mtv-dao\\src\\lib.rs",
                        106u32,
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
    let sql = {
        let res = ::alloc::fmt::format(
            format_args!(
                "select {0} from users order by id asc limit 5",
                User::get_columns(),
            ),
        );
        res
    };
    let rows = db.query(&sql, &[]).await.unwrap();
    let users: Vec<User> = rows.iter().map(|row| row.try_into().unwrap()).collect();
    match users {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}] {2} = {3:#?}\n",
                        "mtv-dao\\src\\lib.rs",
                        121u32,
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
        .batch_execute(
            "/*\r\n \u{7528}\u{6237}\u{8868} users\r\n id              \u{7528}\u{6237}\u{7f16}\u{53f7}\r\n nickname        \u{7528}\u{6237}\u{6635}\u{79f0}\u{ff0c}\u{53ef}\u{9009}\r\n avatar          \u{7528}\u{6237}\u{5934}\u{50cf}\u{ff0c} \u{53ef}\u{9009}\r\n score           \u{7528}\u{6237}\u{79ef}\u{5206} \u{5145}\u{503c}\u{83b7}\u{53d6}\r\n vip             \u{7528}\u{6237}\u{4f1a}\u{5458}\u{7c7b}\u{578b} 0:\u{666e}\u{901a}\u{7528}\u{6237} 1:\u{6708}\u{4f1a}\u{5458} 2:\u{5b63}\u{4f1a}\u{5458} 3:\u{5e74}\u{4f1a}\u{5458} \u{9ed8}\u{8ba4}0\r\n vip_expire_time \u{4f1a}\u{5458}\u{8fc7}\u{671f}\u{65f6}\u{95f4} \u{53ef}\u{7a7a}\u{ff0c}\u{9ed8}\u{8ba4} 1970-01-01 00:00:00\r\n auth \u{8ba4}\u{8bc1}\u{4fe1}\u{606f} jsonb\u{683c}\u{5f0f}\r\n {\r\n \"phone\":\"\",   \u{53ef}\u{9009}\r\n \"password\":\"\", \u{53ef}\u{9009}\r\n \"wechat_unionid\":\"\", \u{53ef}\u{9009}\r\n \"wechat_openid\":\"\", \u{53ef}\u{9009}\r\n }\r\n create_time     \u{521b}\u{5efa}\u{65f6}\u{95f4}\u{ff0c}\u{65f6}\u{95f4}\u{6233}+\u{65f6}\u{533a}\r\n update_time     \u{66f4}\u{65b0}\u{65f6}\u{95f4}\u{ff0c}\u{65f6}\u{533a}\r\n */\r\nCREATE TABLE users (\r\n    id serial PRIMARY KEY,\r\n    nickname varchar(20),\r\n    avatar varchar(255),\r\n    score int4 DEFAULT 0,\r\n    vip int4 DEFAULT 0,\r\n    vip_expire_time timestamp with time zone not null DEFAULT now(),\r\n    auth jsonb DEFAULT \'{}\' :: jsonb,\r\n    create_time timestamp with time zone not null DEFAULT now(),\r\n    update_time timestamp with time zone not null DEFAULT now()\r\n);\r\n\r\nCREATE UNIQUE INDEX users_phone_uindex ON users ((auth ->> \'phone\' :: text) varchar_pattern_ops);\r\n\r\nCREATE UNIQUE INDEX users_wechat_unionid_uindex ON users (\r\n    (auth ->> \'wechat_unionid\' :: text) varchar_pattern_ops\r\n);\r\n\r\nCREATE UNIQUE INDEX users_wechat_openid_uindex ON users (\r\n    (auth ->> \'wechat_openid\' :: text) varchar_pattern_ops\r\n);\r\n\r\n/*\r\n * \u{5f71}\u{7247}\u{8868} movies\r\n id              \u{5f71}\u{7247}\u{7f16}\u{53f7}\r\n name            \u{5f71}\u{7247}\u{540d}\u{79f0}\r\n cover           \u{5f71}\u{7247}\u{5c01}\u{9762}\r\n total           \u{5f71}\u{7247}\u{603b}\u{96c6}\u{6570}\r\n description     \u{5f71}\u{7247}\u{63cf}\u{8ff0}\r\n is_top          \u{662f}\u{5426}\u{7f6e}\u{9876} boolean\r\n is_hot          \u{662f}\u{5426}\u{70ed}\u{95e8} boolean\r\n tags            \u{5f71}\u{7247}\u{6807}\u{7b7e} \u{6570}\u{7ec4}\r\n price_total     \u{6574}\u{90e8}\u{5f71}\u{7247}\u{4ef7}\u{683c}\r\n price_single    \u{5355}\u{96c6}\u{5f71}\u{7247}\u{4ef7}\u{683c}\r\n is_show         \u{5f71}\u{7247}\u{72b6}\u{6001} boolean\r\n view            \u{89c2}\u{770b}\u{6570}\r\n create_time     \u{521b}\u{5efa}\u{65f6}\u{95f4}\r\n update_time     \u{66f4}\u{65b0}\u{65f6}\u{95f4}\r\n */\r\nCREATE TABLE movies (\r\n    id serial PRIMARY KEY,\r\n    name varchar(20) NOT NULL,\r\n    cover varchar(255) NOT NULL,\r\n    total int4 NOT NULL,\r\n    description varchar(255) NOT NULL,\r\n    is_top bool NOT NULL DEFAULT false,\r\n    is_hot bool NOT NULL DEFAULT false,\r\n    tags varchar(255) [] NOT NULL DEFAULT \'{}\' :: varchar(255) [],\r\n    price_total int4 NOT NULL DEFAULT 0,\r\n    price_single int4 NOT NULL DEFAULT 0,\r\n    is_show bool NOT NULL DEFAULT false,\r\n    view int4 NOT NULL DEFAULT 0,\r\n    create_time timestamp with time zone not null DEFAULT now(),\r\n    update_time timestamp with time zone not null DEFAULT now()\r\n);\r\n\r\nCREATE UNIQUE INDEX movies_name_uindex ON movies (name);\r\n\r\n/*\r\n \u{5f71}\u{7247}\u{96c6}\u{8868} movie_parts\r\n id \r\n movie_id        \u{5f71}\u{7247}\u{7f16}\u{53f7} \u{521b}\u{5efa}\u{5916}\u{952e}\r\n name            \u{96c6}\u{540d}\u{79f0}\r\n video           \u{96c6}\u{89c6}\u{9891}\u{5730}\u{5740}\r\n price           \u{96c6}\u{4ef7}\u{683c}\r\n status          \u{96c6}\u{72b6}\u{6001} 0:\u{4e0b}\u{67b6} 1:\u{4e0a}\u{67b6}\r\n likes           \u{70b9}\u{8d5e}\u{6570}\r\n share           \u{5206}\u{4eab}\u{6570}\r\n view            \u{89c2}\u{770b}\u{6570}\r\n rank            \u{6392}\u{5e8f}\r\n create_time     \u{521b}\u{5efa}\u{65f6}\u{95f4}\r\n update_time     \u{66f4}\u{65b0}\u{65f6}\u{95f4}\r\n */\r\nCREATE TABLE movie_parts (\r\n    id serial PRIMARY KEY,\r\n    movie_id int4 NOT NULL,\r\n    name varchar(20) NOT NULL,\r\n    video varchar(255) NOT NULL,\r\n    price int4 NOT NULL DEFAULT 0,\r\n    status int4 NOT NULL DEFAULT 0,\r\n    likes int4 NOT NULL DEFAULT 0,\r\n    share int4 NOT NULL DEFAULT 0,\r\n    view int4 NOT NULL DEFAULT 0,\r\n    rank int4 NOT NULL DEFAULT 0,\r\n    create_time timestamp with time zone not null DEFAULT now(),\r\n    update_time timestamp with time zone not null DEFAULT now()\r\n);\r\n\r\n-- \u{5916}\u{952e}\r\nALTER TABLE\r\n    movie_parts\r\nADD\r\n    CONSTRAINT movie_parts_movie_id_fkey FOREIGN KEY (movie_id) REFERENCES movies(id);\r\n\r\n/*\r\n \u{5145}\u{503c}\u{8bb0}\u{5f55}\u{8868} recharge_records\r\n id\r\n user_id         \u{7528}\u{6237}\u{7f16}\u{53f7}\r\n amount          \u{5145}\u{503c}\u{91d1}\u{989d}\r\n score           \u{83b7}\u{5f97}\u{79ef}\u{5206}  \u{5982}\u{679c}\u{79ef}\u{5206}\u{4e3a}0 \u{5c31}\u{662f}\u{5f00}\u{4f1a}\u{5458}\u{ff0c}\u{5728}\u{5907}\u{6ce8}\u{4e2d}\u{8bf4}\u{660e}\r\n mark            \u{5907}\u{6ce8}\r\n status          \u{5145}\u{503c}\u{72b6}\u{6001} 0:\u{5931}\u{8d25} 1:\u{6210}\u{529f}\r\n create_time     \u{521b}\u{5efa}\u{65f6}\u{95f4}\r\n update_time     \u{66f4}\u{65b0}\u{65f6}\u{95f4}\r\n */\r\nCREATE TABLE recharge_records (\r\n    id serial PRIMARY KEY,\r\n    user_id int4 NOT NULL,\r\n    amount int4 NOT NULL DEFAULT 0,\r\n    score int4 NOT NULL DEFAULT 0,\r\n    mark varchar(255) NOT NULL,\r\n    status int4 NOT NULL DEFAULT 0,\r\n    create_time timestamp with time zone not null DEFAULT now(),\r\n    update_time timestamp with time zone not null DEFAULT now()\r\n);\r\n\r\n-- \u{5916}\u{952e}\r\nALTER TABLE\r\n    recharge_records\r\nADD\r\n    CONSTRAINT recharge_records_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);\r\n\r\n/*\r\n \u{6d88}\u{8d39}\u{8bb0}\u{5f55} consume_records\r\n id \r\n user_id         \u{7528}\u{6237}\u{7f16}\u{53f7}\r\n movie_id        \u{5f71}\u{7247}\u{7f16}\u{53f7}\r\n movie_part_id   \u{5f71}\u{7247}\u{96c6}\u{7f16}\u{53f7}\r\n score           \u{6d88}\u{8d39}\u{79ef}\u{5206}\r\n mark            \u{5907}\u{6ce8}\r\n create_time     \u{521b}\u{5efa}\u{65f6}\u{95f4}\r\n */\r\nCREATE TABLE consume_records (\r\n    id serial PRIMARY KEY,\r\n    user_id int4 NOT NULL,\r\n    movie_id int4 NOT NULL,\r\n    movie_part_id int4 NOT NULL,\r\n    score int4 NOT NULL DEFAULT 0,\r\n    mark varchar(255) NOT NULL,\r\n    create_time timestamp with time zone not null DEFAULT now()\r\n);\r\n\r\n-- \u{5916}\u{952e}\r\nALTER TABLE\r\n    consume_records\r\nADD\r\n    CONSTRAINT consume_records_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);\r\n\r\nALTER TABLE\r\n    consume_records\r\nADD\r\n    CONSTRAINT consume_records_movie_id_fkey FOREIGN KEY (movie_id) REFERENCES movies(id);\r\n\r\nALTER TABLE\r\n    consume_records\r\nADD\r\n    CONSTRAINT consume_records_movie_part_id_fkey FOREIGN KEY (movie_part_id) REFERENCES movie_parts(id);\r\n\r\n/*\r\n \u{6d4f}\u{89c8}\u{8bb0}\u{5f55} view_records\r\n id\r\n user_id         \u{7528}\u{6237}\u{7f16}\u{53f7}\r\n movie_id        \u{5f71}\u{7247}\u{7f16}\u{53f7}\r\n movie_part_id   \u{5f71}\u{7247}\u{96c6}\u{7f16}\u{53f7}\r\n create_time     \u{521b}\u{5efa}\u{65f6}\u{95f4}\r\n update_time     \u{66f4}\u{65b0}\u{65f6}\u{95f4}\r\n */\r\nCREATE TABLE view_records (\r\n    id serial PRIMARY KEY,\r\n    user_id int4 NOT NULL,\r\n    movie_id int4 NOT NULL,\r\n    movie_part_id int4 NOT NULL,\r\n    create_time timestamp with time zone not null DEFAULT now(),\r\n    update_time timestamp with time zone not null DEFAULT now()\r\n);\r\n\r\n-- \u{5916}\u{952e}\r\nALTER TABLE\r\n    view_records\r\nADD\r\n    CONSTRAINT view_records_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);\r\n\r\nALTER TABLE\r\n    view_records\r\nADD\r\n    CONSTRAINT view_records_movie_id_fkey FOREIGN KEY (movie_id) REFERENCES movies(id);\r\n\r\nALTER TABLE\r\n    view_records\r\nADD\r\n    CONSTRAINT view_records_movie_part_id_fkey FOREIGN KEY (movie_part_id) REFERENCES movie_parts(id);\r\n\r\n/*\r\n \u{8ffd}\u{5267}\u{8bb0}\u{5f55} follow_records\r\n id\r\n user_id         \u{7528}\u{6237}\u{7f16}\u{53f7}\r\n movie_id        \u{5f71}\u{7247}\u{7f16}\u{53f7}\r\n create_time     \u{521b}\u{5efa}\u{65f6}\u{95f4}\r\n */\r\nCREATE TABLE follow_records (\r\n    id serial PRIMARY KEY,\r\n    user_id int4 NOT NULL,\r\n    movie_id int4 NOT NULL,\r\n    create_time timestamp with time zone not null DEFAULT now()\r\n);\r\n\r\n-- \u{5916}\u{952e}\r\nALTER TABLE\r\n    follow_records\r\nADD\r\n    CONSTRAINT follow_records_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);\r\n\r\nALTER TABLE\r\n    follow_records\r\nADD\r\n    CONSTRAINT follow_records_movie_id_fkey FOREIGN KEY (movie_id) REFERENCES movies(id);",
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
                        133u32,
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
        .batch_execute(
            "-- \u{5220}\u{9664}\u{5916}\u{952e}\u{548c}\u{7d22}\u{5f15}\r\nALTER TABLE\r\n    movie_parts DROP CONSTRAINT IF EXISTS movie_parts_movie_id_fkey;\r\n\r\nDROP INDEX IF EXISTS movie_parts_movie_id_fkey;\r\n\r\nDROP INDEX IF EXISTS movies_name_uindex;\r\n\r\nDROP INDEX IF EXISTS users_wechat_openid_uindex;\r\n\r\nDROP INDEX IF EXISTS users_wechat_unionid_uindex;\r\n\r\nDROP INDEX IF EXISTS users_phone_uindex;\r\n\r\n-- \u{5220}\u{9664}\u{8868}\r\ndrop table if exists follow_records;\r\n\r\ndrop table if exists view_records;\r\n\r\ndrop table if exists consume_records;\r\n\r\ndrop table if exists recharge_records;\r\n\r\ndrop table if exists movie_parts;\r\n\r\ndrop table if exists movies;\r\n\r\ndrop table if exists users;",
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
                        141u32,
                        "modified",
                        &tmp,
                    ),
                );
            };
            tmp
        }
    };
}
