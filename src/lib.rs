#![feature(plugin_registrar, rustc_private, quote)]

extern crate aster;
extern crate syntax;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::ast;
use syntax::ast::{MetaItem, Item};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager, Annotatable};
use syntax::parse::token;
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;

mod naming;

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
    let struct_name = item.ident.name.as_str();
    let table_name = naming::table_name(&*struct_name);
    println!("table_name -> {:?}", table_name);

    let fname = builder.id("f1");

    let x = quote_item!(cx,
impl Foo {
    pub fn $fname(&self) {
        println!("f1!!!!!!!!!!!!!!!!!!!!!!");
    }
}
    );

    Ok(x.unwrap())
}

/// //////////////////////////////////////////////////////////////////////////

fn expand_rn(cx: &mut ExtCtxt, span: Span, args: &[ast::TokenTree]) -> Box<MacResult + 'static> {

    if args.len() != 1 {
        cx.span_err(span,
                    &format!("argument should be a single identifier, but got {} arguments",
                             args.len()));
        return DummyResult::any(span);
    }

    let ident = match args[0] {
        ast::TokenTree::Token(_, token::Ident(s, _)) => s,
        _ => {
            cx.span_err(span, "argument should be a single identifier");
            return DummyResult::any(span);
        }
    };
    println!("ident -> {}", ident);

    let builder = aster::AstBuilder::new().span(span);
    let x = builder.item()
                   .attr()
                   .word("derive_Debug")
                   .attr()
                   .word("derive_PartialEq")
                   .attr()
                   .word("derive_Eq")
                   .attr()
                   .word("derive_Hash")
                   .struct_(ident)
                   .field("id")
                   .ty()
                   .i32()
                   .field("name")
                   .ty()
                   .path()
                   .global()
                   .ids(&["std", "string", "String"])
                   .build()
                   .build();

    println!("x -> {:?}", x);

    MacEager::items(SmallVector::many(vec![x]))
}

/// //////////////////////////////////////////////////////////////////////////
#[plugin_registrar]

pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(syntax::parse::token::intern("derive_Ar"),
                                  syntax::ext::base::MultiDecorator(Box::new(expand_table)));
    reg.register_macro("ar", expand_rn);
}
