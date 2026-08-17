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
    pub exit_code: u8,
}

impl Finding {
    pub fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
            exit_code: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouldNotCheck {
    pub detail: String,
    pub exit_code: u8,
}

impl CouldNotCheck {
    pub fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
            exit_code: 2,
        }
    }

    pub fn with_exit_code(detail: impl Into<String>, exit_code: u8) -> Self {
        Self {
            detail: detail.into(),
            exit_code,
        }
    }
}
