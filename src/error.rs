use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ProviderError {}

impl fmt::Display for ProviderError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

pub const ERROR_CLIENT_INIT: u8 = 1u8;

#[derive(Debug)]
pub enum InnerMerkleError {
    ClientError(ProviderError),
}

impl fmt::Display for InnerMerkleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ClientError(e) => write!(f, "ClientError: {}", e),
        }
    }
}

impl Error for InnerMerkleError {}

#[derive(Debug)]
pub struct MerkleError {
    code: u8,
    msg: String,
    inner: Option<InnerMerkleError>,
}

impl MerkleError {
    pub fn new(code: u8, msg: String, inner: Option<InnerMerkleError>) -> Self {
        Self { code, msg, inner }
    }
}

impl fmt::Display for MerkleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.inner {
            Some(e) => write!(f, "E{}: {} due to {}", self.code, self.msg, e),
            None => write!(f, "E{}: {}", self.code, self.msg),
        }
    }
}

impl Error for MerkleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.inner.as_ref().map(|x| x as &dyn Error)
    }
}

impl From<ProviderError> for MerkleError {
    fn from(value: ProviderError) -> Self {
        Self::new(
            ERROR_CLIENT_INIT,
            "Failed to initialise client".to_string(),
            Some(InnerMerkleError::ClientError(value)),
        )
    }
}
