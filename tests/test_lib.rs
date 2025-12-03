use flat_error::FlatError;
use pretty_assertions::assert_eq;
use std::{
    any::type_name, error::Error, fmt::{Display, Formatter, Result as FmtResult}
};

// ------------------------------------------------------------------------------------------------
// Test Fixtures
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct MyError;

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "MyError!")
    }
}

impl Error for MyError {}

// ------------------------------------------------------------------------------------------------
// Integration Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_my_error() {
    assert_eq!(MyError.to_string(), "MyError!".to_string());
}

#[test]
fn test_my_error_as_flat() {
    let err = FlatError::from_any(&MyError);
    assert_eq!(err.to_string(), "MyError!".to_string());
}

#[test]
fn test_my_error_as_flat_alt() {
    let err = FlatError::from_any(&MyError);
    assert_eq!(
        format!("{:#}", err),
        "MyError! (original type: `test_lib::MyError`)".to_string()
    );
}

#[test]
fn test_my_error_type_name() {
    let err = FlatError::from_any(&MyError);
    assert_eq!(err.original_type_name(), "test_lib::MyError".to_string());
}

#[test]
fn test_new_without_source() {
    let err = FlatError::new(type_name::<u32>(), "The value `foo` is not value for u32");
    assert_eq!(err.original_type_name(), "u32".to_string());
    assert_eq!(
        err.to_string(),
        "The value `foo` is not value for u32".to_string()
    );
    assert_eq!(
        format!("{:#}", err),
        "The value `foo` is not value for u32 (original type: `u32`)".to_string()
    );
}