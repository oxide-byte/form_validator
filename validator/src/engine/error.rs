use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub code: Cow<'static, str>,
    pub message: Cow<'static, str>,
    pub params: BTreeMap<Cow<'static, str>, Cow<'static, str>>,
}

impl ValidationError {
    pub fn new(code: impl Into<Cow<'static, str>>, message: impl Into<Cow<'static, str>>) -> Self {
        Self { code: code.into(), message: message.into(), params: BTreeMap::new() }
    }

    pub fn with_param(mut self, key: impl Into<Cow<'static, str>>, value: impl Into<Cow<'static, str>>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }
}

impl core::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}