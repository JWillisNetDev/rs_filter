#[cfg(feature = "macros")]
pub use rs_filter_macros::*;

pub trait Filterable<Filter> {
    fn is_match(&self, filter: &Filter) -> bool;
}

#[derive(Default, Clone, PartialEq)]
pub enum EqFilter<T: PartialEq> {
    #[default]
    Any,
    None,
    Eq(T),
    Neq(T),
}

impl<T: PartialEq> Filterable<EqFilter<T>> for T {
    fn is_match(&self, filter: &EqFilter<T>) -> bool {
        match filter {
            EqFilter::Any => true,
            EqFilter::None => false,
            EqFilter::Eq(val) => self == val,
            EqFilter::Neq(val) => self != val,
        }
    }
}

impl<T: PartialEq> Filterable<EqFilter<T>> for Option<T> {
    fn is_match(&self, filter: &EqFilter<T>) -> bool {
        match filter {
            EqFilter::Any => true,
            EqFilter::None => self.is_none(),
            EqFilter::Eq(val) => self.as_ref().is_some_and(|inner| inner == val),
            EqFilter::Neq(val) => self.as_ref().is_some_and(|inner| inner != val),
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub enum OrdFilter<T: PartialOrd> {
    #[default]
    Any,
    None,
    Eq(T),
    Neq(T),
    Gt(T),
    Gte(T),
    Lt(T),
    Lte(T),
}

impl<T: PartialOrd> Filterable<OrdFilter<T>> for T {
    fn is_match(&self, filter: &OrdFilter<T>) -> bool {
        match filter {
            OrdFilter::Any => true,
            OrdFilter::None => false,
            OrdFilter::Eq(val) => self == val,
            OrdFilter::Neq(val) => self != val,
            OrdFilter::Gt(val) => self > val,
            OrdFilter::Gte(val) => self >= val,
            OrdFilter::Lt(val) => self < val,
            OrdFilter::Lte(val) => self <= val,
        }
    }
}

impl<T: PartialOrd> Filterable<OrdFilter<T>> for Option<T> {
    fn is_match(&self, filter: &OrdFilter<T>) -> bool {
        match filter {
            OrdFilter::Any => true,
            OrdFilter::None => self.is_none(),
            OrdFilter::Eq(val) => self.as_ref().is_some_and(|inner| inner == val),
            OrdFilter::Neq(val) => self.as_ref().is_some_and(|inner| inner != val),
            OrdFilter::Gt(val) => self.as_ref().is_some_and(|inner| inner > val),
            OrdFilter::Gte(val) => self.as_ref().is_some_and(|inner| inner >= val),
            OrdFilter::Lt(val) => self.as_ref().is_some_and(|inner| inner < val),
            OrdFilter::Lte(val) => self.as_ref().is_some_and(|inner| inner <= val),
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub enum StringFilter {
    #[default]
    Any,
    None,
    Eq(String),
    Neq(String),
    Contains(String),
    StartsWith(String),
    EndsWith(String),
}

trait NotOption {}
impl NotOption for String {}
impl NotOption for &str {}

impl<T: NotOption + AsRef<str>> Filterable<StringFilter> for T {
    fn is_match(&self, filter: &StringFilter) -> bool {
        match filter {
            StringFilter::Any => true,
            StringFilter::None => false,
            StringFilter::Eq(val) => self.as_ref() == val,
            StringFilter::Neq(val) => self.as_ref() != val,
            StringFilter::Contains(val) => self.as_ref().contains(val),
            StringFilter::StartsWith(val) => self.as_ref().starts_with(val),
            StringFilter::EndsWith(val) => self.as_ref().ends_with(val),
        }
    }
}

impl<T: AsRef<str>> Filterable<StringFilter> for Option<T> {
    fn is_match(&self, filter: &StringFilter) -> bool {
        match filter {
            StringFilter::Any => true,
            StringFilter::None => self.is_none(),
            StringFilter::Eq(val) => self.as_ref().is_some_and(|inner| inner.as_ref() == val),
            StringFilter::Neq(val) => self.as_ref().is_some_and(|inner| inner.as_ref() != val),
            StringFilter::Contains(val) => self
                .as_ref()
                .is_some_and(|inner| inner.as_ref().contains(val)),
            StringFilter::StartsWith(val) => self
                .as_ref()
                .is_some_and(|inner| inner.as_ref().starts_with(val)),
            StringFilter::EndsWith(val) => self
                .as_ref()
                .is_some_and(|inner| inner.as_ref().ends_with(val)),
        }
    }
}
