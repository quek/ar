use db;

use mysql::Row;
use err::Error;

pub trait Query {
    type Item;

    fn all() -> Result<Vec<Self::Item>, Error> {
        Ok(vec![])
    }

    fn first() -> Result<Option<Self::Item>, Error> {
        let mut cn = try!(db::connect());
        let sql = format!("select * from {} limit 1", Self::table_name());
        let mut rs = try!(cn.connection.prep_exec(sql, ()));
        match rs.next() {
            None => Ok(None),
            Some(row) => {
                row.map(|row| Some(Self::from_row(row)))
                   .map_err(Error::MySql)
            }
        }
    }

    fn table_name() -> &'static str;
    fn from_row(row: Row) -> Self::Item;
}
