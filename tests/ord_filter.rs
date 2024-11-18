use rs_filter::{OrdFilter, Filterable};

#[test]
fn test_ord_filter_eq() {
    let filter = OrdFilter::Eq(123);

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
fn test_ord_filter_any() {
    let filter = OrdFilter::Any;

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
fn test_ord_filter_none() {
    let filter = OrdFilter::None;

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
fn test_ord_filter_gt_gte() {
    let filter = OrdFilter::Gt(123);

    let value = 123;
    assert!(!value.is_match(&filter));

    let value = Some(123);
    assert!(!value.is_match(&filter));

    let value = 321;
    assert!(value.is_match(&filter));

    let value = Some(321);
    assert!(value.is_match(&filter));

    let filter = OrdFilter::Gte(123);

    let value = 1;
    assert!(!value.is_match(&filter));

    let value = Some(1);
    assert!(!value.is_match(&filter));

    let value = 123;
    assert!(value.is_match(&filter));

    let value = Some(123);
    assert!(value.is_match(&filter));

    let value = 321;
    assert!(value.is_match(&filter));

    let value = Some(321);
    assert!(value.is_match(&filter));
}

#[test]
fn test_ord_filter_lt_lte() {
    let filter = OrdFilter::Lt(123);

    let value = 123;
    assert!(!value.is_match(&filter));

    let value = Some(123);
    assert!(!value.is_match(&filter));

    let value = 1;
    assert!(value.is_match(&filter));

    let value = Some(1);
    assert!(value.is_match(&filter));

    let value = 321;
    assert!(!value.is_match(&filter));

    let value = Some(321);
    assert!(!value.is_match(&filter));

    let filter = OrdFilter::Lte(123);

    let value = 123;
    assert!(value.is_match(&filter));

    let value = Some(123);
    assert!(value.is_match(&filter));
}