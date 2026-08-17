#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome<T> {
    Passed(T),
    Finding(Finding),
    CouldNotCheck(CouldNotCheck),
}

impl<T> Outcome<T> {
    pub fn is_passed(&self) -> bool {
        matches!(self, Self::Passed(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub detail: String,
}

impl Finding {
    pub fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouldNotCheck {
    pub detail: String,
}

impl CouldNotCheck {
    pub fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}
