#[cfg(feature = "derive")]
pub mod derive;

pub trait Filterable<Filter> {
    fn is_match(&self, filter: &Filter) -> bool;
}

pub enum EqFilter<T: PartialEq> {
    Any,
    None,
    Eq(T),
    Neq(T),
}

impl<T: PartialEq> std::default::Default for EqFilter<T> {
    fn default() -> Self {
        EqFilter::Any
    }
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

pub enum OrdFilter<T: PartialOrd> {
    Any,
    None,
    Eq(T),
    Neq(T),
    Gt(T),
    Gte(T),
    Lt(T),
    Lte(T),
}

impl<T: PartialOrd> std::default::Default for OrdFilter<T> {
    fn default() -> Self {
        OrdFilter::Any
    }
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

pub enum StringFilter<T: AsRef<str>> {
    Any,
    None,
    Eq(T),
    Neq(T),
    Contains(T),
    StartsWith(T),
    EndsWith(T),
}

impl std::default::Default for StringFilter<String> {
    fn default() -> Self {
        StringFilter::Any
    }
}

impl<T: AsRef<str>> Filterable<StringFilter<T>> for T {
    fn is_match(&self, filter: &StringFilter<T>) -> bool {
        match filter {
            StringFilter::Any => true,
            StringFilter::None => false,
            StringFilter::Eq(val) => self.as_ref() == val.as_ref(),
            StringFilter::Neq(val) => self.as_ref() != val.as_ref(),
            StringFilter::Contains(val) => self.as_ref().contains(val.as_ref()),
            StringFilter::StartsWith(val) => self.as_ref().starts_with(val.as_ref()),
            StringFilter::EndsWith(val) => self.as_ref().ends_with(val.as_ref()),
        }
    }
}

impl<T: AsRef<str>> Filterable<StringFilter<T>> for Option<T> {
    fn is_match(&self, filter: &StringFilter<T>) -> bool {
        match filter {
            StringFilter::Any => true,
            StringFilter::None => self.is_none(),
            StringFilter::Eq(val) => self.as_ref().is_some_and(|inner| inner.as_ref() == val.as_ref()),
            StringFilter::Neq(val) => self.as_ref().is_some_and(|inner| inner.as_ref() != val.as_ref()),
            StringFilter::Contains(val) => self.as_ref().is_some_and(|inner| inner.as_ref().contains(val.as_ref())),
            StringFilter::StartsWith(val) => self.as_ref().is_some_and(|inner| inner.as_ref().starts_with(val.as_ref())),
            StringFilter::EndsWith(val) => self.as_ref().is_some_and(|inner| inner.as_ref().ends_with(val.as_ref())),
        }
    }
}