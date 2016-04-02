#![feature(plugin_registrar, rustc_private, quote)]

extern crate aster;
extern crate syntax;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::ast;
use syntax::ast::{Ident, MetaItem, Item};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager, Annotatable};
use syntax::ext::build::AstBuilder; // trait for expr_usize
use syntax::parse::token;
use syntax::ptr::P;

pub struct Error;


pub fn expand_table(cx: &mut ExtCtxt,
                    span: Span,
                    meta_item: &MetaItem,
                    annotatable: &Annotatable,
                    push: &mut FnMut(Annotatable)) {

    // println!("span -> {:?}", span);
    // println!("meta_item -> {:?}", meta_item);
    // println!("annotatable -> {:?}", annotatable);

    let item = match *annotatable {
        Annotatable::Item(ref item) => {
            match item.node {
                ast::ItemKind::Struct(_, _) => (),
                _ => {
                    cx.span_err(item.span, "`#[derive(Ar)]` may only be applied to structs");
                    return;
                }
            };
            item
        }
        _ => {
            cx.span_err(meta_item.span,
                        "`#[derive(Ar)]` may only be applied to structs");
            return;
        }
    };

    let builder = aster::AstBuilder::new().span(span);

    let impl_item = match make_item(cx, &builder, item) {
        Ok(item) => item,
        Err(Error) => return,
    };
    push(Annotatable::Item(impl_item));

    push(Annotatable::Item(quote_item!(cx,
                    struct Bar { id: u64 }
        )
                               .unwrap()));
}

fn make_item(cx: &ExtCtxt,
             builder: &aster::AstBuilder,
             item: &Item)
             -> Result<P<ast::Item>, Error> {

    println!("item -> {:?}", item);

    let x = quote_item!(cx,
impl Foo {
    pub fn f1(&self) {
        println!("f1!!!!!!!!!!!!!!!!!!!!!!");
    }
}
    );

    Ok(x.unwrap())
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(syntax::parse::token::intern("derive_Ar"),
                                  syntax::ext::base::MultiDecorator(Box::new(expand_table)));
}
