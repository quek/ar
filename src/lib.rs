#![feature(plugin_registrar, rustc_private, quote)]

extern crate aster;
extern crate syntax;
extern crate rustc_plugin;
extern crate mysql;

use rustc_plugin::Registry;

mod err;
mod db;
mod mcr;
mod drv;
mod naming;
pub mod query;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(syntax::parse::token::intern("derive_Ar"),
                                  syntax::ext::base::MultiDecorator(Box::new(drv::ar)));
    reg.register_macro("ar", mcr::ar);
}
