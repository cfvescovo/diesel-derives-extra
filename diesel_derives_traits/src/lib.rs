use diesel::associations::HasTable;
use diesel::prelude::*;

pub trait Model<'a>
where
    &'a Self: Identifiable,
    Self: Sized + 'a,
{
    fn save(self, conn: &mut PgConnection) -> QueryResult<Self>;
    fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Self>>;
    fn find_one(
        conn: &mut PgConnection,
        id: <&'a Self as Identifiable>::Id,
    ) -> QueryResult<Option<Self>>;
    fn exists(conn: &mut PgConnection, id: <&'a Self as Identifiable>::Id) -> QueryResult<bool>;
    fn count_all(conn: &mut PgConnection) -> QueryResult<i64>;
    fn destroy(self, conn: &mut PgConnection) -> QueryResult<()>;
}

pub trait NewModel<'a, T>
where
    &'a T: HasTable,
    T: 'a,
    &'a Self: Insertable<<&'a T as HasTable>::Table>,
    Self: 'a,
{
    fn save(self, conn: &mut PgConnection) -> QueryResult<T>;
}
