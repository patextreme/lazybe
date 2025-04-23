mod todo {
    use lazybe::macros::{Entity, Enum, Newtype};
    use serde::{Deserialize, Serialize};
    use sqlx::types::chrono::{DateTime, Utc};
    use utoipa::ToSchema;
    pub struct TodoId(u64);
    #[automatically_derived]
    impl ::core::fmt::Debug for TodoId {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "TodoId", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TodoId {
        #[inline]
        fn clone(&self) -> TodoId {
            TodoId(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TodoId {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TodoId {
        #[inline]
        fn eq(&self, other: &TodoId) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for TodoId {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u64>;
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TodoId {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "TodoId",
                    &self.0,
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TodoId {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TodoId>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TodoId;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct TodoId",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: u64 = <u64 as _serde::Deserialize>::deserialize(
                            __e,
                        )?;
                        _serde::__private::Ok(TodoId(__field0))
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
                            u64,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct TodoId with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(TodoId(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "TodoId",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TodoId>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl<'r, Db> sqlx::Decode<'r, Db> for TodoId
    where
        Db: sqlx::Database,
        for<'s> u64: sqlx::Decode<'s, Db>,
    {
        fn decode(
            value: <Db as sqlx::Database>::ValueRef<'r>,
        ) -> Result<Self, sqlx::error::BoxDynError> {
            let inner_value = <u64 as sqlx::Decode<Db>>::decode(value)?;
            Ok(TodoId(inner_value))
        }
    }
    impl<Db> sqlx::Type<Db> for TodoId
    where
        Db: sqlx::Database,
        u64: sqlx::Type<Db>,
    {
        fn type_info() -> <Db as sqlx::Database>::TypeInfo {
            <u64 as sqlx::Type<Db>>::type_info()
        }
    }
    impl From<TodoId> for sea_query::Value {
        fn from(value: TodoId) -> Self {
            value.0.into()
        }
    }
    impl sea_query::Nullable for TodoId {
        fn null() -> sea_query::Value {
            <u64 as sea_query::Nullable>::null()
        }
    }
    impl utoipa::__dev::ComposeSchema for TodoId {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(
                    utoipa::openapi::schema::SchemaType::new(
                        utoipa::openapi::schema::Type::Integer,
                    ),
                )
                .format(
                    Some(
                        utoipa::openapi::schema::SchemaFormat::KnownFormat(
                            utoipa::openapi::schema::KnownFormat::Int64,
                        ),
                    ),
                )
                .minimum(Some(0f64))
                .into()
        }
    }
    impl utoipa::ToSchema for TodoId {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("TodoId")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas.extend([]);
        }
    }
    #[lazybe(table = "todo", endpoint = "/todos", derive_to_schema)]
    pub struct Todo {
        #[lazybe(primary_key, generate_with = "fourty_two")]
        pub id: TodoId,
        pub title: String,
        pub description: Option<String>,
        pub status: Status,
        #[lazybe(created_at)]
        pub created_at: DateTime<Utc>,
        #[lazybe(updated_at)]
        pub updated_at: DateTime<Utc>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Todo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "title",
                "description",
                "status",
                "created_at",
                "updated_at",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.title,
                &self.description,
                &self.status,
                &self.created_at,
                &&self.updated_at,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Todo", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Todo {
        #[inline]
        fn clone(&self) -> Todo {
            Todo {
                id: ::core::clone::Clone::clone(&self.id),
                title: ::core::clone::Clone::clone(&self.title),
                description: ::core::clone::Clone::clone(&self.description),
                status: ::core::clone::Clone::clone(&self.status),
                created_at: ::core::clone::Clone::clone(&self.created_at),
                updated_at: ::core::clone::Clone::clone(&self.updated_at),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Todo {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Todo",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "title",
                    &self.title,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "description",
                    &self.description,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "status",
                    &self.status,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "created_at",
                    &self.created_at,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updated_at",
                    &self.updated_at,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Todo {
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
                    __field4,
                    __field5,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
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
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
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
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "title" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "status" => _serde::__private::Ok(__Field::__field3),
                            "created_at" => _serde::__private::Ok(__Field::__field4),
                            "updated_at" => _serde::__private::Ok(__Field::__field5),
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
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"title" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"status" => _serde::__private::Ok(__Field::__field3),
                            b"created_at" => _serde::__private::Ok(__Field::__field4),
                            b"updated_at" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
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
                    marker: _serde::__private::PhantomData<Todo>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Todo;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Todo",
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
                            TodoId,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Todo with 6 elements",
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
                                        &"struct Todo with 6 elements",
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
                                        &"struct Todo with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Status,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Todo with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            DateTime<Utc>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Todo with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            DateTime<Utc>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct Todo with 6 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Todo {
                            id: __field0,
                            title: __field1,
                            description: __field2,
                            status: __field3,
                            created_at: __field4,
                            updated_at: __field5,
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
                        let mut __field0: _serde::__private::Option<TodoId> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Status> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<DateTime<Utc>> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<DateTime<Utc>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<TodoId>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("title"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
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
                                            <__A::Error as _serde::de::Error>::duplicate_field("status"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Status>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "created_at",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            DateTime<Utc>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "updated_at",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            DateTime<Utc>,
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
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("title")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("description")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("status")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("created_at")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("updated_at")?
                            }
                        };
                        _serde::__private::Ok(Todo {
                            id: __field0,
                            title: __field1,
                            description: __field2,
                            status: __field3,
                            created_at: __field4,
                            updated_at: __field5,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "title",
                    "description",
                    "status",
                    "created_at",
                    "updated_at",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Todo",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Todo>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct CreateTodo {
        pub title: String,
        pub description: Option<String>,
        pub status: Status,
    }
    impl utoipa::__dev::ComposeSchema for CreateTodo {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            {
                let mut object = utoipa::openapi::ObjectBuilder::new();
                object = object
                    .property(
                        "title",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            ),
                    )
                    .required("title");
                object = object
                    .property(
                        "description",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::String,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            }),
                    );
                object = object
                    .property(
                        "status",
                        utoipa::openapi::schema::RefBuilder::new()
                            .ref_location_from_schema_name(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}", <Status as utoipa::ToSchema>::name()),
                                    );
                                    res
                                }),
                            ),
                    )
                    .required("status");
                object
            }
                .into()
        }
    }
    impl utoipa::ToSchema for CreateTodo {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("CreateTodo")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas
                .extend([
                    (
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", <Status as utoipa::ToSchema>::name()),
                                );
                                res
                            }),
                        ),
                        <Status as utoipa::PartialSchema>::schema(),
                    ),
                ]);
            <Status as utoipa::ToSchema>::schemas(schemas);
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CreateTodo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "CreateTodo",
                "title",
                &self.title,
                "description",
                &self.description,
                "status",
                &&self.status,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CreateTodo {
        #[inline]
        fn clone(&self) -> CreateTodo {
            CreateTodo {
                title: ::core::clone::Clone::clone(&self.title),
                description: ::core::clone::Clone::clone(&self.description),
                status: ::core::clone::Clone::clone(&self.status),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for CreateTodo {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "CreateTodo",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "title",
                    &self.title,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "description",
                    &self.description,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "status",
                    &self.status,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CreateTodo {
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
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
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
                            "title" => _serde::__private::Ok(__Field::__field0),
                            "description" => _serde::__private::Ok(__Field::__field1),
                            "status" => _serde::__private::Ok(__Field::__field2),
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
                            b"title" => _serde::__private::Ok(__Field::__field0),
                            b"description" => _serde::__private::Ok(__Field::__field1),
                            b"status" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
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
                    marker: _serde::__private::PhantomData<CreateTodo>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CreateTodo;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct CreateTodo",
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
                                        &"struct CreateTodo with 3 elements",
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
                                        &"struct CreateTodo with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Status,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct CreateTodo with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(CreateTodo {
                            title: __field0,
                            description: __field1,
                            status: __field2,
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
                        let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Status> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("title"),
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
                                                "description",
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
                                            <__A::Error as _serde::de::Error>::duplicate_field("status"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Status>(&mut __map)?,
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
                                _serde::__private::de::missing_field("title")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("description")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("status")?
                            }
                        };
                        _serde::__private::Ok(CreateTodo {
                            title: __field0,
                            description: __field1,
                            status: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "title",
                    "description",
                    "status",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "CreateTodo",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CreateTodo>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct UpdateTodo {
        pub title: Option<String>,
        pub description: Option<Option<String>>,
        pub status: Option<Status>,
    }
    impl utoipa::__dev::ComposeSchema for UpdateTodo {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            {
                let mut object = utoipa::openapi::ObjectBuilder::new();
                object = object
                    .property(
                        "title",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::String,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            }),
                    );
                object = object
                    .property(
                        "description",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::String,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            }),
                    );
                object = object
                    .property(
                        "status",
                        utoipa::openapi::schema::OneOfBuilder::new()
                            .item(
                                utoipa::openapi::schema::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::Type::Null),
                            )
                            .item(
                                utoipa::openapi::schema::RefBuilder::new()
                                    .ref_location_from_schema_name(
                                        ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("{0}", <Status as utoipa::ToSchema>::name()),
                                            );
                                            res
                                        }),
                                    ),
                            ),
                    );
                object
            }
                .into()
        }
    }
    impl utoipa::ToSchema for UpdateTodo {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("UpdateTodo")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas
                .extend([
                    (
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", <Status as utoipa::ToSchema>::name()),
                                );
                                res
                            }),
                        ),
                        <Status as utoipa::PartialSchema>::schema(),
                    ),
                ]);
            <Status as utoipa::ToSchema>::schemas(schemas);
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UpdateTodo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "UpdateTodo",
                "title",
                &self.title,
                "description",
                &self.description,
                "status",
                &&self.status,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UpdateTodo {
        #[inline]
        fn clone(&self) -> UpdateTodo {
            UpdateTodo {
                title: ::core::clone::Clone::clone(&self.title),
                description: ::core::clone::Clone::clone(&self.description),
                status: ::core::clone::Clone::clone(&self.status),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for UpdateTodo {
        #[inline]
        fn default() -> UpdateTodo {
            UpdateTodo {
                title: ::core::default::Default::default(),
                description: ::core::default::Default::default(),
                status: ::core::default::Default::default(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UpdateTodo {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "UpdateTodo",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "title",
                    &self.title,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "description",
                    &self.description,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "status",
                    &self.status,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UpdateTodo {
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
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
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
                            "title" => _serde::__private::Ok(__Field::__field0),
                            "description" => _serde::__private::Ok(__Field::__field1),
                            "status" => _serde::__private::Ok(__Field::__field2),
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
                            b"title" => _serde::__private::Ok(__Field::__field0),
                            b"description" => _serde::__private::Ok(__Field::__field1),
                            b"status" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
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
                    marker: _serde::__private::PhantomData<UpdateTodo>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UpdateTodo;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct UpdateTodo",
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
                                        &"struct UpdateTodo with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<Option<String>>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UpdateTodo with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<Status>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UpdateTodo with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(UpdateTodo {
                            title: __field0,
                            description: __field1,
                            status: __field2,
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
                        let mut __field1: _serde::__private::Option<
                            Option<Option<String>>,
                        > = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<Status>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("title"),
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
                                                "description",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<Option<String>>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("status"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<Status>,
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
                                _serde::__private::de::missing_field("title")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("description")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("status")?
                            }
                        };
                        _serde::__private::Ok(UpdateTodo {
                            title: __field0,
                            description: __field1,
                            status: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "title",
                    "description",
                    "status",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UpdateTodo",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<UpdateTodo>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct ReplaceTodo {
        pub title: String,
        pub description: Option<String>,
        pub status: Status,
    }
    impl utoipa::__dev::ComposeSchema for ReplaceTodo {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            {
                let mut object = utoipa::openapi::ObjectBuilder::new();
                object = object
                    .property(
                        "title",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            ),
                    )
                    .required("title");
                object = object
                    .property(
                        "description",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::String,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            }),
                    );
                object = object
                    .property(
                        "status",
                        utoipa::openapi::schema::RefBuilder::new()
                            .ref_location_from_schema_name(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}", <Status as utoipa::ToSchema>::name()),
                                    );
                                    res
                                }),
                            ),
                    )
                    .required("status");
                object
            }
                .into()
        }
    }
    impl utoipa::ToSchema for ReplaceTodo {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ReplaceTodo")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas
                .extend([
                    (
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", <Status as utoipa::ToSchema>::name()),
                                );
                                res
                            }),
                        ),
                        <Status as utoipa::PartialSchema>::schema(),
                    ),
                ]);
            <Status as utoipa::ToSchema>::schemas(schemas);
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ReplaceTodo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ReplaceTodo",
                "title",
                &self.title,
                "description",
                &self.description,
                "status",
                &&self.status,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ReplaceTodo {
        #[inline]
        fn clone(&self) -> ReplaceTodo {
            ReplaceTodo {
                title: ::core::clone::Clone::clone(&self.title),
                description: ::core::clone::Clone::clone(&self.description),
                status: ::core::clone::Clone::clone(&self.status),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ReplaceTodo {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ReplaceTodo",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "title",
                    &self.title,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "description",
                    &self.description,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "status",
                    &self.status,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ReplaceTodo {
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
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
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
                            "title" => _serde::__private::Ok(__Field::__field0),
                            "description" => _serde::__private::Ok(__Field::__field1),
                            "status" => _serde::__private::Ok(__Field::__field2),
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
                            b"title" => _serde::__private::Ok(__Field::__field0),
                            b"description" => _serde::__private::Ok(__Field::__field1),
                            b"status" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
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
                    marker: _serde::__private::PhantomData<ReplaceTodo>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ReplaceTodo;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ReplaceTodo",
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
                                        &"struct ReplaceTodo with 3 elements",
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
                                        &"struct ReplaceTodo with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Status,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct ReplaceTodo with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ReplaceTodo {
                            title: __field0,
                            description: __field1,
                            status: __field2,
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
                        let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Status> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("title"),
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
                                                "description",
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
                                            <__A::Error as _serde::de::Error>::duplicate_field("status"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Status>(&mut __map)?,
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
                                _serde::__private::de::missing_field("title")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("description")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("status")?
                            }
                        };
                        _serde::__private::Ok(ReplaceTodo {
                            title: __field0,
                            description: __field1,
                            status: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "title",
                    "description",
                    "status",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ReplaceTodo",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ReplaceTodo>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl From<ReplaceTodo> for UpdateTodo {
        fn from(value: ReplaceTodo) -> Self {
            Self {
                title: Some(value.title),
                description: Some(value.description),
                status: Some(value.status),
            }
        }
    }
    pub struct TodoFilter;
    #[automatically_derived]
    impl ::core::fmt::Debug for TodoFilter {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "TodoFilter")
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TodoFilter {
        #[inline]
        fn clone(&self) -> TodoFilter {
            TodoFilter
        }
    }
    impl TodoFilter {
        pub fn id() -> lazybe::filter::FilterCol<Todo, TodoId> {
            lazybe::filter::FilterCol::new(TodoSeaQueryIdent::Id)
        }
        pub fn title() -> lazybe::filter::FilterCol<Todo, String> {
            lazybe::filter::FilterCol::new(TodoSeaQueryIdent::Title)
        }
        pub fn description() -> lazybe::filter::FilterCol<Todo, Option<String>> {
            lazybe::filter::FilterCol::new(TodoSeaQueryIdent::Description)
        }
        pub fn status() -> lazybe::filter::FilterCol<Todo, Status> {
            lazybe::filter::FilterCol::new(TodoSeaQueryIdent::Status)
        }
        pub fn created_at() -> lazybe::filter::FilterCol<Todo, DateTime<Utc>> {
            lazybe::filter::FilterCol::new(TodoSeaQueryIdent::CreatedAt)
        }
        pub fn updated_at() -> lazybe::filter::FilterCol<Todo, DateTime<Utc>> {
            lazybe::filter::FilterCol::new(TodoSeaQueryIdent::UpdatedAt)
        }
    }
    pub struct TodoSort;
    #[automatically_derived]
    impl ::core::fmt::Debug for TodoSort {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "TodoSort")
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TodoSort {
        #[inline]
        fn clone(&self) -> TodoSort {
            TodoSort
        }
    }
    impl TodoSort {
        pub fn id() -> lazybe::sort::SortCol<Todo> {
            lazybe::sort::SortCol::new(TodoSeaQueryIdent::Id)
        }
        pub fn title() -> lazybe::sort::SortCol<Todo> {
            lazybe::sort::SortCol::new(TodoSeaQueryIdent::Title)
        }
        pub fn description() -> lazybe::sort::SortCol<Todo> {
            lazybe::sort::SortCol::new(TodoSeaQueryIdent::Description)
        }
        pub fn status() -> lazybe::sort::SortCol<Todo> {
            lazybe::sort::SortCol::new(TodoSeaQueryIdent::Status)
        }
        pub fn created_at() -> lazybe::sort::SortCol<Todo> {
            lazybe::sort::SortCol::new(TodoSeaQueryIdent::CreatedAt)
        }
        pub fn updated_at() -> lazybe::sort::SortCol<Todo> {
            lazybe::sort::SortCol::new(TodoSeaQueryIdent::UpdatedAt)
        }
    }
    pub struct TodoSqlxRow {
        id: TodoId,
        title: String,
        description: Option<String>,
        status: Status,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }
    #[automatically_derived]
    impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for TodoSqlxRow
    where
        &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
        TodoId: ::sqlx::decode::Decode<'a, R::Database>,
        TodoId: ::sqlx::types::Type<R::Database>,
        String: ::sqlx::decode::Decode<'a, R::Database>,
        String: ::sqlx::types::Type<R::Database>,
        Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
        Option<String>: ::sqlx::types::Type<R::Database>,
        Status: ::sqlx::decode::Decode<'a, R::Database>,
        Status: ::sqlx::types::Type<R::Database>,
        DateTime<Utc>: ::sqlx::decode::Decode<'a, R::Database>,
        DateTime<Utc>: ::sqlx::types::Type<R::Database>,
        DateTime<Utc>: ::sqlx::decode::Decode<'a, R::Database>,
        DateTime<Utc>: ::sqlx::types::Type<R::Database>,
    {
        fn from_row(__row: &'a R) -> ::sqlx::Result<Self> {
            let id: TodoId = __row.try_get("id")?;
            let title: String = __row.try_get("title")?;
            let description: Option<String> = __row.try_get("description")?;
            let status: Status = __row.try_get("status")?;
            let created_at: DateTime<Utc> = __row.try_get("created_at")?;
            let updated_at: DateTime<Utc> = __row.try_get("updated_at")?;
            ::std::result::Result::Ok(TodoSqlxRow {
                id,
                title,
                description,
                status,
                created_at,
                updated_at,
            })
        }
    }
    impl From<TodoSqlxRow> for Todo {
        fn from(value: TodoSqlxRow) -> Self {
            Self {
                id: value.id,
                title: value.title,
                description: value.description,
                status: value.status,
                created_at: value.created_at,
                updated_at: value.updated_at,
            }
        }
    }
    #[iden = "todo"]
    enum TodoSeaQueryIdent {
        Table,
        #[iden = "id"]
        Id,
        #[iden = "title"]
        Title,
        #[iden = "description"]
        Description,
        #[iden = "status"]
        Status,
        #[iden = "created_at"]
        CreatedAt,
        #[iden = "updated_at"]
        UpdatedAt,
    }
    impl sea_query::Iden for TodoSeaQueryIdent {
        fn prepare(&self, s: &mut dyn ::std::fmt::Write, q: sea_query::Quote) {
            s.write_fmt(format_args!("{0}", q.left())).unwrap();
            self.unquoted(s);
            s.write_fmt(format_args!("{0}", q.right())).unwrap();
        }
        fn unquoted(&self, s: &mut dyn ::std::fmt::Write) {
            match self {
                Self::Table => s.write_fmt(format_args!("{0}", "todo")).unwrap(),
                Self::Id => s.write_fmt(format_args!("{0}", "id")).unwrap(),
                Self::Title => s.write_fmt(format_args!("{0}", "title")).unwrap(),
                Self::Description => {
                    s.write_fmt(format_args!("{0}", "description")).unwrap()
                }
                Self::Status => s.write_fmt(format_args!("{0}", "status")).unwrap(),
                Self::CreatedAt => {
                    s.write_fmt(format_args!("{0}", "created_at")).unwrap()
                }
                Self::UpdatedAt => {
                    s.write_fmt(format_args!("{0}", "updated_at")).unwrap()
                }
            };
        }
    }
    impl lazybe::Entity for Todo {
        type Pk = TodoId;
        type Create = CreateTodo;
        type Update = UpdateTodo;
        type Replace = ReplaceTodo;
        fn entity_name() -> &'static str {
            "Todo"
        }
    }
    impl lazybe::TableEntity for Todo {
        type Row = TodoSqlxRow;
    }
    impl lazybe::query::GetQuery for Todo {
        fn get_query(id: Self::Pk) -> sea_query::SelectStatement {
            sea_query::Query::select()
                .columns([
                    TodoSeaQueryIdent::Id,
                    TodoSeaQueryIdent::Title,
                    TodoSeaQueryIdent::Description,
                    TodoSeaQueryIdent::Status,
                    TodoSeaQueryIdent::CreatedAt,
                    TodoSeaQueryIdent::UpdatedAt,
                ])
                .from(TodoSeaQueryIdent::Table)
                .cond_where(
                    sea_query::Cond::all()
                        .add(sea_query::Expr::col(TodoSeaQueryIdent::Id).eq(id)),
                )
                .to_owned()
        }
    }
    impl lazybe::query::ListQuery for Todo {
        fn list_query(
            filter: lazybe::filter::Filter<Self>,
        ) -> sea_query::SelectStatement {
            sea_query::Query::select()
                .columns([
                    TodoSeaQueryIdent::Id,
                    TodoSeaQueryIdent::Title,
                    TodoSeaQueryIdent::Description,
                    TodoSeaQueryIdent::Status,
                    TodoSeaQueryIdent::CreatedAt,
                    TodoSeaQueryIdent::UpdatedAt,
                ])
                .from(TodoSeaQueryIdent::Table)
                .cond_where(sea_query::Cond::all().add(filter))
                .to_owned()
        }
    }
    impl lazybe::query::CreateQuery for Todo {
        fn create_query(input: Self::Create) -> sea_query::InsertStatement {
            let now = sqlx::types::chrono::Utc::now();
            sea_query::Query::insert()
                .into_table(TodoSeaQueryIdent::Table)
                .columns([
                    TodoSeaQueryIdent::Id,
                    TodoSeaQueryIdent::Title,
                    TodoSeaQueryIdent::Description,
                    TodoSeaQueryIdent::Status,
                    TodoSeaQueryIdent::CreatedAt,
                    TodoSeaQueryIdent::UpdatedAt,
                ])
                .values_panic([
                    (fourty_two(&input).into()),
                    input.title.into(),
                    input.description.into(),
                    input.status.into(),
                    now.into(),
                    now.into(),
                ])
                .returning_all()
                .to_owned()
        }
    }
    impl lazybe::query::UpdateQuery for Todo {
        fn update_query(
            id: Self::Pk,
            input: Self::Update,
        ) -> sea_query::UpdateStatement {
            let now = sqlx::types::chrono::Utc::now();
            let mut values = Vec::new();
            if let Some(new_value) = input.title {
                values.push((TodoSeaQueryIdent::Title, new_value.into()));
            }
            if let Some(new_value) = input.description {
                values.push((TodoSeaQueryIdent::Description, new_value.into()));
            }
            if let Some(new_value) = input.status {
                values.push((TodoSeaQueryIdent::Status, new_value.into()));
            }
            values.push((TodoSeaQueryIdent::UpdatedAt, now.into()));
            sea_query::Query::update()
                .table(TodoSeaQueryIdent::Table)
                .values(values)
                .cond_where(
                    sea_query::Cond::all()
                        .add(sea_query::Expr::col(TodoSeaQueryIdent::Id).eq(id)),
                )
                .returning_all()
                .to_owned()
        }
    }
    impl lazybe::query::DeleteQuery for Todo {
        fn delete_query(id: Self::Pk) -> sea_query::DeleteStatement {
            sea_query::Query::delete()
                .from_table(TodoSeaQueryIdent::Table)
                .cond_where(
                    sea_query::Cond::all()
                        .add(sea_query::Expr::col(TodoSeaQueryIdent::Id).eq(id)),
                )
                .to_owned()
        }
    }
    impl lazybe::router::Routable for Todo {
        fn entity_path() -> &'static str {
            "/todos/{id}"
        }
        fn entity_collection_path() -> &'static str {
            "/todos"
        }
    }
    impl lazybe::router::EntityCollectionApi for Todo {
        type Resp = Vec<Self>;
        type Query = ();
        fn page_response(page: lazybe::page::Page<Self>) -> Self::Resp {
            page.data
        }
        fn page_input(_input: &Self::Query) -> Option<lazybe::page::PaginationInput> {
            None
        }
        fn filter_input(_input: &Self::Query) -> lazybe::filter::Filter<Self> {
            lazybe::filter::Filter::empty()
        }
        fn sort_input(_input: &Self::Query) -> lazybe::sort::Sort<Self> {
            lazybe::sort::Sort::new([TodoSort::id().asc()])
        }
    }
    impl lazybe::router::ValidationHook for Todo {}
    impl utoipa::__dev::ComposeSchema for Todo {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            {
                let mut object = utoipa::openapi::ObjectBuilder::new();
                object = object
                    .property(
                        "id",
                        utoipa::openapi::schema::RefBuilder::new()
                            .ref_location_from_schema_name(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}", <TodoId as utoipa::ToSchema>::name()),
                                    );
                                    res
                                }),
                            ),
                    )
                    .required("id");
                object = object
                    .property(
                        "title",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            ),
                    )
                    .required("title");
                object = object
                    .property(
                        "description",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::String,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            }),
                    );
                object = object
                    .property(
                        "status",
                        utoipa::openapi::schema::RefBuilder::new()
                            .ref_location_from_schema_name(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}", <Status as utoipa::ToSchema>::name()),
                                    );
                                    res
                                }),
                            ),
                    )
                    .required("status");
                object = object
                    .property(
                        "created_at",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            )
                            .format(
                                Some(
                                    utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                        utoipa::openapi::schema::KnownFormat::DateTime,
                                    ),
                                ),
                            ),
                    )
                    .required("created_at");
                object = object
                    .property(
                        "updated_at",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            )
                            .format(
                                Some(
                                    utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                        utoipa::openapi::schema::KnownFormat::DateTime,
                                    ),
                                ),
                            ),
                    )
                    .required("updated_at");
                object
            }
                .into()
        }
    }
    impl utoipa::ToSchema for Todo {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("Todo")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas
                .extend([
                    (
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", <TodoId as utoipa::ToSchema>::name()),
                                );
                                res
                            }),
                        ),
                        <TodoId as utoipa::PartialSchema>::schema(),
                    ),
                    (
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", <Status as utoipa::ToSchema>::name()),
                                );
                                res
                            }),
                        ),
                        <Status as utoipa::PartialSchema>::schema(),
                    ),
                ]);
            <TodoId as utoipa::ToSchema>::schemas(schemas);
            <Status as utoipa::ToSchema>::schemas(schemas);
        }
    }
    pub enum Status {
        Todo,
        Doing,
        Done,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Status {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Status::Todo => "Todo",
                    Status::Doing => "Doing",
                    Status::Done => "Done",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Status {
        #[inline]
        fn clone(&self) -> Status {
            match self {
                Status::Todo => Status::Todo,
                Status::Doing => Status::Doing,
                Status::Done => Status::Done,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Status {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Status {
        #[inline]
        fn eq(&self, other: &Status) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Status {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Status {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Status::Todo => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Status",
                            0u32,
                            "Todo",
                        )
                    }
                    Status::Doing => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Status",
                            1u32,
                            "Doing",
                        )
                    }
                    Status::Done => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Status",
                            2u32,
                            "Done",
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Status {
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
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
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
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 3",
                                    ),
                                )
                            }
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
                            "Todo" => _serde::__private::Ok(__Field::__field0),
                            "Doing" => _serde::__private::Ok(__Field::__field1),
                            "Done" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
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
                            b"Todo" => _serde::__private::Ok(__Field::__field0),
                            b"Doing" => _serde::__private::Ok(__Field::__field1),
                            b"Done" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
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
                    marker: _serde::__private::PhantomData<Status>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Status;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum Status",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Status::Todo)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Status::Doing)
                            }
                            (__Field::__field2, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Status::Done)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["Todo", "Doing", "Done"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Status",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Status>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl std::fmt::Display for Status {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                Self::Todo => "Todo".to_string(),
                Self::Doing => "Doing".to_string(),
                Self::Done => "Done".to_string(),
            };
            f.write_fmt(format_args!("{0}", s))
        }
    }
    impl std::str::FromStr for Status {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parsed = match s {
                "Todo" => Self::Todo,
                "Doing" => Self::Doing,
                "Done" => Self::Done,
                s => {
                    Err(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Cannot parse enum value \'{0}\' to type {1}",
                                    s,
                                    "Status",
                                ),
                            );
                            res
                        }),
                    )?
                }
            };
            Ok(parsed)
        }
    }
    impl sea_query::Nullable for Status {
        fn null() -> sea_query::Value {
            sea_query::Value::String(None)
        }
    }
    impl From<Status> for sea_query::Value {
        fn from(value: Status) -> Self {
            sea_query::Value::String(Some(value.to_string().into()))
        }
    }
    impl<Db> sqlx::Type<Db> for Status
    where
        Db: sqlx::Database,
        String: sqlx::Type<Db>,
    {
        fn type_info() -> <Db as sqlx::Database>::TypeInfo {
            <String as sqlx::Type<Db>>::type_info()
        }
    }
    impl<'r, Db> sqlx::Decode<'r, Db> for Status
    where
        Db: sqlx::Database,
        for<'s> String: sqlx::Decode<'s, Db>,
    {
        fn decode(
            value: <Db as sqlx::Database>::ValueRef<'r>,
        ) -> Result<Self, sqlx::error::BoxDynError> {
            let value_str = <String as sqlx::Decode<Db>>::decode(value)?;
            let parsed = <Status as std::str::FromStr>::from_str(&value_str)?;
            Ok(parsed)
        }
    }
    impl utoipa::__dev::ComposeSchema for Status {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            utoipa::openapi::schema::Object::builder()
                .schema_type(
                    utoipa::openapi::schema::SchemaType::new(
                        utoipa::openapi::schema::Type::String,
                    ),
                )
                .enum_values::<[&str; 3usize], &str>(Some(["Todo", "Doing", "Done"]))
                .into()
        }
    }
    impl utoipa::ToSchema for Status {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("Status")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas.extend([]);
        }
    }
    fn fourty_two(_: &CreateTodo) -> TodoId {
        TodoId(42)
    }
}
