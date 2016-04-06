use syntax::ast;
use syntax::ast::{MetaItem, Item};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ptr::P;
use aster;

use err::Error;

pub fn ar(cx: &mut ExtCtxt,
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
        Err(_) => return,
    };
    push(Annotatable::Item(impl_item));

    push(Annotatable::Item(quote_item!(cx,
                    struct Bar { id: u64 }
        )
                               .unwrap()));
}

fn make_item(cx: &ExtCtxt,
             builder: &aster::AstBuilder,
             _item: &Item)
             -> Result<P<ast::Item>, Error> {

    // println!("item -> {:?}", item);

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
