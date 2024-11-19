use rs_filter::*;

struct TestStruct<'a> {
    a: &'a str,
    b: Option<&'a str>,
}

#[filter_for(TestStruct<'a>)]
struct TestFilter {
    a: StringFilter,
    b: StringFilter,
}

#[test]
fn it_matches_lifetimes() {
    let actual = TestStruct {
        a: "hello",
        b: None,
    };

    let filter = TestFilter {
        a: StringFilter::Contains("ell".to_string()),
        b: StringFilter::None,
    };

    assert!(actual.is_match(&filter));
}
