extern crate mysql as m;

use std::env;
use syntax::ptr::P;
use syntax::ast::{Ident, Item, StructField};
use aster;

use self::m::{Pool, PooledConn, Opts};
use self::m::from_value;
use self::m::error::Error;

use naming;

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

pub fn quote_column_name(name: &str) -> String {
    format!("`{}`", name.replace("`", "``"))
}

pub fn quote_table_name(name: &str) -> String {
    quote_column_name(name).replace(".", "`.`")
}

#[derive(Debug, PartialEq, Eq)]
pub struct TableInfo {
    pub ident: Ident,
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Column {
    pub field_name: String,
    pub field_type: String,
}

impl TableInfo {
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
    pub fn table_info(&mut self, ident: &Ident) -> Result<TableInfo, Error> {
        let table_name = &naming::table_name(&*ident.name.as_str());
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
        Ok(TableInfo {
            ident: ident.clone(),
            name: table_name.to_string(),
            columns: vec,
        })
    }
}
