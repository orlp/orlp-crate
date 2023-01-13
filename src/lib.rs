#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

mod ord_assign;
mod option_some_ext;

pub use ord_assign::OrdAssign;
pub use option_some_ext::{NoneError, OptionSomeExt};
