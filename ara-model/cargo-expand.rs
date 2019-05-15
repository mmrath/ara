pub(crate) mod schema {
    pub mod app_user {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::active;
            pub use super::columns::created_at;
            pub use super::columns::created_by;
            pub use super::columns::email;
            pub use super::columns::first_name;
            pub use super::columns::id;
            pub use super::columns::last_name;
            pub use super::columns::phone_number;
            pub use super::columns::updated_at;
            pub use super::columns::updated_by;
            pub use super::columns::username;
            pub use super::columns::version;
            pub use super::table as app_user;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (
            id,
            first_name,
            last_name,
            username,
            email,
            phone_number,
            active,
            created_at,
            created_by,
            updated_at,
            updated_by,
            version,
        ) = (
            id,
            first_name,
            last_name,
            username,
            email,
            phone_number,
            active,
            created_at,
            created_by,
            updated_at,
            updated_by,
            version,
        );
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (
            Int8,
            Text,
            Text,
            Text,
            Text,
            Nullable<Text>,
            Bool,
            Timestamptz,
            Text,
            Timestamptz,
            Text,
            Int4,
        );
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("app_user")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (
                id,
                first_name,
                last_name,
                username,
                email,
                phone_number,
                active,
                created_at,
                created_by,
                updated_at,
                updated_by,
                version,
            );
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (
                    id,
                    first_name,
                    last_name,
                    username,
                    email,
                    phone_number,
                    active,
                    created_at,
                    created_by,
                    updated_at,
                    updated_by,
                    version,
                )
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int8;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int8>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct first_name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for first_name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        first_name => {
                            let mut debug_trait_builder = f.debug_tuple("first_name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for first_name {
                #[inline]
                fn clone(&self) -> first_name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for first_name {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_first_name() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for first_name {
                    type QueryId = first_name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for first_name {
                #[inline]
                fn default() -> first_name {
                    first_name {}
                }
            }
            impl ::diesel::expression::Expression for first_name {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for first_name
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("first_name")
                }
            }
            impl SelectableExpression<table> for first_name {}
            impl<QS> AppearsOnTable<QS> for first_name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for first_name
            where
                first_name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for first_name
            where
                first_name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for first_name where
                first_name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for first_name where
                first_name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for first_name {}
            impl ::diesel::query_source::Column for first_name {
                type Table = table;
                const NAME: &'static str = "first_name";
            }
            impl<T> ::diesel::EqAll<T> for first_name
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<first_name, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct last_name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for last_name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        last_name => {
                            let mut debug_trait_builder = f.debug_tuple("last_name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for last_name {
                #[inline]
                fn clone(&self) -> last_name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for last_name {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_last_name() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for last_name {
                    type QueryId = last_name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for last_name {
                #[inline]
                fn default() -> last_name {
                    last_name {}
                }
            }
            impl ::diesel::expression::Expression for last_name {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for last_name
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("last_name")
                }
            }
            impl SelectableExpression<table> for last_name {}
            impl<QS> AppearsOnTable<QS> for last_name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for last_name
            where
                last_name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for last_name
            where
                last_name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for last_name where
                last_name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for last_name where
                last_name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for last_name {}
            impl ::diesel::query_source::Column for last_name {
                type Table = table;
                const NAME: &'static str = "last_name";
            }
            impl<T> ::diesel::EqAll<T> for last_name
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<last_name, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct username;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for username {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        username => {
                            let mut debug_trait_builder = f.debug_tuple("username");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for username {
                #[inline]
                fn clone(&self) -> username {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for username {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_username() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for username {
                    type QueryId = username;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for username {
                #[inline]
                fn default() -> username {
                    username {}
                }
            }
            impl ::diesel::expression::Expression for username {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for username
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("username")
                }
            }
            impl SelectableExpression<table> for username {}
            impl<QS> AppearsOnTable<QS> for username where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for username
            where
                username: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for username
            where
                username: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for username where
                username: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for username where
                username: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for username {}
            impl ::diesel::query_source::Column for username {
                type Table = table;
                const NAME: &'static str = "username";
            }
            impl<T> ::diesel::EqAll<T> for username
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<username, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct email;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for email {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        email => {
                            let mut debug_trait_builder = f.debug_tuple("email");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for email {
                #[inline]
                fn clone(&self) -> email {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for email {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_email() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for email {
                    type QueryId = email;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for email {
                #[inline]
                fn default() -> email {
                    email {}
                }
            }
            impl ::diesel::expression::Expression for email {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for email
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("email")
                }
            }
            impl SelectableExpression<table> for email {}
            impl<QS> AppearsOnTable<QS> for email where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for email
            where
                email: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for email
            where
                email: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for email where
                email: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for email where
                email: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for email {}
            impl ::diesel::query_source::Column for email {
                type Table = table;
                const NAME: &'static str = "email";
            }
            impl<T> ::diesel::EqAll<T> for email
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<email, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct phone_number;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for phone_number {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        phone_number => {
                            let mut debug_trait_builder = f.debug_tuple("phone_number");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for phone_number {
                #[inline]
                fn clone(&self) -> phone_number {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for phone_number {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_phone_number() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for phone_number {
                    type QueryId = phone_number;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for phone_number {
                #[inline]
                fn default() -> phone_number {
                    phone_number {}
                }
            }
            impl ::diesel::expression::Expression for phone_number {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for phone_number
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("phone_number")
                }
            }
            impl SelectableExpression<table> for phone_number {}
            impl<QS> AppearsOnTable<QS> for phone_number where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for phone_number
            where
                phone_number: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for phone_number
            where
                phone_number: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for phone_number where
                phone_number: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for phone_number where
                phone_number: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for phone_number {}
            impl ::diesel::query_source::Column for phone_number {
                type Table = table;
                const NAME: &'static str = "phone_number";
            }
            impl<T> ::diesel::EqAll<T> for phone_number
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<phone_number, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct active;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for active {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        active => {
                            let mut debug_trait_builder = f.debug_tuple("active");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for active {
                #[inline]
                fn clone(&self) -> active {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for active {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_active() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for active {
                    type QueryId = active;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for active {
                #[inline]
                fn default() -> active {
                    active {}
                }
            }
            impl ::diesel::expression::Expression for active {
                type SqlType = Bool;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for active
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("active")
                }
            }
            impl SelectableExpression<table> for active {}
            impl<QS> AppearsOnTable<QS> for active where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for active
            where
                active: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for active
            where
                active: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for active where
                active: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for active where
                active: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for active {}
            impl ::diesel::query_source::Column for active {
                type Table = table;
                const NAME: &'static str = "active";
            }
            impl<T> ::diesel::EqAll<T> for active
            where
                T: ::diesel::expression::AsExpression<Bool>,
                ::diesel::dsl::Eq<active, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct created_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for created_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        created_at => {
                            let mut debug_trait_builder = f.debug_tuple("created_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for created_at {
                #[inline]
                fn clone(&self) -> created_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for created_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_created_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for created_at {
                    type QueryId = created_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for created_at {
                #[inline]
                fn default() -> created_at {
                    created_at {}
                }
            }
            impl ::diesel::expression::Expression for created_at {
                type SqlType = Timestamptz;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for created_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("created_at")
                }
            }
            impl SelectableExpression<table> for created_at {}
            impl<QS> AppearsOnTable<QS> for created_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for created_at where
                created_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for created_at where
                created_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for created_at {}
            impl ::diesel::query_source::Column for created_at {
                type Table = table;
                const NAME: &'static str = "created_at";
            }
            impl<T> ::diesel::EqAll<T> for created_at
            where
                T: ::diesel::expression::AsExpression<Timestamptz>,
                ::diesel::dsl::Eq<created_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for created_at where Rhs : :: diesel :: expression :: AsExpression < < < created_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for created_at where Rhs : :: diesel :: expression :: AsExpression < < < created_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct created_by;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for created_by {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        created_by => {
                            let mut debug_trait_builder = f.debug_tuple("created_by");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for created_by {
                #[inline]
                fn clone(&self) -> created_by {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for created_by {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_created_by() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for created_by {
                    type QueryId = created_by;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for created_by {
                #[inline]
                fn default() -> created_by {
                    created_by {}
                }
            }
            impl ::diesel::expression::Expression for created_by {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for created_by
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("created_by")
                }
            }
            impl SelectableExpression<table> for created_by {}
            impl<QS> AppearsOnTable<QS> for created_by where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for created_by
            where
                created_by: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for created_by
            where
                created_by: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for created_by where
                created_by: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for created_by where
                created_by: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for created_by {}
            impl ::diesel::query_source::Column for created_by {
                type Table = table;
                const NAME: &'static str = "created_by";
            }
            impl<T> ::diesel::EqAll<T> for created_by
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<created_by, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct updated_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for updated_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        updated_at => {
                            let mut debug_trait_builder = f.debug_tuple("updated_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for updated_at {
                #[inline]
                fn clone(&self) -> updated_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for updated_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_updated_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for updated_at {
                    type QueryId = updated_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for updated_at {
                #[inline]
                fn default() -> updated_at {
                    updated_at {}
                }
            }
            impl ::diesel::expression::Expression for updated_at {
                type SqlType = Timestamptz;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for updated_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("updated_at")
                }
            }
            impl SelectableExpression<table> for updated_at {}
            impl<QS> AppearsOnTable<QS> for updated_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for updated_at
            where
                updated_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for updated_at
            where
                updated_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for updated_at where
                updated_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for updated_at where
                updated_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for updated_at {}
            impl ::diesel::query_source::Column for updated_at {
                type Table = table;
                const NAME: &'static str = "updated_at";
            }
            impl<T> ::diesel::EqAll<T> for updated_at
            where
                T: ::diesel::expression::AsExpression<Timestamptz>,
                ::diesel::dsl::Eq<updated_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for updated_at where Rhs : :: diesel :: expression :: AsExpression < < < updated_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for updated_at where Rhs : :: diesel :: expression :: AsExpression < < < updated_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct updated_by;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for updated_by {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        updated_by => {
                            let mut debug_trait_builder = f.debug_tuple("updated_by");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for updated_by {
                #[inline]
                fn clone(&self) -> updated_by {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for updated_by {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_updated_by() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for updated_by {
                    type QueryId = updated_by;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for updated_by {
                #[inline]
                fn default() -> updated_by {
                    updated_by {}
                }
            }
            impl ::diesel::expression::Expression for updated_by {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for updated_by
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("updated_by")
                }
            }
            impl SelectableExpression<table> for updated_by {}
            impl<QS> AppearsOnTable<QS> for updated_by where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for updated_by
            where
                updated_by: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for updated_by
            where
                updated_by: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for updated_by where
                updated_by: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for updated_by where
                updated_by: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for updated_by {}
            impl ::diesel::query_source::Column for updated_by {
                type Table = table;
                const NAME: &'static str = "updated_by";
            }
            impl<T> ::diesel::EqAll<T> for updated_by
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<updated_by, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct version;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for version {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        version => {
                            let mut debug_trait_builder = f.debug_tuple("version");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for version {
                #[inline]
                fn clone(&self) -> version {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for version {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_version() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for version {
                    type QueryId = version;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for version {
                #[inline]
                fn default() -> version {
                    version {}
                }
            }
            impl ::diesel::expression::Expression for version {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for version
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("version")
                }
            }
            impl SelectableExpression<table> for version {}
            impl<QS> AppearsOnTable<QS> for version where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for version
            where
                version: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for version
            where
                version: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for version where
                version: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for version where
                version: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for version {}
            impl ::diesel::query_source::Column for version {
                type Table = table;
                const NAME: &'static str = "version";
            }
            impl<T> ::diesel::EqAll<T> for version
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<version, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
        }
    }
    pub mod country {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::code;
            pub use super::columns::currency;
            pub use super::columns::dial_code;
            pub use super::columns::id;
            pub use super::columns::name;
            pub use super::table as country;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, code, name, dial_code, currency) =
            (id, code, name, dial_code, currency);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int4, Text, Text, Int2, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("country")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, code, name, dial_code, currency);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, code, name, dial_code, currency)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct code;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for code {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        code => {
                            let mut debug_trait_builder = f.debug_tuple("code");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for code {
                #[inline]
                fn clone(&self) -> code {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for code {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_code() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for code {
                    type QueryId = code;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for code {
                #[inline]
                fn default() -> code {
                    code {}
                }
            }
            impl ::diesel::expression::Expression for code {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for code
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("code")
                }
            }
            impl SelectableExpression<table> for code {}
            impl<QS> AppearsOnTable<QS> for code where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for code
            where
                code: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for code
            where
                code: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for code where
                code: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for code where
                code: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for code {}
            impl ::diesel::query_source::Column for code {
                type Table = table;
                const NAME: &'static str = "code";
            }
            impl<T> ::diesel::EqAll<T> for code
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<code, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        name => {
                            let mut debug_trait_builder = f.debug_tuple("name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for name {
                #[inline]
                fn clone(&self) -> name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for name {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_name() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for name {
                    type QueryId = name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for name {
                #[inline]
                fn default() -> name {
                    name {}
                }
            }
            impl ::diesel::expression::Expression for name {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for name
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("name")
                }
            }
            impl SelectableExpression<table> for name {}
            impl<QS> AppearsOnTable<QS> for name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for name
            where
                name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for name
            where
                name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for name where
                name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for name where
                name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for name {}
            impl ::diesel::query_source::Column for name {
                type Table = table;
                const NAME: &'static str = "name";
            }
            impl<T> ::diesel::EqAll<T> for name
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<name, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct dial_code;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for dial_code {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        dial_code => {
                            let mut debug_trait_builder = f.debug_tuple("dial_code");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for dial_code {
                #[inline]
                fn clone(&self) -> dial_code {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for dial_code {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_dial_code() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for dial_code {
                    type QueryId = dial_code;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for dial_code {
                #[inline]
                fn default() -> dial_code {
                    dial_code {}
                }
            }
            impl ::diesel::expression::Expression for dial_code {
                type SqlType = Int2;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for dial_code
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("dial_code")
                }
            }
            impl SelectableExpression<table> for dial_code {}
            impl<QS> AppearsOnTable<QS> for dial_code where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for dial_code
            where
                dial_code: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for dial_code
            where
                dial_code: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for dial_code where
                dial_code: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for dial_code where
                dial_code: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for dial_code {}
            impl ::diesel::query_source::Column for dial_code {
                type Table = table;
                const NAME: &'static str = "dial_code";
            }
            impl<T> ::diesel::EqAll<T> for dial_code
            where
                T: ::diesel::expression::AsExpression<Int2>,
                ::diesel::dsl::Eq<dial_code, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for dial_code where Rhs : :: diesel :: expression :: AsExpression < < < dial_code as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for dial_code where Rhs : :: diesel :: expression :: AsExpression < < < dial_code as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for dial_code where Rhs : :: diesel :: expression :: AsExpression < < < dial_code as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for dial_code where Rhs : :: diesel :: expression :: AsExpression < < < dial_code as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct currency;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for currency {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        currency => {
                            let mut debug_trait_builder = f.debug_tuple("currency");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for currency {
                #[inline]
                fn clone(&self) -> currency {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for currency {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_currency() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for currency {
                    type QueryId = currency;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for currency {
                #[inline]
                fn default() -> currency {
                    currency {}
                }
            }
            impl ::diesel::expression::Expression for currency {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for currency
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("currency")
                }
            }
            impl SelectableExpression<table> for currency {}
            impl<QS> AppearsOnTable<QS> for currency where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for currency
            where
                currency: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for currency
            where
                currency: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for currency where
                currency: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for currency where
                currency: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for currency {}
            impl ::diesel::query_source::Column for currency {
                type Table = table;
                const NAME: &'static str = "currency";
            }
            impl<T> ::diesel::EqAll<T> for currency
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<currency, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod currency {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::code;
            pub use super::columns::format;
            pub use super::columns::id;
            pub use super::columns::name;
            pub use super::columns::precision;
            pub use super::columns::symbol;
            pub use super::table as currency;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, code, symbol, name, precision, format) =
            (id, code, symbol, name, precision, format);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int4, Text, Nullable<Text>, Text, Int2, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("currency")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, code, symbol, name, precision, format);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, code, symbol, name, precision, format)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct code;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for code {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        code => {
                            let mut debug_trait_builder = f.debug_tuple("code");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for code {
                #[inline]
                fn clone(&self) -> code {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for code {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_code() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for code {
                    type QueryId = code;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for code {
                #[inline]
                fn default() -> code {
                    code {}
                }
            }
            impl ::diesel::expression::Expression for code {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for code
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("code")
                }
            }
            impl SelectableExpression<table> for code {}
            impl<QS> AppearsOnTable<QS> for code where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for code
            where
                code: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for code
            where
                code: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for code where
                code: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for code where
                code: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for code {}
            impl ::diesel::query_source::Column for code {
                type Table = table;
                const NAME: &'static str = "code";
            }
            impl<T> ::diesel::EqAll<T> for code
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<code, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct symbol;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for symbol {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        symbol => {
                            let mut debug_trait_builder = f.debug_tuple("symbol");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for symbol {
                #[inline]
                fn clone(&self) -> symbol {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for symbol {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_symbol() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for symbol {
                    type QueryId = symbol;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for symbol {
                #[inline]
                fn default() -> symbol {
                    symbol {}
                }
            }
            impl ::diesel::expression::Expression for symbol {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for symbol
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("symbol")
                }
            }
            impl SelectableExpression<table> for symbol {}
            impl<QS> AppearsOnTable<QS> for symbol where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for symbol
            where
                symbol: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for symbol
            where
                symbol: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for symbol where
                symbol: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for symbol where
                symbol: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for symbol {}
            impl ::diesel::query_source::Column for symbol {
                type Table = table;
                const NAME: &'static str = "symbol";
            }
            impl<T> ::diesel::EqAll<T> for symbol
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<symbol, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        name => {
                            let mut debug_trait_builder = f.debug_tuple("name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for name {
                #[inline]
                fn clone(&self) -> name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for name {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_name() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for name {
                    type QueryId = name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for name {
                #[inline]
                fn default() -> name {
                    name {}
                }
            }
            impl ::diesel::expression::Expression for name {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for name
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("name")
                }
            }
            impl SelectableExpression<table> for name {}
            impl<QS> AppearsOnTable<QS> for name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for name
            where
                name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for name
            where
                name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for name where
                name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for name where
                name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for name {}
            impl ::diesel::query_source::Column for name {
                type Table = table;
                const NAME: &'static str = "name";
            }
            impl<T> ::diesel::EqAll<T> for name
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<name, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct precision;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for precision {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        precision => {
                            let mut debug_trait_builder = f.debug_tuple("precision");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for precision {
                #[inline]
                fn clone(&self) -> precision {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for precision {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_precision() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for precision {
                    type QueryId = precision;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for precision {
                #[inline]
                fn default() -> precision {
                    precision {}
                }
            }
            impl ::diesel::expression::Expression for precision {
                type SqlType = Int2;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for precision
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("precision")
                }
            }
            impl SelectableExpression<table> for precision {}
            impl<QS> AppearsOnTable<QS> for precision where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for precision
            where
                precision: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for precision
            where
                precision: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for precision where
                precision: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for precision where
                precision: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for precision {}
            impl ::diesel::query_source::Column for precision {
                type Table = table;
                const NAME: &'static str = "precision";
            }
            impl<T> ::diesel::EqAll<T> for precision
            where
                T: ::diesel::expression::AsExpression<Int2>,
                ::diesel::dsl::Eq<precision, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for precision where Rhs : :: diesel :: expression :: AsExpression < < < precision as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for precision where Rhs : :: diesel :: expression :: AsExpression < < < precision as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for precision where Rhs : :: diesel :: expression :: AsExpression < < < precision as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for precision where Rhs : :: diesel :: expression :: AsExpression < < < precision as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct format;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for format {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        format => {
                            let mut debug_trait_builder = f.debug_tuple("format");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for format {
                #[inline]
                fn clone(&self) -> format {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for format {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_format() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for format {
                    type QueryId = format;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for format {
                #[inline]
                fn default() -> format {
                    format {}
                }
            }
            impl ::diesel::expression::Expression for format {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for format
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("format")
                }
            }
            impl SelectableExpression<table> for format {}
            impl<QS> AppearsOnTable<QS> for format where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for format
            where
                format: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for format
            where
                format: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for format where
                format: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for format where
                format: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for format {}
            impl ::diesel::query_source::Column for format {
                type Table = table;
                const NAME: &'static str = "format";
            }
            impl<T> ::diesel::EqAll<T> for format
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<format, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod date_format {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::c_format;
            pub use super::columns::date_picker_format;
            pub use super::columns::id;
            pub use super::columns::js_format;
            pub use super::table as date_format;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, c_format, date_picker_format, js_format) =
            (id, c_format, date_picker_format, js_format);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int4, Text, Text, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("date_format")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, c_format, date_picker_format, js_format);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, c_format, date_picker_format, js_format)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct c_format;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for c_format {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        c_format => {
                            let mut debug_trait_builder = f.debug_tuple("c_format");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for c_format {
                #[inline]
                fn clone(&self) -> c_format {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for c_format {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_c_format() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for c_format {
                    type QueryId = c_format;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for c_format {
                #[inline]
                fn default() -> c_format {
                    c_format {}
                }
            }
            impl ::diesel::expression::Expression for c_format {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for c_format
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("c_format")
                }
            }
            impl SelectableExpression<table> for c_format {}
            impl<QS> AppearsOnTable<QS> for c_format where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for c_format
            where
                c_format: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for c_format
            where
                c_format: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for c_format where
                c_format: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for c_format where
                c_format: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for c_format {}
            impl ::diesel::query_source::Column for c_format {
                type Table = table;
                const NAME: &'static str = "c_format";
            }
            impl<T> ::diesel::EqAll<T> for c_format
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<c_format, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct date_picker_format;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for date_picker_format {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        date_picker_format => {
                            let mut debug_trait_builder = f.debug_tuple("date_picker_format");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for date_picker_format {
                #[inline]
                fn clone(&self) -> date_picker_format {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for date_picker_format {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_date_picker_format() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for date_picker_format {
                    type QueryId = date_picker_format;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for date_picker_format {
                #[inline]
                fn default() -> date_picker_format {
                    date_picker_format {}
                }
            }
            impl ::diesel::expression::Expression for date_picker_format {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for date_picker_format
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("date_picker_format")
                }
            }
            impl SelectableExpression<table> for date_picker_format {}
            impl<QS> AppearsOnTable<QS> for date_picker_format where QS: AppearsInFromClause<table, Count = Once>
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for date_picker_format
            where
                date_picker_format: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for date_picker_format
            where
                date_picker_format: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for date_picker_format where
                date_picker_format: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for date_picker_format where
                date_picker_format:
                    SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for date_picker_format {}
            impl ::diesel::query_source::Column for date_picker_format {
                type Table = table;
                const NAME: &'static str = "date_picker_format";
            }
            impl<T> ::diesel::EqAll<T> for date_picker_format
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<date_picker_format, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct js_format;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for js_format {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        js_format => {
                            let mut debug_trait_builder = f.debug_tuple("js_format");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for js_format {
                #[inline]
                fn clone(&self) -> js_format {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for js_format {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_js_format() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for js_format {
                    type QueryId = js_format;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for js_format {
                #[inline]
                fn default() -> js_format {
                    js_format {}
                }
            }
            impl ::diesel::expression::Expression for js_format {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for js_format
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("js_format")
                }
            }
            impl SelectableExpression<table> for js_format {}
            impl<QS> AppearsOnTable<QS> for js_format where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for js_format
            where
                js_format: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for js_format
            where
                js_format: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for js_format where
                js_format: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for js_format where
                js_format: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for js_format {}
            impl ::diesel::query_source::Column for js_format {
                type Table = table;
                const NAME: &'static str = "js_format";
            }
            impl<T> ::diesel::EqAll<T> for js_format
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<js_format, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod datetime_format {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::c_format;
            pub use super::columns::id;
            pub use super::columns::js_format;
            pub use super::table as datetime_format;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, c_format, js_format) = (id, c_format, js_format);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int4, Text, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("datetime_format")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, c_format, js_format);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, c_format, js_format)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct c_format;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for c_format {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        c_format => {
                            let mut debug_trait_builder = f.debug_tuple("c_format");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for c_format {
                #[inline]
                fn clone(&self) -> c_format {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for c_format {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_c_format() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for c_format {
                    type QueryId = c_format;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for c_format {
                #[inline]
                fn default() -> c_format {
                    c_format {}
                }
            }
            impl ::diesel::expression::Expression for c_format {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for c_format
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("c_format")
                }
            }
            impl SelectableExpression<table> for c_format {}
            impl<QS> AppearsOnTable<QS> for c_format where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for c_format
            where
                c_format: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for c_format
            where
                c_format: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for c_format where
                c_format: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for c_format where
                c_format: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for c_format {}
            impl ::diesel::query_source::Column for c_format {
                type Table = table;
                const NAME: &'static str = "c_format";
            }
            impl<T> ::diesel::EqAll<T> for c_format
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<c_format, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct js_format;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for js_format {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        js_format => {
                            let mut debug_trait_builder = f.debug_tuple("js_format");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for js_format {
                #[inline]
                fn clone(&self) -> js_format {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for js_format {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_js_format() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for js_format {
                    type QueryId = js_format;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for js_format {
                #[inline]
                fn default() -> js_format {
                    js_format {}
                }
            }
            impl ::diesel::expression::Expression for js_format {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for js_format
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("js_format")
                }
            }
            impl SelectableExpression<table> for js_format {}
            impl<QS> AppearsOnTable<QS> for js_format where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for js_format
            where
                js_format: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for js_format
            where
                js_format: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for js_format where
                js_format: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for js_format where
                js_format: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for js_format {}
            impl ::diesel::query_source::Column for js_format {
                type Table = table;
                const NAME: &'static str = "js_format";
            }
            impl<T> ::diesel::EqAll<T> for js_format
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<js_format, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod language {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::id;
            pub use super::columns::locale;
            pub use super::columns::name;
            pub use super::table as language;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, name, locale) = (id, name, locale);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int4, Text, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("language")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, name, locale);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, name, locale)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        name => {
                            let mut debug_trait_builder = f.debug_tuple("name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for name {
                #[inline]
                fn clone(&self) -> name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for name {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_name() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for name {
                    type QueryId = name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for name {
                #[inline]
                fn default() -> name {
                    name {}
                }
            }
            impl ::diesel::expression::Expression for name {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for name
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("name")
                }
            }
            impl SelectableExpression<table> for name {}
            impl<QS> AppearsOnTable<QS> for name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for name
            where
                name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for name
            where
                name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for name where
                name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for name where
                name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for name {}
            impl ::diesel::query_source::Column for name {
                type Table = table;
                const NAME: &'static str = "name";
            }
            impl<T> ::diesel::EqAll<T> for name
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<name, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct locale;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for locale {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        locale => {
                            let mut debug_trait_builder = f.debug_tuple("locale");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for locale {
                #[inline]
                fn clone(&self) -> locale {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for locale {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_locale() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for locale {
                    type QueryId = locale;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for locale {
                #[inline]
                fn default() -> locale {
                    locale {}
                }
            }
            impl ::diesel::expression::Expression for locale {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for locale
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("locale")
                }
            }
            impl SelectableExpression<table> for locale {}
            impl<QS> AppearsOnTable<QS> for locale where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for locale
            where
                locale: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for locale
            where
                locale: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for locale where
                locale: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for locale where
                locale: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for locale {}
            impl ::diesel::query_source::Column for locale {
                type Table = table;
                const NAME: &'static str = "locale";
            }
            impl<T> ::diesel::EqAll<T> for locale
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<locale, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod notification {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::body_html;
            pub use super::columns::body_plain_text;
            pub use super::columns::created_at;
            pub use super::columns::from_address;
            pub use super::columns::id;
            pub use super::columns::notification_type;
            pub use super::columns::subject;
            pub use super::table as notification;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (
            id,
            created_at,
            subject,
            notification_type,
            from_address,
            body_html,
            body_plain_text,
        ) = (
            id,
            created_at,
            subject,
            notification_type,
            from_address,
            body_html,
            body_plain_text,
        );
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (
            Int8,
            Timestamptz,
            Text,
            Text,
            Nullable<Text>,
            Nullable<Text>,
            Nullable<Text>,
        );
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("notification")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (
                id,
                created_at,
                subject,
                notification_type,
                from_address,
                body_html,
                body_plain_text,
            );
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (
                    id,
                    created_at,
                    subject,
                    notification_type,
                    from_address,
                    body_html,
                    body_plain_text,
                )
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int8;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int8>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct created_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for created_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        created_at => {
                            let mut debug_trait_builder = f.debug_tuple("created_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for created_at {
                #[inline]
                fn clone(&self) -> created_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for created_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_created_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for created_at {
                    type QueryId = created_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for created_at {
                #[inline]
                fn default() -> created_at {
                    created_at {}
                }
            }
            impl ::diesel::expression::Expression for created_at {
                type SqlType = Timestamptz;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for created_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("created_at")
                }
            }
            impl SelectableExpression<table> for created_at {}
            impl<QS> AppearsOnTable<QS> for created_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for created_at where
                created_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for created_at where
                created_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for created_at {}
            impl ::diesel::query_source::Column for created_at {
                type Table = table;
                const NAME: &'static str = "created_at";
            }
            impl<T> ::diesel::EqAll<T> for created_at
            where
                T: ::diesel::expression::AsExpression<Timestamptz>,
                ::diesel::dsl::Eq<created_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for created_at where Rhs : :: diesel :: expression :: AsExpression < < < created_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for created_at where Rhs : :: diesel :: expression :: AsExpression < < < created_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct subject;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for subject {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        subject => {
                            let mut debug_trait_builder = f.debug_tuple("subject");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for subject {
                #[inline]
                fn clone(&self) -> subject {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for subject {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_subject() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for subject {
                    type QueryId = subject;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for subject {
                #[inline]
                fn default() -> subject {
                    subject {}
                }
            }
            impl ::diesel::expression::Expression for subject {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for subject
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("subject")
                }
            }
            impl SelectableExpression<table> for subject {}
            impl<QS> AppearsOnTable<QS> for subject where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for subject
            where
                subject: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for subject
            where
                subject: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for subject where
                subject: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for subject where
                subject: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for subject {}
            impl ::diesel::query_source::Column for subject {
                type Table = table;
                const NAME: &'static str = "subject";
            }
            impl<T> ::diesel::EqAll<T> for subject
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<subject, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct notification_type;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for notification_type {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        notification_type => {
                            let mut debug_trait_builder = f.debug_tuple("notification_type");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for notification_type {
                #[inline]
                fn clone(&self) -> notification_type {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for notification_type {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_notification_type() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for notification_type {
                    type QueryId = notification_type;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for notification_type {
                #[inline]
                fn default() -> notification_type {
                    notification_type {}
                }
            }
            impl ::diesel::expression::Expression for notification_type {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for notification_type
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("notification_type")
                }
            }
            impl SelectableExpression<table> for notification_type {}
            impl<QS> AppearsOnTable<QS> for notification_type where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for notification_type
            where
                notification_type: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for notification_type
            where
                notification_type: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for notification_type where
                notification_type: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for notification_type where
                notification_type:
                    SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for notification_type {}
            impl ::diesel::query_source::Column for notification_type {
                type Table = table;
                const NAME: &'static str = "notification_type";
            }
            impl<T> ::diesel::EqAll<T> for notification_type
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<notification_type, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct from_address;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for from_address {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        from_address => {
                            let mut debug_trait_builder = f.debug_tuple("from_address");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for from_address {
                #[inline]
                fn clone(&self) -> from_address {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for from_address {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_from_address() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for from_address {
                    type QueryId = from_address;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for from_address {
                #[inline]
                fn default() -> from_address {
                    from_address {}
                }
            }
            impl ::diesel::expression::Expression for from_address {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for from_address
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("from_address")
                }
            }
            impl SelectableExpression<table> for from_address {}
            impl<QS> AppearsOnTable<QS> for from_address where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for from_address
            where
                from_address: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for from_address
            where
                from_address: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for from_address where
                from_address: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for from_address where
                from_address: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for from_address {}
            impl ::diesel::query_source::Column for from_address {
                type Table = table;
                const NAME: &'static str = "from_address";
            }
            impl<T> ::diesel::EqAll<T> for from_address
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<from_address, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct body_html;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for body_html {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        body_html => {
                            let mut debug_trait_builder = f.debug_tuple("body_html");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for body_html {
                #[inline]
                fn clone(&self) -> body_html {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for body_html {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_body_html() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for body_html {
                    type QueryId = body_html;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for body_html {
                #[inline]
                fn default() -> body_html {
                    body_html {}
                }
            }
            impl ::diesel::expression::Expression for body_html {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for body_html
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("body_html")
                }
            }
            impl SelectableExpression<table> for body_html {}
            impl<QS> AppearsOnTable<QS> for body_html where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for body_html
            where
                body_html: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for body_html
            where
                body_html: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for body_html where
                body_html: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for body_html where
                body_html: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for body_html {}
            impl ::diesel::query_source::Column for body_html {
                type Table = table;
                const NAME: &'static str = "body_html";
            }
            impl<T> ::diesel::EqAll<T> for body_html
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<body_html, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct body_plain_text;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for body_plain_text {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        body_plain_text => {
                            let mut debug_trait_builder = f.debug_tuple("body_plain_text");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for body_plain_text {
                #[inline]
                fn clone(&self) -> body_plain_text {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for body_plain_text {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_body_plain_text() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for body_plain_text {
                    type QueryId = body_plain_text;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for body_plain_text {
                #[inline]
                fn default() -> body_plain_text {
                    body_plain_text {}
                }
            }
            impl ::diesel::expression::Expression for body_plain_text {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for body_plain_text
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("body_plain_text")
                }
            }
            impl SelectableExpression<table> for body_plain_text {}
            impl<QS> AppearsOnTable<QS> for body_plain_text where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for body_plain_text
            where
                body_plain_text: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for body_plain_text
            where
                body_plain_text: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for body_plain_text where
                body_plain_text: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for body_plain_text where
                body_plain_text: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for body_plain_text {}
            impl ::diesel::query_source::Column for body_plain_text {
                type Table = table;
                const NAME: &'static str = "body_plain_text";
            }
            impl<T> ::diesel::EqAll<T> for body_plain_text
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<body_plain_text, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod notification_attachment {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::data;
            pub use super::columns::id;
            pub use super::columns::name;
            pub use super::columns::notification_id;
            pub use super::table as notification_attachment;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, notification_id, name, data) =
            (id, notification_id, name, data);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int8, Int8, Text, Bytea);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("notification_attachment")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, notification_id, name, data);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, notification_id, name, data)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int8;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int8>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct notification_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for notification_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        notification_id => {
                            let mut debug_trait_builder = f.debug_tuple("notification_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for notification_id {
                #[inline]
                fn clone(&self) -> notification_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for notification_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_notification_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for notification_id {
                    type QueryId = notification_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for notification_id {
                #[inline]
                fn default() -> notification_id {
                    notification_id {}
                }
            }
            impl ::diesel::expression::Expression for notification_id {
                type SqlType = Int8;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for notification_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("notification_id")
                }
            }
            impl SelectableExpression<table> for notification_id {}
            impl<QS> AppearsOnTable<QS> for notification_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for notification_id
            where
                notification_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for notification_id
            where
                notification_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for notification_id where
                notification_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for notification_id where
                notification_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for notification_id {}
            impl ::diesel::query_source::Column for notification_id {
                type Table = table;
                const NAME: &'static str = "notification_id";
            }
            impl<T> ::diesel::EqAll<T> for notification_id
            where
                T: ::diesel::expression::AsExpression<Int8>,
                ::diesel::dsl::Eq<notification_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for notification_id where Rhs : :: diesel :: expression :: AsExpression < < < notification_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for notification_id where Rhs : :: diesel :: expression :: AsExpression < < < notification_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for notification_id where Rhs : :: diesel :: expression :: AsExpression < < < notification_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for notification_id where Rhs : :: diesel :: expression :: AsExpression < < < notification_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        name => {
                            let mut debug_trait_builder = f.debug_tuple("name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for name {
                #[inline]
                fn clone(&self) -> name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for name {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_name() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for name {
                    type QueryId = name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for name {
                #[inline]
                fn default() -> name {
                    name {}
                }
            }
            impl ::diesel::expression::Expression for name {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for name
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("name")
                }
            }
            impl SelectableExpression<table> for name {}
            impl<QS> AppearsOnTable<QS> for name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for name
            where
                name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for name
            where
                name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for name where
                name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for name where
                name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for name {}
            impl ::diesel::query_source::Column for name {
                type Table = table;
                const NAME: &'static str = "name";
            }
            impl<T> ::diesel::EqAll<T> for name
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<name, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct data;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for data {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        data => {
                            let mut debug_trait_builder = f.debug_tuple("data");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for data {
                #[inline]
                fn clone(&self) -> data {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for data {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_data() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for data {
                    type QueryId = data;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for data {
                #[inline]
                fn default() -> data {
                    data {}
                }
            }
            impl ::diesel::expression::Expression for data {
                type SqlType = Bytea;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for data
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("data")
                }
            }
            impl SelectableExpression<table> for data {}
            impl<QS> AppearsOnTable<QS> for data where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for data
            where
                data: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for data
            where
                data: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for data where
                data: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for data where
                data: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for data {}
            impl ::diesel::query_source::Column for data {
                type Table = table;
                const NAME: &'static str = "data";
            }
            impl<T> ::diesel::EqAll<T> for data
            where
                T: ::diesel::expression::AsExpression<Bytea>,
                ::diesel::dsl::Eq<data, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod notification_recipient {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::address;
            pub use super::columns::id;
            pub use super::columns::name;
            pub use super::columns::notification_id;
            pub use super::columns::recipient_type;
            pub use super::table as notification_recipient;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, notification_id, recipient_type, name, address) =
            (id, notification_id, recipient_type, name, address);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int8, Int8, Text, Nullable<Text>, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("notification_recipient")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, notification_id, recipient_type, name, address);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, notification_id, recipient_type, name, address)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int8;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int8>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct notification_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for notification_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        notification_id => {
                            let mut debug_trait_builder = f.debug_tuple("notification_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for notification_id {
                #[inline]
                fn clone(&self) -> notification_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for notification_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_notification_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for notification_id {
                    type QueryId = notification_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for notification_id {
                #[inline]
                fn default() -> notification_id {
                    notification_id {}
                }
            }
            impl ::diesel::expression::Expression for notification_id {
                type SqlType = Int8;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for notification_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("notification_id")
                }
            }
            impl SelectableExpression<table> for notification_id {}
            impl<QS> AppearsOnTable<QS> for notification_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for notification_id
            where
                notification_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for notification_id
            where
                notification_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for notification_id where
                notification_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for notification_id where
                notification_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for notification_id {}
            impl ::diesel::query_source::Column for notification_id {
                type Table = table;
                const NAME: &'static str = "notification_id";
            }
            impl<T> ::diesel::EqAll<T> for notification_id
            where
                T: ::diesel::expression::AsExpression<Int8>,
                ::diesel::dsl::Eq<notification_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for notification_id where Rhs : :: diesel :: expression :: AsExpression < < < notification_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for notification_id where Rhs : :: diesel :: expression :: AsExpression < < < notification_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for notification_id where Rhs : :: diesel :: expression :: AsExpression < < < notification_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for notification_id where Rhs : :: diesel :: expression :: AsExpression < < < notification_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct recipient_type;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for recipient_type {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        recipient_type => {
                            let mut debug_trait_builder = f.debug_tuple("recipient_type");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for recipient_type {
                #[inline]
                fn clone(&self) -> recipient_type {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for recipient_type {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_recipient_type() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for recipient_type {
                    type QueryId = recipient_type;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for recipient_type {
                #[inline]
                fn default() -> recipient_type {
                    recipient_type {}
                }
            }
            impl ::diesel::expression::Expression for recipient_type {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for recipient_type
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("recipient_type")
                }
            }
            impl SelectableExpression<table> for recipient_type {}
            impl<QS> AppearsOnTable<QS> for recipient_type where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for recipient_type
            where
                recipient_type: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for recipient_type
            where
                recipient_type: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for recipient_type where
                recipient_type: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for recipient_type where
                recipient_type: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for recipient_type {}
            impl ::diesel::query_source::Column for recipient_type {
                type Table = table;
                const NAME: &'static str = "recipient_type";
            }
            impl<T> ::diesel::EqAll<T> for recipient_type
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<recipient_type, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        name => {
                            let mut debug_trait_builder = f.debug_tuple("name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for name {
                #[inline]
                fn clone(&self) -> name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for name {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_name() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for name {
                    type QueryId = name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for name {
                #[inline]
                fn default() -> name {
                    name {}
                }
            }
            impl ::diesel::expression::Expression for name {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for name
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("name")
                }
            }
            impl SelectableExpression<table> for name {}
            impl<QS> AppearsOnTable<QS> for name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for name
            where
                name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for name
            where
                name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for name where
                name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for name where
                name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for name {}
            impl ::diesel::query_source::Column for name {
                type Table = table;
                const NAME: &'static str = "name";
            }
            impl<T> ::diesel::EqAll<T> for name
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<name, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct address;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for address {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        address => {
                            let mut debug_trait_builder = f.debug_tuple("address");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for address {
                #[inline]
                fn clone(&self) -> address {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for address {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_address() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for address {
                    type QueryId = address;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for address {
                #[inline]
                fn default() -> address {
                    address {}
                }
            }
            impl ::diesel::expression::Expression for address {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for address
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("address")
                }
            }
            impl SelectableExpression<table> for address {}
            impl<QS> AppearsOnTable<QS> for address where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for address
            where
                address: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for address
            where
                address: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for address where
                address: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for address where
                address: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for address {}
            impl ::diesel::query_source::Column for address {
                type Table = table;
                const NAME: &'static str = "address";
            }
            impl<T> ::diesel::EqAll<T> for address
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<address, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod role {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::created_at;
            pub use super::columns::created_by;
            pub use super::columns::description;
            pub use super::columns::id;
            pub use super::columns::name;
            pub use super::columns::updated_at;
            pub use super::columns::updated_by;
            pub use super::columns::version;
            pub use super::table as role;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (
            id,
            name,
            description,
            created_at,
            created_by,
            updated_at,
            updated_by,
            version,
        ) = (
            id,
            name,
            description,
            created_at,
            created_by,
            updated_at,
            updated_by,
            version,
        );
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int4, Text, Text, Timestamptz, Text, Timestamptz, Text, Int4);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("role")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (
                id,
                name,
                description,
                created_at,
                created_by,
                updated_at,
                updated_by,
                version,
            );
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (
                    id,
                    name,
                    description,
                    created_at,
                    created_by,
                    updated_at,
                    updated_by,
                    version,
                )
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        name => {
                            let mut debug_trait_builder = f.debug_tuple("name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for name {
                #[inline]
                fn clone(&self) -> name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for name {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_name() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for name {
                    type QueryId = name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for name {
                #[inline]
                fn default() -> name {
                    name {}
                }
            }
            impl ::diesel::expression::Expression for name {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for name
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("name")
                }
            }
            impl SelectableExpression<table> for name {}
            impl<QS> AppearsOnTable<QS> for name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for name
            where
                name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for name
            where
                name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for name where
                name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for name where
                name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for name {}
            impl ::diesel::query_source::Column for name {
                type Table = table;
                const NAME: &'static str = "name";
            }
            impl<T> ::diesel::EqAll<T> for name
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<name, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct description;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for description {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        description => {
                            let mut debug_trait_builder = f.debug_tuple("description");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for description {
                #[inline]
                fn clone(&self) -> description {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for description {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_description() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for description {
                    type QueryId = description;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for description {
                #[inline]
                fn default() -> description {
                    description {}
                }
            }
            impl ::diesel::expression::Expression for description {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for description
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("description")
                }
            }
            impl SelectableExpression<table> for description {}
            impl<QS> AppearsOnTable<QS> for description where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for description
            where
                description: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for description
            where
                description: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for description where
                description: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for description where
                description: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for description {}
            impl ::diesel::query_source::Column for description {
                type Table = table;
                const NAME: &'static str = "description";
            }
            impl<T> ::diesel::EqAll<T> for description
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<description, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct created_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for created_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        created_at => {
                            let mut debug_trait_builder = f.debug_tuple("created_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for created_at {
                #[inline]
                fn clone(&self) -> created_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for created_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_created_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for created_at {
                    type QueryId = created_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for created_at {
                #[inline]
                fn default() -> created_at {
                    created_at {}
                }
            }
            impl ::diesel::expression::Expression for created_at {
                type SqlType = Timestamptz;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for created_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("created_at")
                }
            }
            impl SelectableExpression<table> for created_at {}
            impl<QS> AppearsOnTable<QS> for created_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for created_at where
                created_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for created_at where
                created_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for created_at {}
            impl ::diesel::query_source::Column for created_at {
                type Table = table;
                const NAME: &'static str = "created_at";
            }
            impl<T> ::diesel::EqAll<T> for created_at
            where
                T: ::diesel::expression::AsExpression<Timestamptz>,
                ::diesel::dsl::Eq<created_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for created_at where Rhs : :: diesel :: expression :: AsExpression < < < created_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for created_at where Rhs : :: diesel :: expression :: AsExpression < < < created_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct created_by;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for created_by {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        created_by => {
                            let mut debug_trait_builder = f.debug_tuple("created_by");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for created_by {
                #[inline]
                fn clone(&self) -> created_by {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for created_by {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_created_by() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for created_by {
                    type QueryId = created_by;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for created_by {
                #[inline]
                fn default() -> created_by {
                    created_by {}
                }
            }
            impl ::diesel::expression::Expression for created_by {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for created_by
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("created_by")
                }
            }
            impl SelectableExpression<table> for created_by {}
            impl<QS> AppearsOnTable<QS> for created_by where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for created_by
            where
                created_by: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for created_by
            where
                created_by: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for created_by where
                created_by: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for created_by where
                created_by: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for created_by {}
            impl ::diesel::query_source::Column for created_by {
                type Table = table;
                const NAME: &'static str = "created_by";
            }
            impl<T> ::diesel::EqAll<T> for created_by
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<created_by, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct updated_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for updated_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        updated_at => {
                            let mut debug_trait_builder = f.debug_tuple("updated_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for updated_at {
                #[inline]
                fn clone(&self) -> updated_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for updated_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_updated_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for updated_at {
                    type QueryId = updated_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for updated_at {
                #[inline]
                fn default() -> updated_at {
                    updated_at {}
                }
            }
            impl ::diesel::expression::Expression for updated_at {
                type SqlType = Timestamptz;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for updated_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("updated_at")
                }
            }
            impl SelectableExpression<table> for updated_at {}
            impl<QS> AppearsOnTable<QS> for updated_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for updated_at
            where
                updated_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for updated_at
            where
                updated_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for updated_at where
                updated_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for updated_at where
                updated_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for updated_at {}
            impl ::diesel::query_source::Column for updated_at {
                type Table = table;
                const NAME: &'static str = "updated_at";
            }
            impl<T> ::diesel::EqAll<T> for updated_at
            where
                T: ::diesel::expression::AsExpression<Timestamptz>,
                ::diesel::dsl::Eq<updated_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for updated_at where Rhs : :: diesel :: expression :: AsExpression < < < updated_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for updated_at where Rhs : :: diesel :: expression :: AsExpression < < < updated_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct updated_by;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for updated_by {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        updated_by => {
                            let mut debug_trait_builder = f.debug_tuple("updated_by");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for updated_by {
                #[inline]
                fn clone(&self) -> updated_by {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for updated_by {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_updated_by() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for updated_by {
                    type QueryId = updated_by;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for updated_by {
                #[inline]
                fn default() -> updated_by {
                    updated_by {}
                }
            }
            impl ::diesel::expression::Expression for updated_by {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for updated_by
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("updated_by")
                }
            }
            impl SelectableExpression<table> for updated_by {}
            impl<QS> AppearsOnTable<QS> for updated_by where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for updated_by
            where
                updated_by: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for updated_by
            where
                updated_by: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for updated_by where
                updated_by: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for updated_by where
                updated_by: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for updated_by {}
            impl ::diesel::query_source::Column for updated_by {
                type Table = table;
                const NAME: &'static str = "updated_by";
            }
            impl<T> ::diesel::EqAll<T> for updated_by
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<updated_by, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct version;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for version {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        version => {
                            let mut debug_trait_builder = f.debug_tuple("version");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for version {
                #[inline]
                fn clone(&self) -> version {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for version {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_version() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for version {
                    type QueryId = version;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for version {
                #[inline]
                fn default() -> version {
                    version {}
                }
            }
            impl ::diesel::expression::Expression for version {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for version
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("version")
                }
            }
            impl SelectableExpression<table> for version {}
            impl<QS> AppearsOnTable<QS> for version where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for version
            where
                version: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for version
            where
                version: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for version where
                version: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for version where
                version: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for version {}
            impl ::diesel::query_source::Column for version {
                type Table = table;
                const NAME: &'static str = "version";
            }
            impl<T> ::diesel::EqAll<T> for version
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<version, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
        }
    }
    pub mod permission {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::application;
            pub use super::columns::authority;
            pub use super::columns::description;
            pub use super::columns::id;
            pub use super::table as permission;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, application, authority, description) =
            (id, application, authority, description);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int4, Text, Text, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("permission")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, application, authority, description);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, application, authority, description)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct application;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for application {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        application => {
                            let mut debug_trait_builder = f.debug_tuple("application");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for application {
                #[inline]
                fn clone(&self) -> application {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for application {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_application() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for application {
                    type QueryId = application;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for application {
                #[inline]
                fn default() -> application {
                    application {}
                }
            }
            impl ::diesel::expression::Expression for application {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for application
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("application")
                }
            }
            impl SelectableExpression<table> for application {}
            impl<QS> AppearsOnTable<QS> for application where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for application
            where
                application: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for application
            where
                application: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for application where
                application: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for application where
                application: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for application {}
            impl ::diesel::query_source::Column for application {
                type Table = table;
                const NAME: &'static str = "application";
            }
            impl<T> ::diesel::EqAll<T> for application
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<application, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct authority;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for authority {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        authority => {
                            let mut debug_trait_builder = f.debug_tuple("authority");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for authority {
                #[inline]
                fn clone(&self) -> authority {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for authority {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_authority() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for authority {
                    type QueryId = authority;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for authority {
                #[inline]
                fn default() -> authority {
                    authority {}
                }
            }
            impl ::diesel::expression::Expression for authority {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for authority
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("authority")
                }
            }
            impl SelectableExpression<table> for authority {}
            impl<QS> AppearsOnTable<QS> for authority where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for authority
            where
                authority: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for authority
            where
                authority: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for authority where
                authority: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for authority where
                authority: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for authority {}
            impl ::diesel::query_source::Column for authority {
                type Table = table;
                const NAME: &'static str = "authority";
            }
            impl<T> ::diesel::EqAll<T> for authority
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<authority, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct description;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for description {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        description => {
                            let mut debug_trait_builder = f.debug_tuple("description");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for description {
                #[inline]
                fn clone(&self) -> description {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for description {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_description() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for description {
                    type QueryId = description;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for description {
                #[inline]
                fn default() -> description {
                    description {}
                }
            }
            impl ::diesel::expression::Expression for description {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for description
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("description")
                }
            }
            impl SelectableExpression<table> for description {}
            impl<QS> AppearsOnTable<QS> for description where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for description
            where
                description: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for description
            where
                description: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for description where
                description: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for description where
                description: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for description {}
            impl ::diesel::query_source::Column for description {
                type Table = table;
                const NAME: &'static str = "description";
            }
            impl<T> ::diesel::EqAll<T> for description
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<description, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod role_permission {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::permission_id;
            pub use super::columns::role_id;
            pub use super::table as role_permission;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (role_id, permission_id) = (role_id, permission_id);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int4, Int4);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("role_permission")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (role_id, permission_id);
            type AllColumns = (role_id, permission_id);
            fn primary_key(&self) -> Self::PrimaryKey {
                (role_id, permission_id)
            }
            fn all_columns() -> Self::AllColumns {
                (role_id, permission_id)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct role_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for role_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        role_id => {
                            let mut debug_trait_builder = f.debug_tuple("role_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for role_id {
                #[inline]
                fn clone(&self) -> role_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for role_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_role_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for role_id {
                    type QueryId = role_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for role_id {
                #[inline]
                fn default() -> role_id {
                    role_id {}
                }
            }
            impl ::diesel::expression::Expression for role_id {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for role_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("role_id")
                }
            }
            impl SelectableExpression<table> for role_id {}
            impl<QS> AppearsOnTable<QS> for role_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for role_id
            where
                role_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for role_id
            where
                role_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for role_id where
                role_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for role_id where
                role_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for role_id {}
            impl ::diesel::query_source::Column for role_id {
                type Table = table;
                const NAME: &'static str = "role_id";
            }
            impl<T> ::diesel::EqAll<T> for role_id
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<role_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for role_id where Rhs : :: diesel :: expression :: AsExpression < < < role_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for role_id where Rhs : :: diesel :: expression :: AsExpression < < < role_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for role_id where Rhs : :: diesel :: expression :: AsExpression < < < role_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for role_id where Rhs : :: diesel :: expression :: AsExpression < < < role_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct permission_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for permission_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        permission_id => {
                            let mut debug_trait_builder = f.debug_tuple("permission_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for permission_id {
                #[inline]
                fn clone(&self) -> permission_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for permission_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_permission_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for permission_id {
                    type QueryId = permission_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for permission_id {
                #[inline]
                fn default() -> permission_id {
                    permission_id {}
                }
            }
            impl ::diesel::expression::Expression for permission_id {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for permission_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("permission_id")
                }
            }
            impl SelectableExpression<table> for permission_id {}
            impl<QS> AppearsOnTable<QS> for permission_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for permission_id
            where
                permission_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for permission_id
            where
                permission_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for permission_id where
                permission_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for permission_id where
                permission_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for permission_id {}
            impl ::diesel::query_source::Column for permission_id {
                type Table = table;
                const NAME: &'static str = "permission_id";
            }
            impl<T> ::diesel::EqAll<T> for permission_id
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<permission_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for permission_id where Rhs : :: diesel :: expression :: AsExpression < < < permission_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for permission_id where Rhs : :: diesel :: expression :: AsExpression < < < permission_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for permission_id where Rhs : :: diesel :: expression :: AsExpression < < < permission_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for permission_id where Rhs : :: diesel :: expression :: AsExpression < < < permission_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
        }
    }
    pub mod timezone {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::gmt_offset;
            pub use super::columns::id;
            pub use super::columns::location;
            pub use super::columns::name;
            pub use super::table as timezone;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, name, gmt_offset, location) = (id, name, gmt_offset, location);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int4, Text, Text, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("timezone")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, name, gmt_offset, location);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, name, gmt_offset, location)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        name => {
                            let mut debug_trait_builder = f.debug_tuple("name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for name {
                #[inline]
                fn clone(&self) -> name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for name {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_name() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for name {
                    type QueryId = name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for name {
                #[inline]
                fn default() -> name {
                    name {}
                }
            }
            impl ::diesel::expression::Expression for name {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for name
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("name")
                }
            }
            impl SelectableExpression<table> for name {}
            impl<QS> AppearsOnTable<QS> for name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for name
            where
                name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for name
            where
                name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for name where
                name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for name where
                name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for name {}
            impl ::diesel::query_source::Column for name {
                type Table = table;
                const NAME: &'static str = "name";
            }
            impl<T> ::diesel::EqAll<T> for name
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<name, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct gmt_offset;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for gmt_offset {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        gmt_offset => {
                            let mut debug_trait_builder = f.debug_tuple("gmt_offset");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for gmt_offset {
                #[inline]
                fn clone(&self) -> gmt_offset {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for gmt_offset {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_gmt_offset() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for gmt_offset {
                    type QueryId = gmt_offset;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for gmt_offset {
                #[inline]
                fn default() -> gmt_offset {
                    gmt_offset {}
                }
            }
            impl ::diesel::expression::Expression for gmt_offset {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for gmt_offset
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("gmt_offset")
                }
            }
            impl SelectableExpression<table> for gmt_offset {}
            impl<QS> AppearsOnTable<QS> for gmt_offset where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for gmt_offset
            where
                gmt_offset: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for gmt_offset
            where
                gmt_offset: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for gmt_offset where
                gmt_offset: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for gmt_offset where
                gmt_offset: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for gmt_offset {}
            impl ::diesel::query_source::Column for gmt_offset {
                type Table = table;
                const NAME: &'static str = "gmt_offset";
            }
            impl<T> ::diesel::EqAll<T> for gmt_offset
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<gmt_offset, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct location;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for location {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        location => {
                            let mut debug_trait_builder = f.debug_tuple("location");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for location {
                #[inline]
                fn clone(&self) -> location {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for location {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_location() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for location {
                    type QueryId = location;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for location {
                #[inline]
                fn default() -> location {
                    location {}
                }
            }
            impl ::diesel::expression::Expression for location {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for location
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("location")
                }
            }
            impl SelectableExpression<table> for location {}
            impl<QS> AppearsOnTable<QS> for location where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for location
            where
                location: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for location
            where
                location: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for location where
                location: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for location where
                location: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for location {}
            impl ::diesel::query_source::Column for location {
                type Table = table;
                const NAME: &'static str = "location";
            }
            impl<T> ::diesel::EqAll<T> for location
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<location, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod user_credential {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::activated;
            pub use super::columns::activation_key;
            pub use super::columns::activation_key_expires_at;
            pub use super::columns::expires_at;
            pub use super::columns::id;
            pub use super::columns::invalid_attempts;
            pub use super::columns::locked;
            pub use super::columns::password_hash;
            pub use super::columns::reset_at;
            pub use super::columns::reset_key;
            pub use super::columns::reset_key_expires_at;
            pub use super::columns::updated_at;
            pub use super::columns::version;
            pub use super::table as user_credential;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (
            id,
            password_hash,
            expires_at,
            invalid_attempts,
            locked,
            activation_key,
            activation_key_expires_at,
            activated,
            reset_key,
            reset_key_expires_at,
            reset_at,
            updated_at,
            version,
        ) = (
            id,
            password_hash,
            expires_at,
            invalid_attempts,
            locked,
            activation_key,
            activation_key_expires_at,
            activated,
            reset_key,
            reset_key_expires_at,
            reset_at,
            updated_at,
            version,
        );
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (
            Int8,
            Nullable<Text>,
            Nullable<Timestamptz>,
            Int4,
            Bool,
            Nullable<Text>,
            Nullable<Timestamptz>,
            Bool,
            Nullable<Text>,
            Nullable<Timestamptz>,
            Nullable<Timestamptz>,
            Timestamptz,
            Int4,
        );
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("user_credential")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (
                id,
                password_hash,
                expires_at,
                invalid_attempts,
                locked,
                activation_key,
                activation_key_expires_at,
                activated,
                reset_key,
                reset_key_expires_at,
                reset_at,
                updated_at,
                version,
            );
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (
                    id,
                    password_hash,
                    expires_at,
                    invalid_attempts,
                    locked,
                    activation_key,
                    activation_key_expires_at,
                    activated,
                    reset_key,
                    reset_key_expires_at,
                    reset_at,
                    updated_at,
                    version,
                )
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int8;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int8>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct password_hash;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for password_hash {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        password_hash => {
                            let mut debug_trait_builder = f.debug_tuple("password_hash");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for password_hash {
                #[inline]
                fn clone(&self) -> password_hash {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for password_hash {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_password_hash() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for password_hash {
                    type QueryId = password_hash;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for password_hash {
                #[inline]
                fn default() -> password_hash {
                    password_hash {}
                }
            }
            impl ::diesel::expression::Expression for password_hash {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for password_hash
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("password_hash")
                }
            }
            impl SelectableExpression<table> for password_hash {}
            impl<QS> AppearsOnTable<QS> for password_hash where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for password_hash
            where
                password_hash: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for password_hash
            where
                password_hash: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for password_hash where
                password_hash: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for password_hash where
                password_hash: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for password_hash {}
            impl ::diesel::query_source::Column for password_hash {
                type Table = table;
                const NAME: &'static str = "password_hash";
            }
            impl<T> ::diesel::EqAll<T> for password_hash
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<password_hash, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct expires_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for expires_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        expires_at => {
                            let mut debug_trait_builder = f.debug_tuple("expires_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for expires_at {
                #[inline]
                fn clone(&self) -> expires_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for expires_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_expires_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for expires_at {
                    type QueryId = expires_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for expires_at {
                #[inline]
                fn default() -> expires_at {
                    expires_at {}
                }
            }
            impl ::diesel::expression::Expression for expires_at {
                type SqlType = Nullable<Timestamptz>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for expires_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("expires_at")
                }
            }
            impl SelectableExpression<table> for expires_at {}
            impl<QS> AppearsOnTable<QS> for expires_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for expires_at
            where
                expires_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for expires_at
            where
                expires_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for expires_at where
                expires_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for expires_at where
                expires_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for expires_at {}
            impl ::diesel::query_source::Column for expires_at {
                type Table = table;
                const NAME: &'static str = "expires_at";
            }
            impl<T> ::diesel::EqAll<T> for expires_at
            where
                T: ::diesel::expression::AsExpression<Nullable<Timestamptz>>,
                ::diesel::dsl::Eq<expires_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for expires_at where Rhs : :: diesel :: expression :: AsExpression < < < expires_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for expires_at where Rhs : :: diesel :: expression :: AsExpression < < < expires_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct invalid_attempts;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for invalid_attempts {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        invalid_attempts => {
                            let mut debug_trait_builder = f.debug_tuple("invalid_attempts");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for invalid_attempts {
                #[inline]
                fn clone(&self) -> invalid_attempts {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for invalid_attempts {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_invalid_attempts() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for invalid_attempts {
                    type QueryId = invalid_attempts;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for invalid_attempts {
                #[inline]
                fn default() -> invalid_attempts {
                    invalid_attempts {}
                }
            }
            impl ::diesel::expression::Expression for invalid_attempts {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for invalid_attempts
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("invalid_attempts")
                }
            }
            impl SelectableExpression<table> for invalid_attempts {}
            impl<QS> AppearsOnTable<QS> for invalid_attempts where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for invalid_attempts
            where
                invalid_attempts: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for invalid_attempts
            where
                invalid_attempts: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for invalid_attempts where
                invalid_attempts: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for invalid_attempts where
                invalid_attempts:
                    SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for invalid_attempts {}
            impl ::diesel::query_source::Column for invalid_attempts {
                type Table = table;
                const NAME: &'static str = "invalid_attempts";
            }
            impl<T> ::diesel::EqAll<T> for invalid_attempts
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<invalid_attempts, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for invalid_attempts where Rhs : :: diesel :: expression :: AsExpression < < < invalid_attempts as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for invalid_attempts where Rhs : :: diesel :: expression :: AsExpression < < < invalid_attempts as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for invalid_attempts where Rhs : :: diesel :: expression :: AsExpression < < < invalid_attempts as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for invalid_attempts where Rhs : :: diesel :: expression :: AsExpression < < < invalid_attempts as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct locked;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for locked {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        locked => {
                            let mut debug_trait_builder = f.debug_tuple("locked");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for locked {
                #[inline]
                fn clone(&self) -> locked {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for locked {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_locked() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for locked {
                    type QueryId = locked;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for locked {
                #[inline]
                fn default() -> locked {
                    locked {}
                }
            }
            impl ::diesel::expression::Expression for locked {
                type SqlType = Bool;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for locked
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("locked")
                }
            }
            impl SelectableExpression<table> for locked {}
            impl<QS> AppearsOnTable<QS> for locked where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for locked
            where
                locked: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for locked
            where
                locked: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for locked where
                locked: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for locked where
                locked: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for locked {}
            impl ::diesel::query_source::Column for locked {
                type Table = table;
                const NAME: &'static str = "locked";
            }
            impl<T> ::diesel::EqAll<T> for locked
            where
                T: ::diesel::expression::AsExpression<Bool>,
                ::diesel::dsl::Eq<locked, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct activation_key;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for activation_key {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        activation_key => {
                            let mut debug_trait_builder = f.debug_tuple("activation_key");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for activation_key {
                #[inline]
                fn clone(&self) -> activation_key {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for activation_key {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_activation_key() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for activation_key {
                    type QueryId = activation_key;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for activation_key {
                #[inline]
                fn default() -> activation_key {
                    activation_key {}
                }
            }
            impl ::diesel::expression::Expression for activation_key {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for activation_key
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("activation_key")
                }
            }
            impl SelectableExpression<table> for activation_key {}
            impl<QS> AppearsOnTable<QS> for activation_key where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for activation_key
            where
                activation_key: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for activation_key
            where
                activation_key: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for activation_key where
                activation_key: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for activation_key where
                activation_key: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for activation_key {}
            impl ::diesel::query_source::Column for activation_key {
                type Table = table;
                const NAME: &'static str = "activation_key";
            }
            impl<T> ::diesel::EqAll<T> for activation_key
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<activation_key, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct activation_key_expires_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for activation_key_expires_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        activation_key_expires_at => {
                            let mut debug_trait_builder =
                                f.debug_tuple("activation_key_expires_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for activation_key_expires_at {
                #[inline]
                fn clone(&self) -> activation_key_expires_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for activation_key_expires_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_activation_key_expires_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for activation_key_expires_at {
                    type QueryId = activation_key_expires_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for activation_key_expires_at {
                #[inline]
                fn default() -> activation_key_expires_at {
                    activation_key_expires_at {}
                }
            }
            impl ::diesel::expression::Expression for activation_key_expires_at {
                type SqlType = Nullable<Timestamptz>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for activation_key_expires_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("activation_key_expires_at")
                }
            }
            impl SelectableExpression<table> for activation_key_expires_at {}
            impl<QS> AppearsOnTable<QS> for activation_key_expires_at where
                QS: AppearsInFromClause<table, Count = Once>
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for activation_key_expires_at
            where
                activation_key_expires_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for activation_key_expires_at
            where
                activation_key_expires_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for activation_key_expires_at where
                activation_key_expires_at:
                    SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for activation_key_expires_at where
                activation_key_expires_at:
                    SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for activation_key_expires_at {}
            impl ::diesel::query_source::Column for activation_key_expires_at {
                type Table = table;
                const NAME: &'static str = "activation_key_expires_at";
            }
            impl<T> ::diesel::EqAll<T> for activation_key_expires_at
            where
                T: ::diesel::expression::AsExpression<Nullable<Timestamptz>>,
                ::diesel::dsl::Eq<activation_key_expires_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for activation_key_expires_at where Rhs : :: diesel :: expression :: AsExpression < < < activation_key_expires_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for activation_key_expires_at where Rhs : :: diesel :: expression :: AsExpression < < < activation_key_expires_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct activated;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for activated {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        activated => {
                            let mut debug_trait_builder = f.debug_tuple("activated");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for activated {
                #[inline]
                fn clone(&self) -> activated {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for activated {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_activated() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for activated {
                    type QueryId = activated;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for activated {
                #[inline]
                fn default() -> activated {
                    activated {}
                }
            }
            impl ::diesel::expression::Expression for activated {
                type SqlType = Bool;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for activated
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("activated")
                }
            }
            impl SelectableExpression<table> for activated {}
            impl<QS> AppearsOnTable<QS> for activated where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for activated
            where
                activated: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for activated
            where
                activated: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for activated where
                activated: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for activated where
                activated: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for activated {}
            impl ::diesel::query_source::Column for activated {
                type Table = table;
                const NAME: &'static str = "activated";
            }
            impl<T> ::diesel::EqAll<T> for activated
            where
                T: ::diesel::expression::AsExpression<Bool>,
                ::diesel::dsl::Eq<activated, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct reset_key;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for reset_key {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        reset_key => {
                            let mut debug_trait_builder = f.debug_tuple("reset_key");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for reset_key {
                #[inline]
                fn clone(&self) -> reset_key {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for reset_key {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_reset_key() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for reset_key {
                    type QueryId = reset_key;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for reset_key {
                #[inline]
                fn default() -> reset_key {
                    reset_key {}
                }
            }
            impl ::diesel::expression::Expression for reset_key {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for reset_key
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("reset_key")
                }
            }
            impl SelectableExpression<table> for reset_key {}
            impl<QS> AppearsOnTable<QS> for reset_key where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for reset_key
            where
                reset_key: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for reset_key
            where
                reset_key: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for reset_key where
                reset_key: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for reset_key where
                reset_key: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for reset_key {}
            impl ::diesel::query_source::Column for reset_key {
                type Table = table;
                const NAME: &'static str = "reset_key";
            }
            impl<T> ::diesel::EqAll<T> for reset_key
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<reset_key, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct reset_key_expires_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for reset_key_expires_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        reset_key_expires_at => {
                            let mut debug_trait_builder = f.debug_tuple("reset_key_expires_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for reset_key_expires_at {
                #[inline]
                fn clone(&self) -> reset_key_expires_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for reset_key_expires_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_reset_key_expires_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for reset_key_expires_at {
                    type QueryId = reset_key_expires_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for reset_key_expires_at {
                #[inline]
                fn default() -> reset_key_expires_at {
                    reset_key_expires_at {}
                }
            }
            impl ::diesel::expression::Expression for reset_key_expires_at {
                type SqlType = Nullable<Timestamptz>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for reset_key_expires_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("reset_key_expires_at")
                }
            }
            impl SelectableExpression<table> for reset_key_expires_at {}
            impl<QS> AppearsOnTable<QS> for reset_key_expires_at where
                QS: AppearsInFromClause<table, Count = Once>
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for reset_key_expires_at
            where
                reset_key_expires_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for reset_key_expires_at
            where
                reset_key_expires_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for reset_key_expires_at where
                reset_key_expires_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for reset_key_expires_at where
                reset_key_expires_at:
                    SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for reset_key_expires_at {}
            impl ::diesel::query_source::Column for reset_key_expires_at {
                type Table = table;
                const NAME: &'static str = "reset_key_expires_at";
            }
            impl<T> ::diesel::EqAll<T> for reset_key_expires_at
            where
                T: ::diesel::expression::AsExpression<Nullable<Timestamptz>>,
                ::diesel::dsl::Eq<reset_key_expires_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for reset_key_expires_at where Rhs : :: diesel :: expression :: AsExpression < < < reset_key_expires_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for reset_key_expires_at where Rhs : :: diesel :: expression :: AsExpression < < < reset_key_expires_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct reset_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for reset_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        reset_at => {
                            let mut debug_trait_builder = f.debug_tuple("reset_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for reset_at {
                #[inline]
                fn clone(&self) -> reset_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for reset_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_reset_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for reset_at {
                    type QueryId = reset_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for reset_at {
                #[inline]
                fn default() -> reset_at {
                    reset_at {}
                }
            }
            impl ::diesel::expression::Expression for reset_at {
                type SqlType = Nullable<Timestamptz>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for reset_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("reset_at")
                }
            }
            impl SelectableExpression<table> for reset_at {}
            impl<QS> AppearsOnTable<QS> for reset_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for reset_at
            where
                reset_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for reset_at
            where
                reset_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for reset_at where
                reset_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for reset_at where
                reset_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for reset_at {}
            impl ::diesel::query_source::Column for reset_at {
                type Table = table;
                const NAME: &'static str = "reset_at";
            }
            impl<T> ::diesel::EqAll<T> for reset_at
            where
                T: ::diesel::expression::AsExpression<Nullable<Timestamptz>>,
                ::diesel::dsl::Eq<reset_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for reset_at where Rhs : :: diesel :: expression :: AsExpression < < < reset_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for reset_at where Rhs : :: diesel :: expression :: AsExpression < < < reset_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct updated_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for updated_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        updated_at => {
                            let mut debug_trait_builder = f.debug_tuple("updated_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for updated_at {
                #[inline]
                fn clone(&self) -> updated_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for updated_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_updated_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for updated_at {
                    type QueryId = updated_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for updated_at {
                #[inline]
                fn default() -> updated_at {
                    updated_at {}
                }
            }
            impl ::diesel::expression::Expression for updated_at {
                type SqlType = Timestamptz;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for updated_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("updated_at")
                }
            }
            impl SelectableExpression<table> for updated_at {}
            impl<QS> AppearsOnTable<QS> for updated_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for updated_at
            where
                updated_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for updated_at
            where
                updated_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for updated_at where
                updated_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for updated_at where
                updated_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for updated_at {}
            impl ::diesel::query_source::Column for updated_at {
                type Table = table;
                const NAME: &'static str = "updated_at";
            }
            impl<T> ::diesel::EqAll<T> for updated_at
            where
                T: ::diesel::expression::AsExpression<Timestamptz>,
                ::diesel::dsl::Eq<updated_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for updated_at where Rhs : :: diesel :: expression :: AsExpression < < < updated_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for updated_at where Rhs : :: diesel :: expression :: AsExpression < < < updated_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct version;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for version {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        version => {
                            let mut debug_trait_builder = f.debug_tuple("version");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for version {
                #[inline]
                fn clone(&self) -> version {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for version {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_version() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for version {
                    type QueryId = version;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for version {
                #[inline]
                fn default() -> version {
                    version {}
                }
            }
            impl ::diesel::expression::Expression for version {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for version
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("version")
                }
            }
            impl SelectableExpression<table> for version {}
            impl<QS> AppearsOnTable<QS> for version where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for version
            where
                version: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for version
            where
                version: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for version where
                version: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for version where
                version: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for version {}
            impl ::diesel::query_source::Column for version {
                type Table = table;
                const NAME: &'static str = "version";
            }
            impl<T> ::diesel::EqAll<T> for version
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<version, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for version where Rhs : :: diesel :: expression :: AsExpression < < < version as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
        }
    }
    pub mod user_role {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::role_id;
            pub use super::columns::user_id;
            pub use super::table as user_role;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (user_id, role_id) = (user_id, role_id);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int8, Int4);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("user_role")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (user_id, role_id);
            type AllColumns = (user_id, role_id);
            fn primary_key(&self) -> Self::PrimaryKey {
                (user_id, role_id)
            }
            fn all_columns() -> Self::AllColumns {
                (user_id, role_id)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct user_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for user_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        user_id => {
                            let mut debug_trait_builder = f.debug_tuple("user_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for user_id {
                #[inline]
                fn clone(&self) -> user_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for user_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_user_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for user_id {
                    type QueryId = user_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for user_id {
                #[inline]
                fn default() -> user_id {
                    user_id {}
                }
            }
            impl ::diesel::expression::Expression for user_id {
                type SqlType = Int8;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for user_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("user_id")
                }
            }
            impl SelectableExpression<table> for user_id {}
            impl<QS> AppearsOnTable<QS> for user_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for user_id
            where
                user_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for user_id
            where
                user_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for user_id where
                user_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for user_id where
                user_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for user_id {}
            impl ::diesel::query_source::Column for user_id {
                type Table = table;
                const NAME: &'static str = "user_id";
            }
            impl<T> ::diesel::EqAll<T> for user_id
            where
                T: ::diesel::expression::AsExpression<Int8>,
                ::diesel::dsl::Eq<user_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for user_id where Rhs : :: diesel :: expression :: AsExpression < < < user_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for user_id where Rhs : :: diesel :: expression :: AsExpression < < < user_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for user_id where Rhs : :: diesel :: expression :: AsExpression < < < user_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for user_id where Rhs : :: diesel :: expression :: AsExpression < < < user_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct role_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for role_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        role_id => {
                            let mut debug_trait_builder = f.debug_tuple("role_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for role_id {
                #[inline]
                fn clone(&self) -> role_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for role_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_role_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for role_id {
                    type QueryId = role_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for role_id {
                #[inline]
                fn default() -> role_id {
                    role_id {}
                }
            }
            impl ::diesel::expression::Expression for role_id {
                type SqlType = Int4;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for role_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("role_id")
                }
            }
            impl SelectableExpression<table> for role_id {}
            impl<QS> AppearsOnTable<QS> for role_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for role_id
            where
                role_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for role_id
            where
                role_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for role_id where
                role_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for role_id where
                role_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for role_id {}
            impl ::diesel::query_source::Column for role_id {
                type Table = table;
                const NAME: &'static str = "role_id";
            }
            impl<T> ::diesel::EqAll<T> for role_id
            where
                T: ::diesel::expression::AsExpression<Int4>,
                ::diesel::dsl::Eq<role_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for role_id where Rhs : :: diesel :: expression :: AsExpression < < < role_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for role_id where Rhs : :: diesel :: expression :: AsExpression < < < role_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for role_id where Rhs : :: diesel :: expression :: AsExpression < < < role_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for role_id where Rhs : :: diesel :: expression :: AsExpression < < < role_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
        }
    }
    pub mod user_token {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::created_at;
            pub use super::columns::expires_at;
            pub use super::columns::id;
            pub use super::columns::token;
            pub use super::columns::token_type;
            pub use super::columns::user_id;
            pub use super::table as user_token;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, user_id, token_type, token, created_at, expires_at) =
            (id, user_id, token_type, token, created_at, expires_at);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Int8, Int8, Text, Text, Timestamptz, Timestamptz);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("user_token")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, user_id, token_type, token, created_at, expires_at);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, user_id, token_type, token, created_at, expires_at)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Int8;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Int8>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct user_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for user_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        user_id => {
                            let mut debug_trait_builder = f.debug_tuple("user_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for user_id {
                #[inline]
                fn clone(&self) -> user_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for user_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_user_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for user_id {
                    type QueryId = user_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for user_id {
                #[inline]
                fn default() -> user_id {
                    user_id {}
                }
            }
            impl ::diesel::expression::Expression for user_id {
                type SqlType = Int8;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for user_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("user_id")
                }
            }
            impl SelectableExpression<table> for user_id {}
            impl<QS> AppearsOnTable<QS> for user_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for user_id
            where
                user_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for user_id
            where
                user_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for user_id where
                user_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for user_id where
                user_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for user_id {}
            impl ::diesel::query_source::Column for user_id {
                type Table = table;
                const NAME: &'static str = "user_id";
            }
            impl<T> ::diesel::EqAll<T> for user_id
            where
                T: ::diesel::expression::AsExpression<Int8>,
                ::diesel::dsl::Eq<user_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for user_id where Rhs : :: diesel :: expression :: AsExpression < < < user_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for user_id where Rhs : :: diesel :: expression :: AsExpression < < < user_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for user_id where Rhs : :: diesel :: expression :: AsExpression < < < user_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for user_id where Rhs : :: diesel :: expression :: AsExpression < < < user_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct token_type;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for token_type {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        token_type => {
                            let mut debug_trait_builder = f.debug_tuple("token_type");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for token_type {
                #[inline]
                fn clone(&self) -> token_type {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for token_type {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_token_type() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for token_type {
                    type QueryId = token_type;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for token_type {
                #[inline]
                fn default() -> token_type {
                    token_type {}
                }
            }
            impl ::diesel::expression::Expression for token_type {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for token_type
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("token_type")
                }
            }
            impl SelectableExpression<table> for token_type {}
            impl<QS> AppearsOnTable<QS> for token_type where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for token_type
            where
                token_type: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for token_type
            where
                token_type: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for token_type where
                token_type: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for token_type where
                token_type: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for token_type {}
            impl ::diesel::query_source::Column for token_type {
                type Table = table;
                const NAME: &'static str = "token_type";
            }
            impl<T> ::diesel::EqAll<T> for token_type
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<token_type, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct token;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for token {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        token => {
                            let mut debug_trait_builder = f.debug_tuple("token");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for token {
                #[inline]
                fn clone(&self) -> token {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for token {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_token() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for token {
                    type QueryId = token;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for token {
                #[inline]
                fn default() -> token {
                    token {}
                }
            }
            impl ::diesel::expression::Expression for token {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for token
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("token")
                }
            }
            impl SelectableExpression<table> for token {}
            impl<QS> AppearsOnTable<QS> for token where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for token
            where
                token: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for token
            where
                token: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for token where
                token: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for token where
                token: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for token {}
            impl ::diesel::query_source::Column for token {
                type Table = table;
                const NAME: &'static str = "token";
            }
            impl<T> ::diesel::EqAll<T> for token
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<token, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct created_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for created_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        created_at => {
                            let mut debug_trait_builder = f.debug_tuple("created_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for created_at {
                #[inline]
                fn clone(&self) -> created_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for created_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_created_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for created_at {
                    type QueryId = created_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for created_at {
                #[inline]
                fn default() -> created_at {
                    created_at {}
                }
            }
            impl ::diesel::expression::Expression for created_at {
                type SqlType = Timestamptz;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for created_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("created_at")
                }
            }
            impl SelectableExpression<table> for created_at {}
            impl<QS> AppearsOnTable<QS> for created_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for created_at where
                created_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for created_at where
                created_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for created_at {}
            impl ::diesel::query_source::Column for created_at {
                type Table = table;
                const NAME: &'static str = "created_at";
            }
            impl<T> ::diesel::EqAll<T> for created_at
            where
                T: ::diesel::expression::AsExpression<Timestamptz>,
                ::diesel::dsl::Eq<created_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for created_at where Rhs : :: diesel :: expression :: AsExpression < < < created_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for created_at where Rhs : :: diesel :: expression :: AsExpression < < < created_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct expires_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for expires_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        expires_at => {
                            let mut debug_trait_builder = f.debug_tuple("expires_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for expires_at {
                #[inline]
                fn clone(&self) -> expires_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for expires_at {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_expires_at() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for expires_at {
                    type QueryId = expires_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for expires_at {
                #[inline]
                fn default() -> expires_at {
                    expires_at {}
                }
            }
            impl ::diesel::expression::Expression for expires_at {
                type SqlType = Timestamptz;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for expires_at
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("expires_at")
                }
            }
            impl SelectableExpression<table> for expires_at {}
            impl<QS> AppearsOnTable<QS> for expires_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for expires_at
            where
                expires_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for expires_at
            where
                expires_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for expires_at where
                expires_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for expires_at where
                expires_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for expires_at {}
            impl ::diesel::query_source::Column for expires_at {
                type Table = table;
                const NAME: &'static str = "expires_at";
            }
            impl<T> ::diesel::EqAll<T> for expires_at
            where
                T: ::diesel::expression::AsExpression<Timestamptz>,
                ::diesel::dsl::Eq<expires_at, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for expires_at where Rhs : :: diesel :: expression :: AsExpression < < < expires_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for expires_at where Rhs : :: diesel :: expression :: AsExpression < < < expires_at as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
        }
    }
    impl ::diesel::JoinTo<notification::table> for notification_attachment::table {
        type FromClause = notification::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<notification_attachment::notification_id>,
            ::diesel::expression::nullable::Nullable<
                <notification::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: notification::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                notification_attachment::notification_id.nullable().eq(
                    <notification::table as ::diesel::query_source::Table>::primary_key(
                        &notification::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<notification_attachment::table> for notification::table {
        type FromClause = notification_attachment::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<notification_attachment::notification_id>,
            ::diesel::expression::nullable::Nullable<
                <notification::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: notification_attachment::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                notification_attachment::notification_id.nullable().eq(
                    <notification::table as ::diesel::query_source::Table>::primary_key(
                        &notification::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<notification::table> for notification_recipient::table {
        type FromClause = notification::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<notification_recipient::notification_id>,
            ::diesel::expression::nullable::Nullable<
                <notification::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: notification::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                notification_recipient::notification_id.nullable().eq(
                    <notification::table as ::diesel::query_source::Table>::primary_key(
                        &notification::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<notification_recipient::table> for notification::table {
        type FromClause = notification_recipient::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<notification_recipient::notification_id>,
            ::diesel::expression::nullable::Nullable<
                <notification::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: notification_recipient::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                notification_recipient::notification_id.nullable().eq(
                    <notification::table as ::diesel::query_source::Table>::primary_key(
                        &notification::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<role::table> for role_permission::table {
        type FromClause = role::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<role_permission::role_id>,
            ::diesel::expression::nullable::Nullable<
                <role::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: role::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                role_permission::role_id.nullable().eq(
                    <role::table as ::diesel::query_source::Table>::primary_key(&role::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<role_permission::table> for role::table {
        type FromClause = role_permission::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<role_permission::role_id>,
            ::diesel::expression::nullable::Nullable<
                <role::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: role_permission::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                role_permission::role_id.nullable().eq(
                    <role::table as ::diesel::query_source::Table>::primary_key(&role::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<permission::table> for role_permission::table {
        type FromClause = permission::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<role_permission::permission_id>,
            ::diesel::expression::nullable::Nullable<
                <permission::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: permission::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                role_permission::permission_id.nullable().eq(
                    <permission::table as ::diesel::query_source::Table>::primary_key(
                        &permission::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<role_permission::table> for permission::table {
        type FromClause = role_permission::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<role_permission::permission_id>,
            ::diesel::expression::nullable::Nullable<
                <permission::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: role_permission::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                role_permission::permission_id.nullable().eq(
                    <permission::table as ::diesel::query_source::Table>::primary_key(
                        &permission::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<app_user::table> for user_credential::table {
        type FromClause = app_user::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<user_credential::id>,
            ::diesel::expression::nullable::Nullable<
                <app_user::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: app_user::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                user_credential::id.nullable().eq(
                    <app_user::table as ::diesel::query_source::Table>::primary_key(
                        &app_user::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<user_credential::table> for app_user::table {
        type FromClause = user_credential::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<user_credential::id>,
            ::diesel::expression::nullable::Nullable<
                <app_user::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: user_credential::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                user_credential::id.nullable().eq(
                    <app_user::table as ::diesel::query_source::Table>::primary_key(
                        &app_user::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<app_user::table> for user_role::table {
        type FromClause = app_user::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<user_role::user_id>,
            ::diesel::expression::nullable::Nullable<
                <app_user::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: app_user::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                user_role::user_id.nullable().eq(
                    <app_user::table as ::diesel::query_source::Table>::primary_key(
                        &app_user::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<user_role::table> for app_user::table {
        type FromClause = user_role::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<user_role::user_id>,
            ::diesel::expression::nullable::Nullable<
                <app_user::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: user_role::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                user_role::user_id.nullable().eq(
                    <app_user::table as ::diesel::query_source::Table>::primary_key(
                        &app_user::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<role::table> for user_role::table {
        type FromClause = role::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<user_role::role_id>,
            ::diesel::expression::nullable::Nullable<
                <role::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: role::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                user_role::role_id.nullable().eq(
                    <role::table as ::diesel::query_source::Table>::primary_key(&role::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<user_role::table> for role::table {
        type FromClause = user_role::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<user_role::role_id>,
            ::diesel::expression::nullable::Nullable<
                <role::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: user_role::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                user_role::role_id.nullable().eq(
                    <role::table as ::diesel::query_source::Table>::primary_key(&role::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<app_user::table> for user_token::table {
        type FromClause = app_user::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<user_token::user_id>,
            ::diesel::expression::nullable::Nullable<
                <app_user::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: app_user::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                user_token::user_id.nullable().eq(
                    <app_user::table as ::diesel::query_source::Table>::primary_key(
                        &app_user::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<user_token::table> for app_user::table {
        type FromClause = user_token::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<user_token::user_id>,
            ::diesel::expression::nullable::Nullable<
                <app_user::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: user_token::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                user_token::user_id.nullable().eq(
                    <app_user::table as ::diesel::query_source::Table>::primary_key(
                        &app_user::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for datetime_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for app_user::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for app_user::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for role_permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for user_credential::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<app_user::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for app_user::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for datetime_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for country::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for notification_recipient::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for role_permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for user_credential::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<country::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for country::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for datetime_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for currency::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for currency::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for role_permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for user_credential::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<currency::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for currency::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for datetime_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for date_format::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for date_format::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for role_permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for user_credential::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<date_format::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for date_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for datetime_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for datetime_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for datetime_format::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for datetime_format::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for datetime_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table>
        for role_permission::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table>
        for datetime_format::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for datetime_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for datetime_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table>
        for user_credential::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table>
        for datetime_format::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for datetime_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<datetime_format::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for datetime_format::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for language::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for language::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for role_permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for user_credential::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<language::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for language::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for notification::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for notification::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for role_permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for user_credential::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for notification::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for notification_attachment::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for role_permission::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for permission::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for timezone::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for user_credential::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for user_role::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_attachment::table>
        for user_token::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table>
        for notification_attachment::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for notification_recipient::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for role_permission::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for permission::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for timezone::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for user_credential::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for user_role::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<notification_recipient::table>
        for user_token::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table>
        for notification_recipient::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for role_permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for user_credential::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for role_permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for role_permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table>
        for user_credential::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table>
        for role_permission::table
    {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for role_permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<role_permission::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for role_permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for user_credential::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<permission::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for permission::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for user_credential::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<timezone::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for timezone::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for user_credential::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_credential::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for user_credential::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_role::table> for user_token::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<user_token::table> for user_role::table {
        type Count = ::diesel::query_source::Never;
    }
}
