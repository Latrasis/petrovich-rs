//! Deprecated types. Don't use them.

use super::{Case, Gender};

#[deprecated(
    since = "0.2",
    note = "Use free functions in the 'petrovich' module instead"
)]
pub struct Petrovich;

#[allow(deprecated)]
impl Petrovich {
    pub fn new() -> Petrovich {
        Petrovich
    }

    #[deprecated(since = "0.2", note = "Use petrovich::firstname function")]
    #[inline]
    pub fn firstname(
        &self,
        gender: Gender,
        name: &str,
        case: Case,
    ) -> Result<String, &'static str> {
        Ok(super::firstname(gender, name, case))
    }

    #[deprecated(since = "0.2", note = "Use petrovich::middlename function")]
    #[inline]
    pub fn middlename(
        &self,
        gender: Gender,
        name: &str,
        case: Case,
    ) -> Result<String, &'static str> {
        Ok(super::middlename(gender, name, case))
    }

    #[deprecated(since = "0.2", note = "Use petrovich::lastname function")]
    #[inline]
    pub fn lastname(&self, gender: Gender, name: &str, case: Case) -> Result<String, &'static str> {
        Ok(super::lastname(gender, name, case))
    }

    #[deprecated(since = "0.2", note = "Use petrovich::detect_gender function")]
    #[inline]
    pub fn detect_gender(middlename: &str) -> Gender {
        super::detect_gender(None, None, Some(middlename))
    }
}
