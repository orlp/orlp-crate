#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

mod option_some_ext;
mod ord_assign;

pub use option_some_ext::{NoneError, OptionSomeExt};
pub use ord_assign::OrdAssign;
