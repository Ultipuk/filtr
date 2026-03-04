#![warn(clippy::all, rust_2018_idioms)]
#![allow(
    clippy::approx_constant,
    clippy::bool_to_int_with_if,
    clippy::enum_glob_use,
    clippy::imprecise_flops,
    clippy::indexing_slicing,
    clippy::let_underscore_untyped,
    clippy::match_same_arms,
    clippy::needless_pass_by_ref_mut,
    clippy::needless_pass_by_value,
    clippy::non_std_lazy_statics,
    clippy::semicolon_if_nothing_returned,
    clippy::single_match_else,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unnecessary_semicolon,
    clippy::useless_let_if_seq,
    clippy::wildcard_imports
)]

mod app;
pub use app::FilterApp;

mod bindings;
mod i18n;
mod models;
mod themes;
mod widgets;
