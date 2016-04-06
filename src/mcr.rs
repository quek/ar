use syntax::ast;
use syntax::ptr::P;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::parse::token;
use syntax::util::small_vector::SmallVector;
use aster;

use db;
use db::TableInfo;
use naming;

pub fn ar(cx: &mut ExtCtxt, span: Span, args: &[ast::TokenTree]) -> Box<MacResult + 'static> {

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

    let mut vec = SmallVector::zero();

    let mut c = db::connect().unwrap();
    let table_info = c.table_info(&ident).unwrap();

    let builder = aster::AstBuilder::new().span(span);
    let x = table_info.build_struct(builder, &*ident.name.as_str());
    // println!("x -> {:?}", x);
    vec.push(x);

    let x = impl_query(cx, &span, &table_info);
    vec.push(x);
    MacEager::items(vec)
}

fn impl_query(cx: &mut ExtCtxt, span: &Span, table_info: &TableInfo) -> P<ast::Item> {
    let ident = table_info.ident;
    let table_name = naming::table_name(&*ident.name.as_str());
    let from_row_body = table_info.build_from_row_body(cx, span);
    quote_item!(cx,
                impl Query for $ident {
                    type Item = $ident;

                    fn table_name() -> &'static str {
                        $table_name
                    }

                    fn from_row(row: Row) -> Self::Item {
                        $from_row_body
                    }
                })
        .unwrap()
}
