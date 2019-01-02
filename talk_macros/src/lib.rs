extern crate imagine;

use proc_macro_hack::proc_macro_hack;

/// Add one to an expression.
#[proc_macro_hack]
pub use talk_macros_impl::add_one;
#[proc_macro_hack]
pub use talk_macros_impl::when;
