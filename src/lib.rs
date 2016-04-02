#![feature(plugin_registrar, rustc_private, quote)]

extern crate aster;
extern crate syntax;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::ast::{Ident, MetaItem, Item};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager, Annotatable};
use syntax::ext::build::AstBuilder; // trait for expr_usize
use syntax::parse::token;


pub fn expand_table(cx: &mut ExtCtxt,
                    span: Span,
                    meta_item: &MetaItem,
                    annotatable: &Annotatable,
                    push: &mut FnMut(Annotatable)) {

    // println!("cx -> {:?}", cx);
    println!("span -> {:?}", span);
    println!("meta_item -> {:?}", meta_item);
    println!("annotatable -> {:?}", annotatable);
    // println!("push -> {:?}", push);

    let item = match *annotatable {
        Annotatable::Item(ref item) => item,
        _ => {
            cx.span_err(meta_item.span,
                        "`#[derive(Serialize)]` may only be applied to structs and enums");
            return;
        }
    };

    let builder = aster::AstBuilder::new().span(span);

    // let impl_item = match serialize_item(cx, &builder, &item) {
    //    Ok(item) => item,
    //    Err(Error) => {
    //        // An error occured, but it should have been reported already.
    //        return;
    //    }
    // };
    //
    // push(Annotatable::Item(impl_item))

    let item = quote_item!(cx,
        impl Foo {
            pub fn f1(&self) {
                println!("f1!!!!!!!!!!!!!!!!!!!!!!");
            }
        }
    );

    push(Annotatable::Item(item.unwrap()));


    push(Annotatable::Item(quote_item!(cx,
                    struct Bar { id: u64 }
        )
                               .unwrap()));
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(syntax::parse::token::intern("derive_Ar"),
                                  syntax::ext::base::MultiDecorator(Box::new(expand_table)));
}
