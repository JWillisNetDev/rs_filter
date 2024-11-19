use rs_filter::*;
use rs_filter_macros::filter_for;

mod lifetimes;

struct TestStruct {
    a: i32,
    b: Option<i32>,
    c: String,
    d: Option<String>,
    e: bool,
    f: Option<bool>,
}

#[filter_for(TestStruct)]
struct TestFilter {
    a: OrdFilter<i32>,
    b: OrdFilter<i32>,
    c: StringFilter,
    d: StringFilter,
    e: EqFilter<bool>,
    f: EqFilter<bool>,
}

// This is what should be produced by the attribute macro:
// impl Filterable<TestStructFilter> for TestStruct {
//     fn is_match(&self, filter: &TestStructFilter) -> bool {
//         self.a.is_match(&filter.a)
//             && self.b.is_match(&filter.b)
//             && self.c.is_match(&filter.c)
//             && self.d.is_match(&filter.d)
//             && self.e.is_match(&filter.e)
//             && self.f.is_match(&filter.f)
//     }
// }

#[test]
fn it_matches() {
    let actual = TestStruct {
        a: 123,
        b: None,
        c: "hello".to_string(),
        d: Some("world".to_string()),
        e: true,
        f: None,
    };

    let filter = TestFilter {
        a: OrdFilter::Eq(123),
        b: OrdFilter::None,
        c: StringFilter::Contains("ell".to_string()),
        d: StringFilter::EndsWith("rld".to_string()),
        e: EqFilter::Eq(true),
        f: EqFilter::None,
    };

    assert!(actual.is_match(&filter));
}
