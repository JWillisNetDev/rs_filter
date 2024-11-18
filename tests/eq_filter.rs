use rs_filter::{EqFilter, Filterable};

#[test]
fn test_eq_filter_eq() {
    let filter = EqFilter::Eq(123);

    let value = 123;
    assert!(value.is_match(&filter));

    let value = 321;
    assert!(!value.is_match(&filter));

    let value = Some(123);
    assert!(value.is_match(&filter));

    let value = Some(321);
    assert!(!value.is_match(&filter));

    let value = Option::<i32>::None;
    assert!(!value.is_match(&filter));
}

#[test]
fn test_eq_filter_any() {
    let filter = EqFilter::Any;

    let value = 123;
    assert!(value.is_match(&filter));

    let value = 321;
    assert!(value.is_match(&filter));

    let value = Some(123);
    assert!(value.is_match(&filter));

    let value = Some(321);
    assert!(value.is_match(&filter));

    let value = Option::<i32>::None;
    assert!(value.is_match(&filter));
}

#[test]
fn test_eq_filter_none() {
    let filter = EqFilter::None;

    let value = 123;
    assert!(!value.is_match(&filter));

    let value = 321;
    assert!(!value.is_match(&filter));

    let value = Some(123);
    assert!(!value.is_match(&filter));

    let value = Some(321);
    assert!(!value.is_match(&filter));

    let value = Option::<i32>::None;
    assert!(value.is_match(&filter));
}

#[test]
fn test_eq_filter_neq() {
    let filter = EqFilter::Neq(123);

    let value = 123;
    assert!(!value.is_match(&filter));

    let value = 321;
    assert!(value.is_match(&filter));

    let value = Some(123);
    assert!(!value.is_match(&filter));

    let value = Some(321);
    assert!(value.is_match(&filter));

    let value = Option::<i32>::None;
    assert!(!value.is_match(&filter));
}
