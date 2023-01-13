use core::fmt;
use core::panic::Location;

/// The error type if [`OptionSomeExt::some`] is called on [`None`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoneError {
    location: &'static Location<'static>,
}

#[cfg(feature = "std")]
impl std::error::Error for NoneError {}

impl fmt::Display for NoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "some() was called on None value at {}", self.location)
    }
}

/// An extension trait that allows you to convert `Option<T>` into
/// `Result<T, NoneError>` simply by calling `opt.some()`.
///
/// Particularly useful with type-erased error handling such as
/// `Box<dyn Error>` or `anyhow::Error`. The error type contains the source
/// code location of where `some()` was erroneously called, for easy debugging.
pub trait OptionSomeExt {
    type Item;

    fn some(self) -> Result<Self::Item, NoneError>;
}

impl<T> OptionSomeExt for Option<T> {
    type Item = T;

    #[track_caller]
    fn some(self) -> Result<Self::Item, NoneError> {
        match self {
            Some(val) => Ok(val),
            None => Err(NoneError {
                location: Location::caller(),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::OptionSomeExt;

    #[test]
    fn test_some_ext() {
        let a = Some(42);
        assert!(a.some().is_ok());

        let b: Option<i32> = None;
        assert!(b.some().is_err());
    }
}
