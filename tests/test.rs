#![feature(custom_derive, plugin)]
#![plugin(ar)]

extern crate mysql;
extern crate ar;

use ar::query::Query;
use mysql::Row;

#[derive(Ar, Debug)]
struct Foo {
    yid: u64,
    yname: String,
}


ar!{ Region }
ar!{ Prefecture }

#[test]
fn main() {
    let foo = Foo {
        yid: 0,
        yname: "ねこねこ".to_string(),
    };
    println!("ok -> {:?}", foo);
    foo.f1();

    let bar = Bar { id: 123 };
    println!("bar -> {}", bar.id);

    let region = Region {
        id: 777,
        name: "Hello".to_string(),
        reading: "".to_string(),
        roman: "".to_string(),
        prefecture_id: 1,
    };
    println!("region -> {:?}", region);

    let region = Region::first().unwrap().unwrap();
    assert_eq!(region.id, 1);
    assert_eq!(region.name, "北海道･東北");
}
