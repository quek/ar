#![feature(custom_derive, plugin)]
#![plugin(ar)]


#[derive(Ar, Debug)]
struct Foo {
    id: u64,
    name: String,
}


#[test]
fn main() {
    let foo = Foo {
        id: 0,
        name: "ねこねこ".to_string(),
    };
    println!("ok -> {:?}", foo);
    foo.f1();

    let bar = Bar { id: 123 };
    println!("bar -> {}", bar.id);
}
