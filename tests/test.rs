#![feature(custom_derive, plugin)]
#![plugin(ar)]


#[derive(Ar, Debug)]
struct Foo {
    yid: u64,
    yname: String,
}


ar!{ Region }

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
}
