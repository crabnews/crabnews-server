use std::fmt::{self, Display, Formatter};

pub type CrabResult<T> = Result<T, CrabError>;

#[derive(Debug, thiserror::Error)]
pub enum CrabError {
    FetchError(#[from] reqwest::Error),
}

impl Display for CrabError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "CRAB ERRORRRRR")
    }
}
