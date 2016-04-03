extern crate mysql as m;

use std::env;
use syntax::ptr::P;
use syntax::ast::{Item, StructField};
use aster;

use self::m::{Pool, PooledConn, Opts};
use self::m::from_value;
use self::m::error::Error;

pub struct Connection {
    pub connection: PooledConn,
}

pub fn connect() -> Result<Connection, Error> {
    let uri = env::var("MYSQL_URI")
                  .unwrap_or("mysql://root:@localhost:3307/outing_development".to_string());
    let opts = try!(Opts::from_url(&uri));
    let pool = try!(Pool::new(opts));
    Ok(Connection { connection: try!(pool.get_conn()) })
}

fn quote_column_name(name: &str) -> String {
    format!("`{}`", name.replace("`", "``"))
}

fn quote_table_name(name: &str) -> String {
    quote_column_name(name).replace(".", "`.`")
}

#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Column {
    field_name: String,
    field_type: String,
}

impl Table {
    pub fn build_ast(&self, builder: aster::AstBuilder, struct_name: &str) -> P<Item> {

        let builder = builder.item()
                             .attr()
                             .word("derive_Debug")
                             .attr()
                             .word("derive_PartialEq")
                             .attr()
                             .word("derive_Eq")
                             .attr()
                             .word("derive_Hash")
                             .struct_(struct_name);

        let fs: Vec<StructField> =
            self.columns
                .iter()
                .map(|column| {
                    let b = aster::struct_field::StructFieldBuilder::named(column.field_name
                                                                                 .clone());
                    let b = b.ty();
                    match column.field_type.as_ref() {
                        "int(11)" => b.i32(),
                        x if x.starts_with("varchar") => {
                            b.path()
                             .global()
                             .ids(&["std", "string", "String"])
                             .build()
                        }
                        _ => b.i32(), // TODO エラーにすべきかな
                    }
                })
                .collect();

        let builder = builder.with_fields(fs);

        builder.build()
    }
}

impl Connection {
    pub fn columns(&mut self, table_name: &str) -> Result<Table, Error> {
        let query = format!("SHOW FULL FIELDS FROM {}", quote_table_name(table_name));
        let query_result = try!(self.connection
                                    .prep_exec(query, ()));
        let idxs = query_result.column_indexes();
        let vec = query_result.map(|row| row.unwrap())
                              .map(|mut row| {
                                  let v = row.take(idxs["Field"]).unwrap();
                                  let nm = from_value::<String>(v);
                                  let v = row.take(idxs["Type"]).unwrap();
                                  let ty = from_value::<String>(v);
                                  Column {
                                      field_name: nm,
                                      field_type: ty,
                                  }
                              })
                              .collect();
        // println!("vec<row> -> {:?}", vec);
        Ok(Table {
            name: table_name.to_string(),
            columns: vec,
        })
    }
}
