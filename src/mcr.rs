use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::parse::token;
use syntax::util::small_vector::SmallVector;
use aster;

use db;
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

    let mut c = db::connect().unwrap();
    let table = c.columns(&naming::table_name(&*ident.name.as_str())).unwrap();

    let builder = aster::AstBuilder::new().span(span);
    let x = table.build_ast(builder, &*ident.name.as_str());

    println!("x -> {:?}", x);

    MacEager::items(SmallVector::many(vec![x]))
}
