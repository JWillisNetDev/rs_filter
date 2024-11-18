use rs_filter::*;

struct TestStruct<'a> {
    a: &'a str,
    b: Option<&'a str>,
}

#[filter_for(TestStruct<'a>)]
struct TestFilter<'a> {
    a: StringFilter<&'a str>,
    b: StringFilter<&'a str>,
}

#[test]
fn it_matches_lifetimes() {
    let actual = TestStruct {
        a: "hello",
        b: None,
    };

    let filter = TestFilter {
        a: StringFilter::Contains("ell"),
        b: StringFilter::None,
    };

    assert!(actual.is_match(&filter));
}
