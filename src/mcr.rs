use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::parse::token;
use syntax::util::small_vector::SmallVector;
use aster;

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
