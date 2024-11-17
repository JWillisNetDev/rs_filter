use rs_filter::{StringFilter, Filterable};

#[test]
fn test_string_filter_eq() {
    let filter = StringFilter::Eq("hello");

    let value = "hello";
    assert!(value.is_match(&filter));

    let value = "world";
    assert!(!value.is_match(&filter));

    let value = Some("hello");
    assert!(value.is_match(&filter));

    let value = Some("world");
    assert!(!value.is_match(&filter));

    let value = Option::<&str>::None;
    assert!(!value.is_match(&filter));
}

#[test]
fn test_string_filter_any() {
    let filter = StringFilter::Any;

    let value = "hello";
    assert!(value.is_match(&filter));

    let value = "world";
    assert!(value.is_match(&filter));

    let value = Some("hello");
    assert!(value.is_match(&filter));

    let value = Some("world");
    assert!(value.is_match(&filter));

    let value = Option::<&str>::None;
    assert!(value.is_match(&filter));
}

#[test]
fn test_string_filter_none() {
    let filter = StringFilter::None;

    let value = "hello";
    assert!(!value.is_match(&filter));

    let value = "world";
    assert!(!value.is_match(&filter));

    let value = Some("hello");
    assert!(!value.is_match(&filter));

    let value = Some("world");
    assert!(!value.is_match(&filter));

    let value = Option::<&str>::None;
    assert!(value.is_match(&filter));
}

#[test]
fn test_string_filter_contains() {
    let filter = StringFilter::Contains("hello");

    let value = "hello world";
    assert!(value.is_match(&filter));

    let value = "world";
    assert!(!value.is_match(&filter));

    let value = Some("hello world");
    assert!(value.is_match(&filter));

    let value = Some("world");
    assert!(!value.is_match(&filter));

    let value = Option::<&str>::None;
    assert!(!value.is_match(&filter));
}

#[test]
fn test_string_filter_starts_with() {
    let filter = StringFilter::StartsWith("hello");

    let value = "hello world";
    assert!(value.is_match(&filter));

    let value = "world";
    assert!(!value.is_match(&filter));

    let value = Some("hello world");
    assert!(value.is_match(&filter));

    let value = Some("world");
    assert!(!value.is_match(&filter));

    let value = Option::<&str>::None;
    assert!(!value.is_match(&filter));
}

#[test]
fn test_string_filter_ends_with() {
    let filter = StringFilter::EndsWith("world");

    let value = "hello world";
    assert!(value.is_match(&filter));

    let value = "hello";
    assert!(!value.is_match(&filter));

    let value = Some("hello world");
    assert!(value.is_match(&filter));

    let value = Some("hello");
    assert!(!value.is_match(&filter));

    let value = Option::<&str>::None;
    assert!(!value.is_match(&filter));
}